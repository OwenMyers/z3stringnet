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

    let mut updater: Update = Update{
        working_loc: BoundPoint{
            size: lat.size,
            location: Point{x: 0, y: 0},
        }
    };

    for i in 0..10 {
        write_lattice(String::from(format!("lattice_{}.csv", i)), &lat);
        updater.update(&mut lat);
    }   

} 
