use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use super::datamodel::Link;
use super::datamodel::Direction;
use super::datamodel::Vertex;
use super::datamodel::Point;
use super::datamodel::BoundPoint;
use super::datamodel::lattice::Lattice;
use std::error::Error;


fn get_out_string_from_link(link: &Link) -> &str{
    match link{
        &Link::In => "In",
        &Link::Out => "Out",
        &Link::Blank => "Blank",
    }
}

pub fn increment_loc(direction: &Direction, location_in: &BoundPoint) -> BoundPoint{
    let increment: Option<Point>; 
    match *direction {
        Direction::N => {
            increment = Some(Point {x: 0, y: 1});
        },
        Direction::E => {
            increment = Some(Point {x: 1, y: 0});
        },
        Direction::S => {
            increment = Some(Point {x: 0, y: -1});
        },
        Direction::W => {
            increment = Some(Point {x: -1, y: 0});
        },
    }
    return match increment {
        Some(inc) => location_in + inc,
        None => panic!("No step taken for some reason. No increment."),
    }
}

pub fn write_lattice(f_str: String, lat: &Lattice) {
    let path = Path::new(&f_str);
    let display = path.display();

    let mut file = match File::create(&path){
        Err(err) => panic!("could not create {}: {}",
                           display,
                           err.description()),
        Ok(good_file) => good_file,
    };

    let mut out_string = String::new();
    out_string.push_str("x,y,N,E,S,W\n");

    let mut working_loc: BoundPoint;
    for vertex in &lat.vertices{
        out_string.push_str(
            &format!("{},{},{},{},{},{}\n",
                vertex.xy.x,
                vertex.xy.y,
                get_out_string_from_link(&vertex.n),
                get_out_string_from_link(&vertex.e),
                get_out_string_from_link(&vertex.s),
                get_out_string_from_link(&vertex.w),
                )
        );
        // Write the other sublattice as well

        working_loc = BoundPoint {
            size: lat.size,
            location: vertex.xy,
        };
        working_loc = &working_loc + Point{x: 1, y: 0};

        let temp_loc1 = increment_loc(&Direction::N, &working_loc);
        let fake_vertex = Vertex{
            n: Link::soft_flip(lat.safe_get_link_from_point(&temp_loc1.location, &Direction::S)),
            e: Link::soft_flip(lat.safe_get_link_from_point(&temp_loc1.location, &Direction::S)),
            s: Link::soft_flip(lat.safe_get_link_from_point(&temp_loc1.location, &Direction::S)),
            w: Link::soft_flip(lat.safe_get_link_from_point(&temp_loc1.location, &Direction::S)),
            xy: working_loc.location
        };
        out_string.push_str(
            &format!("{},{},{},{},{},{}\n",
                fake_vertex.xy.x,
                fake_vertex.xy.y,
                get_out_string_from_link(&fake_vertex.n),
                get_out_string_from_link(&fake_vertex.e),
                get_out_string_from_link(&fake_vertex.s),
                get_out_string_from_link(&fake_vertex.w),
                )
        );
        //temp_loc = increment_loc(&Direction::E, &working_loc);
        //fake_vertex.n = lat.get_link_from_point(&temp_loc.location, &Direction::W).flip();
        //temp_loc = increment_loc(&Direction::S, &working_loc);
        //fake_vertex.n = lat.get_link_from_point(&temp_loc.location, &Direction::N).flip();
        //temp_loc = increment_loc(&Direction::W, &working_loc);
        //fake_vertex.n = lat.get_link_from_point(&temp_loc.location, &Direction::E).flip();
    }
    out_string.push_str("\n");
    println!("{}", out_string);

    match file.write_all(out_string.as_bytes()){
        Err(err) => panic!("could not create {}: {}",
                           display,
                           err.description()),
        Ok(_) => println!("file out worked"),
    }
}

pub fn write_vec(f_str: String, vec: &Vec<u8>) {
    let path = Path::new(&f_str);
    let display = path.display();

    let mut file = match File::create(&path){
        Err(err) => panic!("could not create {}: {}",
                           display,
                           err.description()),
        Ok(good_file) => good_file,
    };

    let mut out_string = String::new();
    for i in vec{
        out_string.push_str(&format!("{} ", i));
    }
    out_string.push_str("\n");
    println!("{}", out_string);

    match file.write_all(out_string.as_bytes()){
        Err(err) => panic!("could not create {}: {}",
                           display,
                           err.description()),
        Ok(_) => println!("fjile out worked"),
    }


}


pub fn write_2d_vec(vec: &Vec<Vec<i32>>) {

    let path = Path::new("foo.txt");
    let display = path.display();

    let mut file = match File::create(&path){
        Err(err) => panic!("could not create {}: {}",
                           display,
                           err.description()),
        Ok(good_file) => good_file,
    };

    let mut out_string = String::new();
    for i in vec{
        for j in i{
            out_string.push_str(&format!("{} ", j))
        }
        out_string.push_str("\n")
    }
    println!("{}", out_string);

    match file.write_all(out_string.as_bytes()){
        Err(err) => panic!("could not create {}: {}",
                           display,
                           err.description()),
        Ok(_) => println!("fjile out worked"),
    }
}
