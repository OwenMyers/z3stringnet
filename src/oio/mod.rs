use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use super::datamodel::Link;
use super::datamodel::Direction;
use super::datamodel::Vertex;
use super::datamodel::Point;
use super::datamodel::BoundPoint;
use super::datamodel::lattice::Lattice;

/// real_bool: If true this is link from a real vertex (lower left of plaquett)
/// If false this is link from a fake vertex (upper right of plaquett)
fn get_plaquett_out_string_from_link<'a>(link: &'a Link, real_bool: bool, direction: &'a Direction) -> &'a str {
    if real_bool {
        match direction {
            &Direction::N => match link{
                &Link::In => "S",
                &Link::Out => "N",
                &Link::Blank => "B",
            },
            &Direction::E => match link {
                &Link::In => "W",
                &Link::Out => "E",
                &Link::Blank => "B",
            }
            _ => panic!("Unexpected direction for determining plaquett string.")
        }
    }
    else {
        match direction {
            &Direction::S => match link{
                &Link::In => "N",
                &Link::Out => "S",
                &Link::Blank => "B",
            },
            &Direction::W => match link {
                &Link::In => "E",
                &Link::Out => "W",
                &Link::Blank => "B",
            }
            _ => panic!("Unexpected direction for determining plaquett string.")
        }
    }
}

