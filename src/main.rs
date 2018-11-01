//use std::error::Error;
extern crate z3stringnet;
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
use z3stringnet::estimators::Measurable;
use z3stringnet::oio::*;


fn main() {

    let equilibrate = true;
    let write_configurations = false;
    let update_type: &UpdateType = &UpdateType::Walk;

    let size: Point = Point {
        x: 4,
        y: 4,
    };

    let mut lat: Lattice;
    // lat now owns size -> That is good and intentional
    lat = build_blank_lat(size);
    //lat = build_z3_striped_lat(size);

    // number_bins: The number of lines in the data file (10000)
    let number_bins: u64 = 20000;
    // number_measure: How many measurements to average over per bin (500)
    let number_measure: u64 = 500;
    // number_update: How many updated before a measurement (5)
    let number_update: u64 = 5;
    // for local updates it should be
    //let number_update: u64 = 2 * lat.size.x * lat.size.y;

    // Initialize the object to update the lattice
    let mut updater = Update{
        working_loc: BoundPoint{
            size: lat.size,
            location: Point{x: 0, y: 0},
        },
        link_number_tuning: 0.2,
        link_number_change: 0,
    };

    // Initialize the object to measure the string density,
    let mut density_estimator = DensityEstimator::new(&lat.size);
    let mut correlation_origin_estimator = CorrelationOriginEstimator::new(&lat.size);
    let mut total_link_count_estimator = TotalLinkCountEstimator::new();

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
        for _j in 0..number_measure {
            //println!("j {}", _j);
            for _k in 0..number_update {
                //println!("k {}", _k);
                if write_configurations {
                    write_lattice(
                        String::from(format!("lattice_{}.csv", total_update_count)), &lat
                    );
                }
                updater.main_update(&mut lat, &update_type);
                total_update_count += 1;
            }
            density_estimator.measure(&lat);
            correlation_origin_estimator.measure(&lat);
            total_link_count_estimator.measure(&lat);
        }
        density_estimator.finalize_bin_and_write(number_measure);
        correlation_origin_estimator.finalize_bin_and_write(number_measure);
        total_link_count_estimator.finalize_bin_and_write(number_measure);
        density_estimator.clear();
        correlation_origin_estimator.clear();
        total_link_count_estimator.clear();
    }   
} 
