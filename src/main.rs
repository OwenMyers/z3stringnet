use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn write_2d_vec(vec: &mut Vec<u32>) -> Result {

    for i in &mut vec{
        for j in i{
            println!("{}", j);
        }
    }

    Ok(_)

}

fn main() {
    
    let mut vrt = vec![vec![0; 3]; 3];

    //for i in &mut vrt{
    //    for j in i{
    //        println!("{}", j);
    //    }
    //}

    match write_2d_vec(&vrt){
        Err(err) => panic!("Could not write vrt"),
        Ok(_) => println!("Wrote the vector"),
    }

    let path = Path::new("foo.txt");
    let display = path.display();

    let mut file = match File::create(&path){
        Err(err) => panic!("could not create {}: {}",
                           display,
                           err.description()),
        Ok(good_file) => good_file,
    };

    match file.write_all(b"Hi there"){
        Err(err) => panic!("could not create {}: {}",
                           display,
                           err.description()),
        Ok(_) => println!("file out worked"),
    }
} 
