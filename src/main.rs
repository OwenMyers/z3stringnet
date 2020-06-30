//use std::error::Error;
#[macro_use]
extern crate clap;
extern crate z3stringnet;
#[macro_use]
extern crate conrod_core;
extern crate glium;
extern crate conrod_winit;
extern crate conrod_glium;
use conrod_glium::Renderer;
use clap::App;
use z3stringnet::datamodel::Point;
use z3stringnet::datamodel::BoundPoint;
use z3stringnet::datamodel::lattice::Lattice;
use z3stringnet::datamodel::lattice::build_z3_striped_lat;
use z3stringnet::datamodel::lattice::build_blank_lat;
use z3stringnet::lattice_updates::Update;
use z3stringnet::lattice_updates::UpdateType;
use z3stringnet::estimators::density_estimator::DensityEstimator;
use z3stringnet::estimators::correlation_origin_estimator::CorrelationOriginEstimator;
use z3stringnet::estimators::total_link_count_estimator::TotalLinkCountEstimator;
use z3stringnet::estimators::winding_number_estimator::WindingNumberCountEstimator;
use z3stringnet::estimators::winding_variance_estimator::WindingNumberVarianceEstimator;
use z3stringnet::estimators::Measurable;
use z3stringnet::oio::*;

// For conrod
pub const WIN_W: u32 = 1000;
pub const WIN_H: u32 = 1000;

/// A demonstration of some application state we want to control with a conrod GUI.
pub struct DemoApp {
    ball_xy: conrod_core::Point,
    ball_color: conrod_core::Color,
    sine_frequency: f32,
    rust_logo: conrod_core::image::Id,
}

