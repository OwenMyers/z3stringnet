use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn write_vec(f_str: String, vec: &Vec<u8>) {
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

fn write_2d_vec(vec: &Vec<Vec<i32>>) {


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

enum Link {
    In,
    Out,
    Blank,
}

struct Vertex {
    n: Link,
    e: Link,
    s: Link,
    w: Link,
}

struct Size {
    lx: u64,
    ly: u64,
}

struct Lattice {
    // All links can be defined by the vertices of one sublattice.
    // This means the len of vertices will always be N/2, where N is the
    // total number of vertices.
    // TODO: Do a check or asertation to ensure the length of vertices
    // is correct given Size.
    vertices: Vec<Vertex>,
    size: Size,
}

fn build_blank_lat(size: Size) -> Lattice {
    println!("Building blank lattice of size x {}, y {}",
             size.lx, size.ly);

    let lat: Lattice = Lattice {
        vertices = Vec::new(),
        size,
    }

    // Only need half of N because we only need vertices from one sub
    // lattice to compleatly define all links.
    let half_N = lat.size.lx*lat.size.ly
        
    println!("Filling vertex array:");
    for i in (..half_N) {
        println!("i {}", i)
        let cur_vertex: Vertex = Vertex{
            n: Link::Blank,
            e: Link::Blank,
            s: Link::Blank,
            w: Link::Blank,
        }
        lat.vertices.push(cur_vertex);
    }

    lat
}
    

fn main() {

    let mut lat: Lattice = Lattice {
        vertices: Vec::new(),
        lx: 2,
        ly: 2,
    };
    let mut v: Vertex = Vertex{
        n: Link::In,
        e: Link::In,
        s: Link::Out,
        w: Link::Blank,
    };
    
    match v.n {
        Link::Out => println!("out"),
        Link::In => println!("in"),
        Link::Blank => println!("blank"),
    }

    

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
