//use std::error::Error;
extern crate z3stringnet;
use z3stringnet::datamodel::*;
use z3stringnet::datamodel::lattice::*;
use z3stringnet::oio::*;
    

fn main() {

    let equilibrate = true;
    let write_configurations = false;

    let size: Point = Point {
        x: 4,
        y: 4,
    };

    let mut lat: Lattice;
    // lat now owns size -> That is good and intentional
    //lat = build_blank_lat(size);
    lat = build_z3_striped_lat(size);

    // number_bins: The number of lines in the data file
    let number_bins: u64 = 1;
    // number_measure: How many measurements to average over per bin
    let number_measure: u64 = 1;
    // number_update: How many updated before a measurement
    let number_update: u64 = 1;
    // for local updates it should be
    //let number_update: u64 = 2 * lat.size.x * lat.size.y;


    // Initilize the object to update the lattice
    let mut updater = Update{
        working_loc: BoundPoint{
            size: lat.size,
            location: Point{x: 0, y: 0},
        }
    };

    // Initilize the object to measure the string density,
    let mut density_estimator: DensityEstimator = DensityEstimator::new(&lat.size);
    density_estimator.count_in_out(&lat);
    density_estimator.write_total_count(String::from(format!("density_estimator_{}.csv", 0)))

    // Equilibrate
    if equilibrate {
        equilibration_time = lat.size.x * lat.size.y;
        for i in equilibration_time {
            updater.random_walk_update(&mut lat);
        }
    }

    // Actual run
    for i in 0..number_bins {
        for j in 0..number_measure {
            for k in 0..number_update {
                if write_configurations {
                    write_lattice(String::from(format!("lattice_{}.csv", i)), &lat);
                }
                updater.random_walk_update(&mut lat);
            }
            density_estimator.count_in_out(&lat);
            density_estimator.write_total_count(String::from(format!("density_estimator_{}.csv", 0)))
        }
    }   
} 