impl DemoApp {
    /// Simple constructor for the `DemoApp`.
    pub fn new(rust_logo: conrod_core::image::Id) -> Self {
        DemoApp {
            ball_xy: [0.0, 0.0],
            ball_color: conrod_core::color::WHITE,
            sine_frequency: 1.0,
            rust_logo: rust_logo,
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
        // Shapes.
        //shapes_canvas,
        //rounded_rectangle,
        //shapes_left_col,
        //shapes_right_col,
        //shapes_title,
        line,
        //point_path,
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


pub fn gui(ui: &mut conrod_core::UiCell, ids: &Ids, app: &mut DemoApp) {
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

    let start = [-40.0, -40.0];
    let end = [40.0, 40.0];
    widget::Line::centred(start, end)
        .mid_left_of(ids.canvas)
        .set(ids.line, ui);

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



fn main() {
    // Parse arguments
    let yaml = load_yaml!("cli.yml");

    // Conrod Start
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Conrod with glium!")
        .with_dimensions((WIN_W, WIN_H).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let display = GliumDisplayWinitWrapper(display);

    // Construct our `Ui`.
    let mut ui = conrod_core::UiBuilder::new([WIN_W as f64, WIN_H as f64])
        .theme(theme())
        .build();

    // The `widget::Id` of each widget instantiated in `conrod_example_shared::gui`.
    let ids = Ids::new(ui.widget_id_generator());
    // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    //
    // Internally, the `Renderer` maintains:
    // - a `backend::glium::GlyphCache` for caching text onto a `glium::texture::Texture2d`.
    // - a `glium::Program` to use as the shader program when drawing to the `glium::Surface`.
    // - a `Vec` for collecting `backend::glium::Vertex`s generated when translating the
    // `conrod_core::render::Primitive`s.
    // - a `Vec` of commands that describe how to draw the vertices.
    let mut renderer = Renderer::new(&display.0).unwrap();
    // Start the loop:
    //
    // - Poll the window for available events.
    // - Update the widgets via the `conrod_example_shared::gui` fn.
    // - Render the current state of the `Ui`.
    // - Repeat.
    let mut event_loop = EventLoop::new();
    'main: loop {
        // Handle all events.
        for event in event_loop.next(&mut events_loop) {
        }
    }
    // Conrod End
    let matches = App::from_yaml(yaml).get_matches();

    let lattice_size_arg_str = matches.value_of("size").unwrap_or("4");
    let lattice_size_arg: i64 = lattice_size_arg_str.parse().unwrap();
    println!("Lattice size from argument: {}", lattice_size_arg);

    let weights_arg_str = matches.value_of("weights").unwrap_or("1.0");
    let weights_arg: f64 = weights_arg_str.parse().unwrap();
    println!("Weight parameter from argument: {}", weights_arg);

    let n_updates_arg_str = matches.value_of("nupdate").unwrap_or("5");
    let n_updates_arg: u64 = n_updates_arg_str.parse().unwrap();
    println!("Number of updates from argument: {}", n_updates_arg);

    let n_measure_arg_str = matches.value_of("nmeasure").unwrap_or("500");
    let n_measure_arg: u64 = n_measure_arg_str.parse().unwrap();
    println!("Number of measurements to make per bin: {}", n_measure_arg);

    let n_bins_arg_str = matches.value_of("nbins").unwrap_or("10");
    let n_bins_arg: u64 = n_bins_arg_str.parse().unwrap();
    println!("Number of bins: {}", n_bins_arg);

    let write_update_configurations_str = matches.value_of("write-update-confs").unwrap_or("false");
    let write_update_configurations: bool = write_update_configurations_str.parse().unwrap();
    println!("Write update configs: {}", write_update_configurations);

    let write_measure_configurations_str = matches.value_of("write-measure-confs").unwrap_or("false");
    let write_measure_configurations: bool = write_measure_configurations_str.parse().unwrap();
    println!("Write measure configs: {}", write_measure_configurations);

    let write_bin_configurations_str = matches.value_of("write-bin-confs").unwrap_or("false");
    let write_bin_configurations: bool = write_bin_configurations_str.parse().unwrap();
    println!("Write bin configs: {}", write_bin_configurations);

    let update_type: &UpdateType = &UpdateType::Local;
    if matches.is_present("loop-update") {
        let update_type: &UpdateType = &UpdateType::Walk;
        println!("Lattice will be updated using random walk.");
    } else {
        println!("Lattice will be updated using plaquette flips.");
    }

    let equilibrate = true;

    let size: Point = Point {
        x: lattice_size_arg,
        y: lattice_size_arg,
    };

    let mut lat: Lattice;
    // lat now owns size -> That is good and intentional
    lat = build_blank_lat(size);
    //lat = build_z3_striped_lat(size);

    // number_bins: The number of lines in the data file (10000)
    let number_bins: u64 = n_bins_arg;
    // number_measure: How many measurements to average over per bin (500)
    let number_measure: u64 = n_measure_arg;
    // number_update: How many updated before a measurement (5)
    let number_update: u64 = n_updates_arg;
    // for local updates it should be
    //let number_update: u64 = 2 * lat.size.x * lat.size.y;

    // Initialize the object to update the lattice
    let mut updater = Update{
        working_loc: BoundPoint{
            size: lat.size,
            location: Point{x: 0, y: 0},
        },
        link_number_tuning: weights_arg,
        link_number_change: 0,
    };

    // Initialize the object to measure the string density,
    let mut density_estimator = DensityEstimator::new(&lat.size);
    let mut correlation_origin_estimator = CorrelationOriginEstimator::new(&lat.size);
    let mut total_link_count_estimator = TotalLinkCountEstimator::new();
    let mut winding_count_estimator = WindingNumberCountEstimator::new();
    let mut winding_variance_estimator = WindingNumberVarianceEstimator::new();

    // Equilibrate
    if equilibrate {
        println!("Equilibrating");
        let equilibration_time = lat.size.x * lat.size.y;
        //let equilibration_time = 1;

        println!("Number of updates in equilibration: {}", equilibration_time);
        for _ in 0..equilibration_time {
            updater.main_update(&mut lat, &update_type);
        }
        println!("Done equilibrating");
    }

    // Actual run
    let mut total_update_count: u64 = 0;
    for _i in 0..number_bins {
        println!("Working on bin {}", _i);
        if write_bin_configurations {
            write_lattice(String::from(format!("lattice_bin_{}.csv", total_update_count)), &lat);
        }
        for _j in 0..number_measure {
            //println!("j {}", _j);
            if write_measure_configurations {
                write_lattice(String::from(format!("lattice_measure_{}.csv", total_update_count)), &lat);
            }
            for _k in 0..number_update {
                //println!("k {}", _k);
                if write_update_configurations {
                    write_lattice(String::from(format!("lattice_{}.csv", total_update_count)), &lat);
                }
                updater.main_update(&mut lat, &update_type);
                total_update_count += 1;
            }
            density_estimator.measure(&lat);
            correlation_origin_estimator.measure(&lat);
            total_link_count_estimator.measure(&lat);
            winding_variance_estimator.measure(&lat);
        }

        density_estimator.finalize_bin_and_write(number_measure);
        correlation_origin_estimator.finalize_bin_and_write(number_measure);
        total_link_count_estimator.finalize_bin_and_write(number_measure);
        winding_variance_estimator.finalize_bin_and_write(number_measure);

        density_estimator.clear();
        correlation_origin_estimator.clear();
        total_link_count_estimator.clear();
        winding_variance_estimator.clear();

        winding_count_estimator.measure(&lat);
        winding_count_estimator.finalize_bin_and_write(1);
    }
    winding_count_estimator.clear();
}
