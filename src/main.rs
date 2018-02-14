//use std::error::Error;
extern crate z3stringnet;
use z3stringnet::datamodel::*;
use z3stringnet::datamodel::lattice::*;
use z3stringnet::oio::*;
    

fn main() {

    let size: Point = Point {
        x: 4,
        y: 4,
    };

    let mut lat: Lattice;
    // lat now owns size -> That is good and intentional
    //lat = build_blank_lat(size);
    lat = build_z3_striped_lat(size);

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

    // Make some updates and print the results.
    //for i in 0..2 {
    //    write_lattice(String::from(format!("lattice_{}.csv", i)), &lat);
    //    updater.random_walk_update(&mut lat);
    //}   
} 
