use conrod_core;
use conrod_glium;
use conrod_winit;
use std::path::Iter;
use conrod_core::{Point, Positionable, Colorable, Widget};
use conrod_core::widget::grid::Lines;
use conrod_core::position::Position::Absolute;
use conrod_core::Color;
use datamodel::lattice::Lattice;
use datamodel::Link;
use conrod_core::widget;
use conrod_core::widget::Id;
use datamodel::Direction as Compass;
use conrod_core::text::line::width;


// For conrod
pub const WIN_W: u32 = 1000;
pub const WIN_H: u32 = 1000;
pub const LINK_MINOR: u32 = 20;
pub const LINK_MAJOR: u32 = 40;

/// A demonstration of some application state we want to control with a conrod GUI.
pub struct DemoApp {
    ball_xy: conrod_core::Point,
    ball_color: conrod_core::Color,
    sine_frequency: f32,
}

impl DemoApp {
    /// Simple constructor for the `DemoApp`.
    pub fn new() -> Self {
        DemoApp {
            ball_xy: [0.0, 0.0],
            ball_color: conrod_core::color::WHITE,
            sine_frequency: 1.0,
        }
    }
}

pub fn draw_triangle(tip: Point, point_direction: Compass, id1: Id, id2: Id, id3: Id, ui: &mut conrod_core::UiCell) {
    let long_side: f64 = LINK_MAJOR as f64/ 3.0;
    let short_side: f64 = LINK_MINOR as f64/ 2.0;

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

    widget::Line::new(tip,end_1).x_position(Absolute(tip[0])).y_position(Absolute(tip[1])).set(id1, ui);
    widget::Line::new(tip,end_1a).x_position(Absolute(tip[0])).y_position(Absolute(tip[1])).set(id2, ui);
    widget::Line::new(end_1,end_2).x_position(Absolute(tip[0])).y_position(Absolute(tip[1])).set(id3, ui);

    //widget::Line::centred(tip,end_1). //.x_position(Absolute(tip[0])).y_position(Absolute(tip[1])).set(id1, ui);
    //widget::Line::
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
        // Shapes.
        //shapes_canvas,
        //rounded_rectangle,
        //shapes_left_col,
        //shapes_right_col,
        //shapes_title,
        line,
        lines[],
        //point_path,
        //rectangle_fill,
        lattice_links[],
        triangle
        //rectangle_outline,
        //trapezoid,
        //oval_fill,
        //oval_outline,
        //circle,
        // Button, XyPad, Toggle.
        //button_title,
        //button,
    }
}


pub fn gui(ui: &mut conrod_core::UiCell,
           ids: &mut Ids,
           app: &mut DemoApp,
           lattice_dim: i64,
           lattice: &Lattice) {
    use conrod_core::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    use std::iter::once;

    const MARGIN: conrod_core::Scalar = 30.0;
    const TITLE_SIZE: conrod_core::FontSize = 42;
    const SUBTITLE_SIZE: conrod_core::FontSize = 32;

    const TITLE: &'static str = "Stringnet";
    widget::Canvas::new()
        .pad(MARGIN)
        .set(ids.canvas, ui);

    widget::Text::new(TITLE)
        .font_size(TITLE_SIZE)
        .mid_top_of(ids.canvas)
        .set(ids.title, ui);

    //let start = [-(WIN_W as f64/2.0), -(WIN_H as f64/2.0)];
    let start = [0.0, 0.0];
    let end= [WIN_W as f64/2.0 , WIN_H as f64/2.0];
    widget::Line::centred(start, end)
        .mid_left_of(ids.canvas)
        .set(ids.line, ui);

    ids.lines.resize(
        (6 * lattice_dim * lattice_dim) as usize, &mut ui.widget_id_generator()
    );
    let mut triangle_line_iter = ids.lines.iter();


    // For now just do the horizontal links. Add factor of 2 when you do all of them
    ids.lattice_links.resize(
        (2 * lattice_dim * lattice_dim) as usize, &mut ui.widget_id_generator()
    );

    let mut lattice_link_id_iter = ids.lattice_links.iter();
    let in_color = conrod_core::color::rgb(0.7, 0.0, 0.3);
    let out_color = conrod_core::color::rgb(3.0, 0.0, 0.7);

    let initial_offset = -200.0;
    for (i, cur_vertex) in lattice.vertices.iter().enumerate() {
        let x = cur_vertex.xy.x;
        let y = cur_vertex.xy.y;

        let tri_x = initial_offset + (x as u32 * LINK_MAJOR) as f64;
        let tri_y = initial_offset + (y as u32 * LINK_MAJOR) as f64;

        let &next_id = match lattice_link_id_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID.")
        };
        match cur_vertex.n {
            Link::In => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, in_color, true, 1.0)

            },
            Link::Out => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, out_color, true, 1.0)
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
                draw_triangle([tri_x, tri_y], Compass::W, id1, id2, id3, ui);
            },
            Link::Out => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, out_color, false, 1.0)
            },
            Link::Blank => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, theme().shape_color, false, 1.0)
            }
        }
        let &next_id = match lattice_link_id_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID.")
        };
        match cur_vertex.s {
            Link::In => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, in_color, true, -1.0)
            },
            Link::Out => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, out_color, true, -1.0)
            },
            Link::Blank => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, theme().shape_color, true, -1.0)
            }
        }
        let &next_id = match lattice_link_id_iter.next() {
            Some(id) => id,
            None => panic!("Need a widget ID (2).")
        };
        match cur_vertex.w {
            Link::In => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, in_color, false, -1.0)
            },
            Link::Out => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, out_color, false, -1.0)
            },
            Link::Blank => {
                add_in_lattice_link(initial_offset, x, y, next_id, ui, theme().shape_color, false, -1.0)
            }
        }
    }

