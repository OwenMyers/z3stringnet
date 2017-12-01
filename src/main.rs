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

struct Lattice {
    hrz: Vec<u8>,
    vrt: Vec<u8>,
    lx: u64,
    ly: u64,
}

fn write_lattice(lat: &Lattice) {
    write_vec(String::from("vrt.txt"), &lat.vrt);
    write_vec(String::from("hrz.txt"), &lat.hrz);
}

fn main() {

    let mut lat: Lattice = Lattice {
        hrz: Vec::new(),
        vrt: Vec::new(),
        lx: 2,
        ly: 2,
    };

    lat.hrz.push(0);
    lat.hrz.push(0);
    lat.vrt.push(0);
    lat.vrt.push(0);
    
    write_lattice(&lat);
    
    //let mut vrt = vec![vec![0; 4]; 4];
    //let mut hrz = vec![vec![0; 4]; 4];

    //write_2d_vec(&lat.vrt);

} 
