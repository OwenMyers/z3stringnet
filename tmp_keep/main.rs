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
    lat = build_vertical_staggered_lat(size);

    let p: Point = Point {
        x: 1,
        y: 1,
    };

    let d: Direction = Direction::E;


    let mut updater: Update = Update{
        working_loc: BoundPoint{
            size: lat.size,
            location: Point{x: 0, y: 0},
        }
    };
    
    updater.update(&mut lat);

    write_lattice(String::from("lattice.txt"), &lat);

    // Need the mutable reference to go out of scope.
    //{
    //    let gotten_link: &mut Link = lat.get_link_from_point(&p, &d);
    //    println!("gotten_link {:?}", gotten_link);
    //    *gotten_link = Link::Out;
    //}

    
    //write_lattice(String::from("lattice.txt"), &lat);

    //let mut bound_point: BoundPoint = BoundPoint {
    //    size: Point{x: 4, y: 4},
    //    location: Point{x: 1, y: 1},
    //};

    //println!("Checking out X");
    //for _i in 0..10 {
    //    let update_point: Point = Point{x: -1, y: 0};
    //    bound_point = &bound_point + update_point;
    //    println!("new bound point {:?}", bound_point);
    //}

    //let mut bound_point: BoundPoint = BoundPoint {
    //    size: Point{x: 4, y: 4},
    //    location: Point{x: 1, y: 1},
    //};


    //println!("Checking out y");
    //for _i in 0..10 {
    //    let update_point: Point = Point{x: 0, y: 1};
    //    bound_point = &bound_point + update_point;
    //    println!("new bound point {:?}", bound_point);
    //}

    // Playing with the update struct.
    //let mut update: Update = Update {
    //    lat_size: Point {
    //        x: 4,
    //        y: 4,
    //    },
    //    working_loc: Point {
    //        x: 0,
    //        y: 0,
    //    }
    //};
    //println!("plaquette {:?}", update);
    //for i in 0..10 {
    //    update.get_rand_point();
    //    println!("rand x {}", update.working_loc.x);
    //    println!("rand y {}", update.working_loc.y);
    //}



    //let tmpx = x_from_vertex_vec_position(9, &lat.size);
    //println!("tmpx {}", tmpx)


    //let mut v: Vertex = Vertex{
    //    n: Link::In,
    //    e: Link::In,
    //    s: Link::Out,
    //    w: Link::Blank,
    //};
    //
    //match v.n {
    //    Link::Out => println!("out"),
    //    Link::In => println!("in"),
    //    Link::Blank => println!("blank"),
    //}

    

//    lat.hrz.push(0);
//    lat.hrz.push(0);
//    lat.vrt.push(0);
//    lat.vrt.push(0);
//    
//    write_lattice(&lat);
    
    //let mut vrt = vec![vec![0; 4]; 4];
    //let mut hrz = vec![vec![0; 4]; 4];

    //write_2d_vec(&lat.vrt);

} 