//    let mut count = 0;
//    let initial_offset = -200.0;
//    for &id in ids.lattice_links.iter() {
//        widget::RoundedRectangle::fill([
//            LINK_MAJOR as f64, LINK_MINOR as f64],
//      2.0
//        ).x_position(Absolute(
//            initial_offset + ((count % lattice_dim) as f64) * (LINK_MAJOR as f64)
//        ))
//        .y_position(Absolute(
//            initial_offset + (count as f64 / lattice_dim as f64).floor() * (LINK_MAJOR as f64)
//        )).set(id, ui);
//
//        widget::RoundedRectangle::fill(
//                [LINK_MINOR as f64, LINK_MAJOR as f64],
//                2.0
//        ).x_position(Absolute(
//                initial_offset + ((count % lattice_dim) as f64) * (LINK_MAJOR as f64)
//        ))
//        .y_position(Absolute(
//            initial_offset + (count as f64 / lattice_dim as f64).floor() * (LINK_MAJOR as f64)
//        )).set(id, ui);
//        count += 1;
//    }
    //widget::RoundedRectangle::fill([100.0, 200.0], 10.0).x_position(Absolute(-100.0))
    //    .y_position(Absolute(-100.0)).set(ids.rectangle_fill, ui);

}

pub struct GliumDisplayWinitWrapper(pub glium::Display);

impl conrod_winit::WinitWindow for GliumDisplayWinitWrapper {
    fn get_inner_size(&self) -> Option<(u32, u32)> {
        self.0.gl_window().get_inner_size().map(Into::into)
    }
    fn hidpi_factor(&self) -> f32 {
        self.0.gl_window().get_hidpi_factor() as _
    }
}

/// In most of the examples the `glutin` crate is used for providing the window context and
/// events while the `glium` crate is used for displaying `conrod_core::render::Primitives` to the
/// screen.
///
/// This `Iterator`-like type simplifies some of the boilerplate involved in setting up a
/// glutin+glium event loop that works efficiently with conrod.
pub struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
        }
    }

    /// Produce an iterator yielding all available events.
    pub fn next(&mut self, events_loop: &mut glium::glutin::EventsLoop) -> Vec<glium::glutin::Event> {
        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));

        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !self.ui_needs_update {
            events_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            });
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    /// Notifies the event loop that the `Ui` requires another update whether or not there are any
    /// pending events.
    ///
    /// This is primarily used on the occasion that some part of the `Ui` is still animating and
    /// requires further updates to do so.
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }
}

// Conversion functions for converting between types from glium's version of `winit` and
// `conrod_core`.
conrod_winit::conversion_fns!();
