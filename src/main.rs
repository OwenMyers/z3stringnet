//use std::error::Error;
#[macro_use]
extern crate clap;
extern crate z3stringnet;
extern crate conrod_glium;
extern crate glium;
extern crate conrod_core;


use clap::App;
use conrod_glium::Renderer;
use conrod_core::Dimensions;
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
use z3stringnet::gui::*;
use glium::Surface;
use conrod_core::widget::Image;


fn main() {
    // Parse arguments
    let yaml = load_yaml!("cli.yml");

    let matches = App::from_yaml(yaml).get_matches();

    let lattice_size_arg_str = matches.value_of("size").unwrap_or("4");
    let lattice_size_arg: i64 = lattice_size_arg_str.parse().unwrap();
    println!("Lattice size from argument: {}", lattice_size_arg);

    // Conrod Start
    let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    let window = glium::glutin::window::WindowBuilder::new()
        .with_title("Conrod with glium!").with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0));
        //.with_dimensions((WIN_W, WIN_H).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    //let display = GliumDisplayWinitWrapper(display);

    // Construct our `Ui`.
    let mut ui = conrod_core::UiBuilder::new([WIN_W as f64, WIN_H as f64])
        .theme(theme())
        .build();

    //let mut image_map = conrod_core::image::Map::new();
    let mut image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();
    //let try_rectangle = Rectangle::fill([WIN_H as f64/10.0, WIN_W as f64/5.0]);

    // The `widget::Id` of each widget instantiated in `conrod_example_shared::gui`.
    let mut ids = Ids::new(ui.widget_id_generator());
    // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    //
    // Internally, the `Renderer` maintains:
    // - a `backend::glium::GlyphCache` for caching text onto a `glium::texture::Texture2d`.
    // - a `glium::Program` to use as the shader program when drawing to the `glium::Surface`.
    // - a `Vec` for collecting `backend::glium::Vertex`s generated when translating the
    // `conrod_core::render::Primitive`s.
    // - a `Vec` of commands that describe how to draw the vertices.
    let mut renderer = Renderer::new(&display).unwrap();

    let size: Point = Point {
        x: lattice_size_arg,
        y: lattice_size_arg,
    };
    let mut lat: Lattice;
    // lat now owns size -> That is good and intentional
    // lat = build_blank_lat(size);
    lat = build_z3_striped_lat(size);

    let equilibrate = true;

    // A demonstration of some app state that we want to control with the conrod GUI.
    let mut app = DemoApp::new();
    // Start the loop:
    //
    // - Send available events to the `Ui`.
    // - Update the widgets via the `conrod_example_shared::gui` fn.
    // - Render the current state of the `Ui`.
    // - Repeat.
    run_loop(display, events_loop, move |request, display| {
        match request {
            Request::Event {
                event,
                should_update_ui,
                should_exit,
            } => {
                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = convert_event(&event, &display.gl_window().window()) {
                    ui.handle_event(event);
                    *should_update_ui = true;
                }

                match event {
                    glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                        // Break from the loop upon `Escape`.
                        glium::glutin::event::WindowEvent::CloseRequested
                        | glium::glutin::event::WindowEvent::KeyboardInput {
                            input:
                            glium::glutin::event::KeyboardInput {
                                virtual_keycode:
                                Some(glium::glutin::event::VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => *should_exit = true,
                        _ => {}
                    },
                    _ => {}
                }
            }
            Request::SetUi { needs_redraw } => {
                gui(&mut ui.set_widgets(), &mut ids, &mut app, lattice_size_arg, &lat);
                // Instantiate a GUI demonstrating every widget type provided by conrod.
                //conrod_example_shared::gui(&mut ui.set_widgets(), &ids, &mut app);

                *needs_redraw = ui.has_changed();
            }
            Request::Redraw => {
                if let Some(primitives) = ui.draw_if_changed() {
                    renderer.fill(display, primitives, &image_map);
                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 0.0, 1.0);
                    renderer.draw(display, &mut target, &image_map).unwrap();
                    target.finish().unwrap();
                }
            }
            //    // Render the `Ui` and then display it on the screen.
            //    let primitives = ui.draw();

            //    renderer.fill(display, primitives, &image_map);
            //    let mut target = display.draw();
            //    target.clear_color(0.0, 0.0, 0.0, 1.0);
            //    renderer.draw(display, &mut target, &image_map).unwrap();
            //    target.finish().unwrap();
            //}
        }
    });

    // Conrod End

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


    // More Conrod
    //'main: loop {
    //    // Handle all events.
    //    for event in event_loop.next(&mut events_loop) {
    //        // Use the `winit` backend feature to convert the winit event to a conrod one.
    //        if let Some(event) = convert_event(event.clone(), &display) {
    //            ui.handle_event(event);
    //            event_loop.needs_update();
    //        }

    //        match event {
    //            glium::glutin::event::Event::WindowEvent { event, .. } => match event {
    //                // Break from the loop upon `Escape`.
    //                glium::glutin::event::WindowEvent::CloseRequested
    //                | glium::glutin::event::WindowEvent::KeyboardInput {
    //                    input:
    //                    glium::glutin::event::KeyboardInput {
    //                        virtual_keycode: Some(glium::glutin::event::VirtualKeyCode::Escape),
    //                        ..
    //                    },
    //                    ..
    //                } => break 'main,
    //                _ => (),
    //            },
    //            _ => (),
    //        }
    //    }
    //    gui(&mut ui.set_widgets(), &mut ids, &mut app, lattice_size_arg, &lat);
    //    // Draw the `Ui`.
    //    if let Some(primitives) = ui.draw_if_changed() {
    //        renderer.fill(&display.0, primitives, &image_map);
    //        let mut target = display.0.draw();
    //        target.clear_color(0.0, 0.0, 0.0, 1.0);
    //        renderer.draw(&display.0, &mut target, &image_map).unwrap();
    //        target.finish().unwrap();
    //    }
    //}
    // End More Coonrod

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
