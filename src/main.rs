//use std::error::Error;
#[macro_use]
extern crate clap;
extern crate z3stringnet;
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


fn main() {
    // Parse arguments
    let yaml = load_yaml!("cli.yml");
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
