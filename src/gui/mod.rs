use conrod_core;
use conrod_glium;
use conrod_winit;
use std::path::Iter;
use conrod_core::{Point, Positionable, Colorable, Widget, Borderable};
use conrod_core::widget::grid::Lines;
use conrod_core::position::Position::Absolute;
use conrod_core::Color;
use datamodel::lattice::Lattice;
use datamodel::Link;
use conrod_core::widget;
use conrod_core::widget::Id;
use datamodel;
use datamodel::Direction as Compass;
use conrod_core::text::line::width;
use estimators::winding_number_estimator::{WindingNumberCountEstimator, WindingNumberCountEstimatorDisplay};


// For conrod
pub const WIN_W: u32 = 1100;
pub const WIN_H: u32 = 1100;
pub const LINK_MINOR: u32 = 20;
pub const LINK_MAJOR: u32 = 40;

use glium::{
    glutin::{event, event_loop},
    Display,
};
use conrod_core::color::TRANSPARENT;

/// A demonstration of some application state we want to control with a conrod GUI.
pub struct DemoApp {
    ball_xy: conrod_core::Point,
    ball_color: conrod_core::Color,
    sine_frequency: f32,
    winding_number_display: WindingNumberCountEstimatorDisplay,
}

impl DemoApp {
    /// Simple constructor for the `DemoApp`.
    pub fn new() -> Self {
        DemoApp {
            ball_xy: [0.0, 0.0],
            ball_color: conrod_core::color::WHITE,
            sine_frequency: 1.0,
            winding_number_display: WindingNumberCountEstimatorDisplay {
                local_text: String::from("Not Started"),
                position: datamodel::Point { x: -1, y: -1 },
            },
        }
    }
}

pub fn draw_winding_number_display(
    wnd: & WindingNumberCountEstimatorDisplay,
    ids: &mut Ids,
    ui: &mut conrod_core::UiCell,
    initial_offset: f64,
) {
    let mut x_pos_mod = 0.0;
    let mut y_pos_mod = LINK_MAJOR as f64 / 2.0 * 1.0;

    const size: conrod_core::FontSize = 12;
    let in_color = conrod_core::color::rgb(0.7, 0.0, 0.3);
    //const INTRODUCTION: &'static str = "Testing Text\nSome more";
    widget::TextBox::new(&wnd.local_text).border(0.0)
        .font_size(size).x_position(Absolute(150.0)).y_position(Absolute(150.0))
        .text_color(in_color).color(TRANSPARENT)
        .set(ids.winding_text_box, ui);

    if wnd.position.y >= 0 {
        let dimensions = [(LINK_MINOR as f64)/2.0, LINK_MAJOR as f64];
        widget::Rectangle::fill(dimensions)
            .x_position(Absolute(initial_offset + ((wnd.position.x as f64) * 2.0 + 1.0) * (LINK_MINOR as f64)))
            .y_position(Absolute(initial_offset + (wnd.position.y as f64) * ( LINK_MAJOR as f64)))
            .color(in_color).set(ids.winding_link, ui);
    }
}

