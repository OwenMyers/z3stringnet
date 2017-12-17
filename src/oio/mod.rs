use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use super::datamodel::Link;
use super::datamodel::lattice::Lattice;
use std::error::Error;

fn get_out_string_from_link(link: &Link) -> &str{
    match link{
        &Link::In => "In",
        &Link::Out => "Out",
        &Link::Blank => "Blank",
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

    for vertex in &lat.vertices{
        //TODO: Need to write xy here.
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
