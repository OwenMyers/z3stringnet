use conrod_core;
use conrod_glium;
use conrod_winit;
use std::path::Iter;
use conrod_core::Point;
use conrod_core::widget::grid::Lines;


// For conrod
pub const WIN_W: u32 = 1000;
pub const WIN_H: u32 = 1000;

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
        grid,
        // Shapes.
        //shapes_canvas,
        //rounded_rectangle,
        //shapes_left_col,
        //shapes_right_col,
        //shapes_title,
        line,
        point_path,
        //rectangle_fill,
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

struct LatticeShapeIter {
    lattice_size: u32,
    count: u32
}

impl Iterator for LatticeShapeIter {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        self.count += 1;
        if self.count > 5 {
            None
        }
        else {
            //let cur_point = Point::new(count, 0.0);
            //let cur_point = Point{x: self.count * WIN_H / 5, y: 0.0};
            let cur_point = [((self.count % 2) * 100) as f64, (self.count * 100) as f64];
            Some(cur_point)
        }
    }
}

struct Olines {
    x: u32
}

impl Iterator for Olines {
    type Item = Lines<u32>;

    fn next(&mut self) -> Option<Lines<u32>> {
        Some(
            Lines {
                /// The distance that separates each line.
                step: self.x,
                /// An optional offset for the lines along they're axis.
                offset: None,
                /// The thickness of each of the lines drawn.
                ///
                /// If `None`, the `thickness` specified within the `Style` is used.
                thickness: None,
                /// The color of each of the lines drawn.
                ///
                /// If `None`, the `color` specified within the `Style` is used.
                color: None,
            }
        )
    }
}


pub fn gui(ui: &mut conrod_core::UiCell,
           ids: &Ids,
           app: &mut DemoApp,
           lattice_dim: i64) {
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

    //for i in 0..lattice_dim {
    //}

    let lattice_shape_iter = LatticeShapeIter{lattice_size: 4, count: 0};
    widget::PointPath::new(lattice_shape_iter).set(ids.point_path, ui);

    let cur_olines = Olines{x: 100};
    widget::Grid::new( -400, 400, -400, 400,
        widget::grid::Axis::X(cur_olines.next())
    ).set(ids.grid, ui);
//    let grid = widget::Grid {
//        /// The minimum visible bound along the *x* axis.
//        min_x: X,
//        /// The maximum visible bound along the *x* axis.
//        max_x: X,
//        /// The minimum visible bound along the *y* axis.
//        min_y: Y,
//        /// The maximum visible bound along the *y* axis.
//        max_y: Y,
//        /// An offset for all vertical lines distributed across the *x* axis.
//        x_offset: None,
//        /// An offset for all horizontal lines distributed across the *y* axis.
//        y_offset: None,
//        /// An iterator yielding each sequence of lines to be distributed across the grid.
//        lines: [
//
//        ],
//    }

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