pub fn draw_triangle(tip: Point, point_direction: Compass, id1: Id, id2: Id, id3: Id, ui: &mut conrod_core::UiCell, quadrent: bool) {
    let long_side: f64 = LINK_MAJOR as f64/ 3.0;
    let short_side: f64 = LINK_MINOR as f64/ 2.0;
    let mut tip = tip;
    if quadrent == false {
        tip = match point_direction
        {
            Compass::N => [tip[0], tip[1] + 3.0 * short_side],
            Compass::E => [tip[0] + 3.0 * short_side, tip[1]],
            Compass::S => [tip[0], tip[1] + 1.0 * short_side],
            Compass::W => [tip[0] + 1.0 * short_side, tip[1]]
        };
    } else {
        tip = match point_direction
        {
            Compass::N => [tip[0], tip[1] - 1.0 * short_side],
            Compass::E => [tip[0] - 1.0 * short_side, tip[1]],
            Compass::S => [tip[0], tip[1] - 3.0 * short_side],
            Compass::W => [tip[0] - 3.0 * short_side, tip[1]]
        };
    }
    //} else {
    //    let tip = match point_direction
    //    {
    //        Compass::N => [tip[0], tip[1] - 3.0 * short_side],
    //        Compass::E => [tip[0] - 3.0 * short_side, tip[1]],
    //        Compass::S => [tip[0], tip[1] + 3.0 * short_side],
    //        Compass::W => [tip[0] + 3.0 * short_side, tip[1]]
    //    };
    //};

    let end_1 = match point_direction {
        Compass::N => [tip[0] - short_side, tip[1] - long_side],
        Compass::E => [tip[0] - long_side, tip[1] - short_side],
        Compass::S => [tip[0] + short_side, tip[1] + long_side],
        Compass::W => [tip[0] + long_side, tip[1] + short_side],
    };
    let end_2 = match point_direction {
        Compass::N => [end_1[0] + 2.0 * short_side, end_1[1]],
        Compass::E => [end_1[0], end_1[1] + 2.0 * short_side],
        Compass::S => [end_1[0] - 2.0 * short_side, end_1[1]],
        Compass::W => [end_1[0], end_1[1] - 2.0 * short_side],
    };
    let end_1a = match point_direction {
        Compass::N => [tip[0] + short_side, tip[1] - long_side],
        Compass::E => [tip[0] - long_side, tip[1] + short_side],
        Compass::S => [tip[0] - short_side, tip[1] + long_side],
        Compass::W => [tip[0] + long_side, tip[1] - short_side],
    };

    widget::Line::abs(tip,end_1).set(id1, ui);
    widget::Line::abs(tip,end_1a).set(id2, ui);
    widget::Line::abs(end_1,end_2).set(id3, ui);
}


fn add_in_lattice_link(initial_offset: f64,
                       x: i64,
                       y: i64,
                       next_id: conrod_core::widget::Id,
                       ui: &mut conrod_core::UiCell,
                       color: Color,
                       vertical: bool,
                       shift_direction: f64) -> () {
    let mut link_x = LINK_MINOR;
    let mut link_y = LINK_MAJOR;
    let mut x_pos_mod = 0.0;
    let mut y_pos_mod = LINK_MAJOR as f64 / 2.0 * shift_direction;

    if !vertical {
        link_x = LINK_MAJOR;
        link_y = LINK_MINOR;
        x_pos_mod = LINK_MAJOR as f64 / 2.0 * shift_direction;
        y_pos_mod = 0.0;
    }
    let dimensions = [link_x as f64, link_y as f64];
    widget::RoundedRectangle::fill(dimensions, 8.0).x_position(
        Absolute(initial_offset + (x as f64) * (LINK_MAJOR as f64) + x_pos_mod)
    ).y_position(
        Absolute(initial_offset + (y as f64) * (LINK_MAJOR as f64) + y_pos_mod)
    ).color(color).set(next_id, ui)
}

/// A set of reasonable stylistic defaults that works for the `gui` below.
pub fn theme() -> conrod_core::Theme {
    use conrod_core::position::{Align, Direction, Padding, Position, Relative};
    conrod_core::Theme {
        name: "Stringnet".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod_core::color::DARK_CHARCOAL,
        shape_color: conrod_core::color::LIGHT_CHARCOAL,
        border_color: conrod_core::color::BLACK,
        border_width: 0.0,
        label_color: conrod_core::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod_core::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}


// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        // The scrollable canvas.
        canvas,
        // The title and introduction widgets.
        title,
        introduction,
        winding_text_box,
        winding_link,
        line,
        lines[],
        lattice_links[],
        triangle,
        button_title,
        button
    }
}