fn get_out_string_from_link(link: &Link) -> &str {
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

pub fn write_lattice(f_str: String, lat: &mut Lattice, style: u8) {
    if style == 1 {
        write_lattice_style_1(f_str, lat);
    }
    else if style == 2 {
        write_lattice_style_2(lat);
    }
    else if style == 0 {
        write_lattice_style_1(f_str, lat);
        write_lattice_style_2(lat);
    }
}

pub fn write_lattice_style_2(lat: &mut Lattice) {
    let file_and_path = Path::new("lattice_configurations.csv");
    let mut line_out_str= String::new();

    if !Path::new(&file_and_path).exists() {
        let mut file_obj = match File::create(&file_and_path) {
            Ok(f) => f,
            Err(e) => panic!("Problem creating file to write configurations: {}", e)
        };
    }
    let mut file_obj = match OpenOptions::new().write(true).append(true).open(&file_and_path)
    {
        Ok(f) => f,
        Err(e) => panic!("Problem creating/opening file to write configurations: {}", e)
    };

    for y in 0..lat.size.y {
        for x in 0..lat.size.x {

            let current_vertex: Vertex = lat.get_vertex_from_point(
                &BoundPoint{
                    size: Point{
                        x: lat.size.x, y: lat.size.y
                    },
                    location: Point{x, y}
                }
            );

            let mut final_comma_str = ",";
            if (y == lat.size.y - 1) && (x == lat.size.x - 1) {
                final_comma_str = "";
            }

            line_out_str.push_str(
                &format!(
                    "{},{}{}",
                     match current_vertex.e{
                         Link::In => 2,
                         Link::Out => 1,
                         Link::Blank => 0,
                     },
                     match current_vertex.n{
                         Link::In => 2,
                         Link::Out => 1,
                         Link::Blank => 0,
                     },
                    final_comma_str
                )
            )
        }
    }
    line_out_str.push_str("\n");

    match file_obj.write_all(line_out_str.as_bytes()) {
        Ok(()) => println!("Wrote configuration to file (2)"),
        Err(_) => panic!("Problem writing configuration (2)")
    }
}

/// Style 1 <- read the cli.yml file for verbose description
pub fn write_lattice_style_1(f_str: String, lat: &Lattice) {
    let vertex_f_str: String = format!("{}_{}", "vertex", f_str);
    let plaquett_f_str: String = format!("{}_{}", "plaquett", f_str);
    let vertex_path = Path::new(&vertex_f_str);
    let plaquett_path = Path::new(&plaquett_f_str);

    let mut vertex_file = match File::create(&vertex_path) {
        Ok(f) => f,
        Err(e) => panic!("Problem creating vertex config file: {}", e)
    };
    let mut plaquett_file = match File::create(&plaquett_path) {
        Ok(f) => f,
        Err(e) => panic!("Problem creating plaquett config file: {}", e)
    };

    let mut vertex_out_str= String::new();
    vertex_out_str.push_str("x,y,N,E,S,W\n");
    let mut plaquett_out_str= String::new();
    plaquett_out_str.push_str("x,y,N,E,S,W\n");

    let mut working_loc: BoundPoint;
    for vertex in &lat.vertices{

        vertex_out_str.push_str(
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
        let temp_loc2 = increment_loc(&Direction::E, &working_loc);
        let temp_loc3 = increment_loc(&Direction::S, &working_loc);
        let fake_vertex = Vertex{
            n: Link::soft_flip(lat.safe_get_link_from_point(&temp_loc1.location, &Direction::S)),
            e: Link::soft_flip(lat.safe_get_link_from_point(&temp_loc2.location, &Direction::W)),
            s: Link::soft_flip(lat.safe_get_link_from_point(&temp_loc3.location, &Direction::N)),
            w: Link::soft_flip(&vertex.e),
            xy: working_loc.location
        };
        vertex_out_str.push_str(
            &format!(
                "{},{},{},{},{},{}\n",
                fake_vertex.xy.x,
                fake_vertex.xy.y,
                get_out_string_from_link(&fake_vertex.n),
                get_out_string_from_link(&fake_vertex.e),
                get_out_string_from_link(&fake_vertex.s),
                get_out_string_from_link(&fake_vertex.w),
            )
        );
        // Upper corner of plaquett 1
        let vertex_corner = Vertex{
            n: *lat.safe_get_link_from_point(&temp_loc1.location, &Direction::N),
            e: *lat.safe_get_link_from_point(&temp_loc1.location, &Direction::E),
            s: *lat.safe_get_link_from_point(&temp_loc1.location, &Direction::S),
            w: *lat.safe_get_link_from_point(&temp_loc1.location, &Direction::W),
            xy: temp_loc1.location
        };
        plaquett_out_str.push_str(
            &format!(
                "{},{},{},{},{},{}\n",
                vertex.xy.x,
                vertex.xy.y,
                get_plaquett_out_string_from_link(&vertex_corner.w, false, &Direction::W),
                get_plaquett_out_string_from_link(&vertex_corner.s, false, &Direction::S),
                get_plaquett_out_string_from_link(&vertex.e, true, &Direction::E),
                get_plaquett_out_string_from_link(&vertex.n, true, &Direction::N),
             )
        );
        // Upper corner of plaquett 2
        let temp_loc4 = increment_loc(&Direction::N, &temp_loc2);
        let temp_loc5 = increment_loc(&Direction::N, &temp_loc4);
        let temp_loc6 = increment_loc(&Direction::E, &temp_loc4);
        let temp_loc7 = increment_loc(&Direction::S, &temp_loc4);
        let temp_loc8 = increment_loc(&Direction::W, &temp_loc4);
        println!("x: {} y: {}", temp_loc4.location.x, temp_loc4.location.y);
        let vertex_corner_2 = Vertex {
            n: Link::soft_flip(lat.safe_get_link_from_point(&temp_loc5.location, &Direction::S)),
            e: Link::soft_flip(lat.safe_get_link_from_point(&temp_loc6.location, &Direction::W)),
            s: Link::soft_flip(lat.safe_get_link_from_point(&temp_loc7.location, &Direction::N)),
            w: Link::soft_flip(lat.safe_get_link_from_point(&temp_loc8.location, &Direction::E)),
            xy: working_loc.location
        };
        plaquett_out_str.push_str(
            &format!(
                "{},{},{},{},{},{}\n",
                fake_vertex.xy.x,
                fake_vertex.xy.y,
                get_plaquett_out_string_from_link(&vertex_corner_2.w, false, &Direction::W),
                get_plaquett_out_string_from_link(&vertex_corner_2.s, false, &Direction::S),
                get_plaquett_out_string_from_link(&fake_vertex.e, true, &Direction::E),
                get_plaquett_out_string_from_link(&fake_vertex.n, true, &Direction::N),
            )
        );
    }
    vertex_out_str.push_str("\n");
    plaquett_out_str.push_str("\n");

    match vertex_file.write_all(vertex_out_str.as_bytes()) {
        Ok(()) => println!("Wrote vertex configuration to file (1)"),
        Err(_) => panic!("Problem vertex writing configuration (1)")
    }
    match plaquett_file.write_all(plaquett_out_str.as_bytes()) {
        Ok(()) => println!("Wrote plaquett configuration to file (1)"),
        Err(_) => panic!("Problem plaquett writing configuration (1)")
    }

}

pub fn write_vec(f_str: String, vec: &Vec<u8>) {
    let path = Path::new(&f_str);
    let display = path.display();

    let mut file = match File::create(&path){
        Err(err) => panic!("could not create {}: {}",
                           display,
                           err),
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
                           err),
        Ok(_) => println!("file out worked"),
    }
}


pub fn write_2d_vec(vec: &Vec<Vec<i32>>) {

    let path = Path::new("foo.txt");
    let display = path.display();

    let mut file = match File::create(&path){
        Err(err) => panic!("could not create {}: {}",
                           display,
                           err),
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
                           err),
        Ok(_) => println!("fjile out worked"),
    }
}
