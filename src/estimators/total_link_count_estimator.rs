use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::BufWriter;
use std::io::prelude::*;
use super::Measurable;
use super::super::datamodel::Link;
use super::super::datamodel::lattice::Lattice;

#[derive(Debug)]
pub struct TotalLinkCountEstimator {
    count: u64,
    result_file_buffer: BufWriter<File>,
}

impl TotalLinkCountEstimator {

    pub fn new() -> TotalLinkCountEstimator {
        println!("Initializing TotalLinkCountEstimator");

        println!("Opening density estimator file");
        let path = Path::new("total_link_count_estimator.csv");
        let display = path.display();
        let file = match File::create(&path) {
            Err(err) => panic!("could not create {}: {}",
                display,
                err),
            Ok(good_file) => good_file,
        };

        let result_file_buffer = BufWriter::new(file);

        let mut total_link_count_estimator = TotalLinkCountEstimator{
            count: 0,
            result_file_buffer,
        };

        let mut header_string = String::new();
        header_string.push_str("Average Total Link Counts\n");
        match total_link_count_estimator.result_file_buffer.write(header_string.as_bytes()){
            Err(_err) => panic!("Can not write total link count header."),
            Ok(_) => println!("Wrote total link count header."),
        };

        println!("Done initializing total link count estimator.");

        total_link_count_estimator
    }
}

impl Measurable for TotalLinkCountEstimator {
    fn clear(&mut self){
        self.count = 0;
    }

    fn finalize_bin_and_write(&mut self, denominator: u64) {
        let avg_count: f64 = (self.count as f64) / (denominator as f64);
        let mut out_string: String = String::new();
        out_string.push_str(&format!("{}\n", &avg_count));

        match self.result_file_buffer.write(out_string.as_bytes()){
            Err(err) => panic!("Can not write to total link count estimator buffer {}",
                err),
            Ok(_) => (),
        }
    }

    fn measure(&mut self, lat: &mut Lattice){
        for (_i, cur_vertex) in lat.vertices.iter().enumerate(){
            match cur_vertex.n {
                Link::In => {
                    self.count += 1;
                }
                Link::Out => {
                    self.count += 1;
                }
                Link::Blank => ()
            }
            match cur_vertex.e {
                Link::In => {
                    self.count += 1;
                }
                Link::Out => {
                    self.count += 1;
                }
                Link::Blank => ()
            }
            match cur_vertex.s {
                Link::In => {
                    self.count += 1;
                }
                Link::Out => {
                    self.count += 1;
                }
                Link::Blank => ()
            }
            match cur_vertex.w {
                Link::In => {
                    self.count += 1;
                }
                Link::Out => {
                    self.count += 1;
                }
                Link::Blank => ()
            }
        }
    }
}