pub fn gui(ui: &mut conrod_core::UiCell,
           ids: &mut Ids,
           app: &mut DemoApp,
           lattice_dim: i64,
           lattice: &Lattice,
           winding_estimator: &mut WindingNumberCountEstimator) {
    use conrod_core::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    use std::iter::once;

    const MARGIN: conrod_core::Scalar = 30.0;
    const TITLE_SIZE: conrod_core::FontSize = 42;
    const SUBTITLE_SIZE: conrod_core::FontSize = 42;

    const TITLE: &'static str = "Stringnet";
    widget::Canvas::new()
        .pad(MARGIN)
        .set(ids.canvas, ui);

    widget::Text::new(TITLE)
        .font_size(TITLE_SIZE)
        .mid_top_of(ids.canvas)
        .set(ids.title, ui);

    let initial_offset = -100.0;
    ids.lines.resize(
        (12 * lattice_dim * lattice_dim) as usize, &mut ui.widget_id_generator()
    );
    let mut triangle_line_iter = ids.lines.iter();

    // For now just do the horizontal links. Add factor of 2 when you do all of them
    ids.lattice_links.resize(
        (2 * lattice_dim * lattice_dim) as usize, &mut ui.widget_id_generator()
    );

    let mut lattice_link_id_iter = ids.lattice_links.iter();
    let in_color = conrod_core::color::rgb(0.7, 0.0, 0.3);
    let out_color = conrod_core::color::rgb(3.0, 0.0, 0.7);

    for (i, cur_vertex) in lattice.vertices.iter().enumerate() {
        let x = cur_vertex.xy.x.clone();
        let y = cur_vertex.xy.y.clone();

        let tri_x = initial_offset + (x as u32 * LINK_MAJOR) as f64;
        let tri_y = initial_offset + (y as u32 * LINK_MAJOR) as f64;

        let &next_id = match lattice_link_id_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID.")
        };
        let &id1 =  match triangle_line_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID.")
        };
        let &id2 =  match triangle_line_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID.")
        };
        let &id3 =  match triangle_line_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID.")
        };
        match cur_vertex.n {
            Link::In => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, in_color, true, 1.0);
                draw_triangle([tri_x, tri_y], Compass::N, id1, id2, id3, ui, true);
            },
            Link::Out => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, out_color, true, 1.0);
                draw_triangle([tri_x, tri_y], Compass::S, id1, id2, id3, ui, true);
            },
            Link::Blank => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, theme().shape_color, true, 1.0)
            }
        }
        let &next_id = match lattice_link_id_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID (2).")
        };
        let &id1 =  match triangle_line_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID.")
        };
        let &id2 =  match triangle_line_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID.")
        };
        let &id3 =  match triangle_line_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID.")
        };
        match cur_vertex.e {
            Link::In => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, in_color, false, 1.0);
                draw_triangle([tri_x, tri_y], Compass::W, id1, id2, id3, ui, false);
            },
            Link::Out => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, out_color, false, 1.0);
                draw_triangle([tri_x, tri_y], Compass::E, id1, id2, id3, ui, false);
            },
            Link::Blank => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, theme().shape_color, false, 1.0)
            }
        }
        let &next_id = match lattice_link_id_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID.")
        };
        let &id1 =  match triangle_line_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID.")
        };
        let &id2 =  match triangle_line_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID.")
        };
        let &id3 =  match triangle_line_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID.")
        };
        match cur_vertex.s {
            Link::In => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, in_color, true, -1.0);
                draw_triangle([tri_x, tri_y], Compass::S, id1, id2, id3, ui, false);
            },
            Link::Out => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, out_color, true, -1.0);
                draw_triangle([tri_x, tri_y], Compass::N, id1, id2, id3, ui, false);
            },
            Link::Blank => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, theme().shape_color, true, -1.0);
            }
        }
        let &next_id = match lattice_link_id_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID (2).")
        };
        let &id1 =  match triangle_line_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID.")
        };
        let &id2 =  match triangle_line_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID.")
        };
        let &id3 =  match triangle_line_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID.")
        };
        match cur_vertex.w {
            Link::In => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, in_color, false, -1.0);
                draw_triangle([tri_x, tri_y], Compass::E, id1, id2, id3, ui, true);
            },
            Link::Out => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, out_color, false, -1.0);
                draw_triangle([tri_x, tri_y], Compass::W, id1, id2, id3, ui, true);
            },
            Link::Blank => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, theme().shape_color, false, -1.0)
            }
        }

    }
    for _press in widget::Button::new()
        .label("Winding Number")
        .x_position(Absolute(-25.0)).y_position(Absolute(150.0))
        //.top_left()
        //.top_left_with_margin_on(ids.canvas, MARGIN)
        //.down_from(ids.button_title, 60.0)
        .w_h(160.0, 40.0)
        .set(ids.button, ui) {
        app.winding_number_display = match winding_estimator.next() {
            Some(w) => w,
            None => //println!("Got no winding number display")
                WindingNumberCountEstimatorDisplay {
                    local_text: String::from("Started Something "),
                    position: datamodel::Point { x: -1, y: -1 },
                }
        }
    };
    //println!("{:?}", winding_estimator_display);
    match Some(&app.winding_number_display){
        Some(wind_disp) => draw_winding_number_display(
            wind_disp, ids, ui, initial_offset
        ),
        None => println!("Got no winding number display")
    };
}


