//use std::error::Error;
use datamodel::*
    

fn main() {

    let size: Point = Point {
        x: 2,
        y: 2,
    };

    let mut lat: Lattice;
    lat = build_blank_lat(size);

    write_lattice(String::from("lattice.txt"), &lat);



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