pub enum Request<'a, 'b: 'a> {
    Event {
        event: &'a event::Event<'b, ()>,
        should_update_ui: &'a mut bool,
        should_exit: &'a mut bool,
    },
    SetUi {
        needs_redraw: &'a mut bool,
    },
    Redraw,
}

/// In most of the examples the `glutin` crate is used for providing the window context and
/// events while the `glium` crate is used for displaying `conrod_core::render::Primitives` to the
/// screen.
///
/// This function simplifies some of the boilerplate involved in limiting the redraw rate in the
/// glutin+glium event loop.
pub fn run_loop<F>(display: Display, event_loop: event_loop::EventLoop<()>, mut callback: F) -> !
    where
        F: 'static + FnMut(Request, &Display),
{
    let sixteen_ms = std::time::Duration::from_millis(16);
    let mut next_update = None;
    let mut ui_update_needed = false;
    event_loop.run(move |event, _, control_flow| {
        {
            let mut should_update_ui = false;
            let mut should_exit = false;
            callback(
                Request::Event {
                    event: &event,
                    should_update_ui: &mut should_update_ui,
                    should_exit: &mut should_exit,
                },
                &display,
            );
            ui_update_needed |= should_update_ui;
            if should_exit {
                *control_flow = event_loop::ControlFlow::Exit;
                return;
            }
        }

        // We don't want to draw any faster than 60 FPS, so set the UI only on every 16ms, unless:
        // - this is the very first event, or
        // - we didn't request update on the last event and new events have arrived since then.
        let should_set_ui_on_main_events_cleared = next_update.is_none() && ui_update_needed;
        match (&event, should_set_ui_on_main_events_cleared) {
            (event::Event::NewEvents(event::StartCause::Init { .. }), _)
            | (event::Event::NewEvents(event::StartCause::ResumeTimeReached { .. }), _)
            | (event::Event::MainEventsCleared, true) => {
                next_update = Some(std::time::Instant::now() + sixteen_ms);
                ui_update_needed = false;

                let mut needs_redraw = false;
                callback(
                    Request::SetUi {
                        needs_redraw: &mut needs_redraw,
                    },
                    &display,
                );
                if needs_redraw {
                    display.gl_window().window().request_redraw();
                } else {
                    // We don't need to redraw anymore until more events arrives.
                    next_update = None;
                }
            }
            _ => {}
        }
        if let Some(next_update) = next_update {
            *control_flow = event_loop::ControlFlow::WaitUntil(next_update);
        } else {
            *control_flow = event_loop::ControlFlow::Wait;
        }

        // Request redraw if needed.
        match &event {
            event::Event::RedrawRequested(_) => {
                callback(Request::Redraw, &display);
            }
            _ => {}
        }
    })
}

// Conversion functions for converting between types from glium's version of `winit` and
// `conrod_core`.
conrod_winit::v023_conversion_fns!();