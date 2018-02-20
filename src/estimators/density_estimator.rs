use super::Measureable;
use super::super::datamodel::VertexLinkCount;
use super::super::datamodel::Link;
use super::super::datamodel::Point;
use super::super::datamodel::lattice::Lattice;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::BufWriter;

/// Measures the string density
/// 
/// By counting the number of non blank links we can measure the string density.
/// For prosterity we also count the number of `In` and `Out` links seperatly.
#[derive(Debug)]
pub struct DensityEstimator {
    cur_link_in_count: Vec<VertexLinkCount>,
    cur_link_out_count: Vec<VertexLinkCount>,
    cur_total_count: Vec<VertexLinkCount>,
    result_file_buffer: BufWriter<File>,
}
impl DensityEstimator{

    /// static "constructor" method.
    pub fn new(size: &Point) -> DensityEstimator{
        println!("Initilizing DensityEstimator"); 
        
        println!("Opening density estimator file;");
        let path = Path::new("density_estimator.csv");
        let display = path.display();
        let file = match File::create(&path){
            Err(err) => panic!("could not create {}: {}",
                            display,
                            err.description()),
            Ok(good_file) => good_file,
        };

        let result_file_buffer = BufWriter::new(file);

        let mut density_estimator = DensityEstimator{
            cur_link_in_count: Vec::new(),
            cur_link_out_count: Vec::new(),
            cur_total_count: Vec::new(),
            result_file_buffer: result_file_buffer,
        };

        let half_n = (size.x * size.y)/2; 
        for i in 0..half_n {
            let cur_vertex_link_count = VertexLinkCount::new(i, size);
            density_estimator.cur_link_in_count.push(cur_vertex_link_count);
            // cur_vertex_link_count was consumed so make another for out count
            let cur_vertex_link_count = VertexLinkCount::new(i, size);
            density_estimator.cur_link_out_count.push(cur_vertex_link_count);
            let cur_vertex_link_count = VertexLinkCount::new(i, size);
            density_estimator.cur_total_count.push(cur_vertex_link_count);
        }

        println!("Done initilizing density estimator.");
        return density_estimator
    }
    pub fn write_total_count(&self, f_str: String) {
        println!("Writing density estimator total count");
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

        for vertex in &self.cur_total_count{
            out_string.push_str(
                    &format!("{},{},{},{},{},{}\n",
                            vertex.xy.x,
                            vertex.xy.y,
                            &vertex.n,
                            &vertex.e,
                            &vertex.s,
                            &vertex.w,
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
}

impl Measureable for DensityEstimator {
    fn finalize_bin_and_write(&mut self, denominator: u64) {
        // Devide all of the counts by `denominator`, which is the 
        // number of measurments per bin, and write the result.
        let float_denominator = denominator as f64;
        let mut out_string = String::new();
        out_string.push_str("x,y,N,E,S,W");
        for vertex in self.cur_total_count.iter(){
            out_string.push_str(
                    &format!("{},{},{},{},{},{}\n",
                            vertex.xy.x,
                            vertex.xy.y,
                            (vertex.n as f64) / float_denominator,
                            (vertex.e as f64) / float_denominator,
                            (vertex.s as f64) / float_denominator,
                            (vertex.w as f64) / float_denominator,
                            )
                    );
        }
        out_string.push_str("\n");
        match self.result_file_buffer.write(out_string.as_bytes()){
            Err(err) => panic!("Can not write to density estimator buffer: {}",
                err.description()),
            Ok(_) => println!("Wrote measurment to density estimator buffer.") ,
        }
    }
    // We are just going to count "in" and "out" for each link of
    // the real vertices.
    fn measure(&mut self, lat: &Lattice){
        // for each direction add to the cur_in_count, cur_out_count
        // vectors if you find those directions.
        // loop over real vertices
        for (i, cur_vertex) in lat.vertices.iter().enumerate(){
            match cur_vertex.n {
                Link::In  => {
                    self.cur_link_in_count[i].n += 1;
                    self.cur_total_count[i].n += 1;
                },
                Link::Out => {
                    self.cur_link_out_count[i].n += 1;
                    self.cur_total_count[i].n += 1;
                },
                Link::Blank => (),
            }
            match cur_vertex.e {
                Link::In  => {
                    self.cur_link_in_count[i].e += 1;
                    self.cur_total_count[i].e += 1;
                },
                Link::Out => {
                    self.cur_link_out_count[i].e += 1;
                    self.cur_total_count[i].e += 1;
                },
                Link::Blank => (),
            }
            match cur_vertex.s {
                Link::In  => {
                    self.cur_link_in_count[i].s += 1;
                    self.cur_total_count[i].s += 1;
                },
                Link::Out => {
                    self.cur_link_out_count[i].s += 1;
                    self.cur_total_count[i].s += 1;
                },
                Link::Blank => (),
            }
            match cur_vertex.w {
                Link::In  => {
                    self.cur_link_in_count[i].w += 1;
                    self.cur_total_count[i].w += 1;
                },
                Link::Out => {
                    self.cur_link_out_count[i].w += 1;
                    self.cur_total_count[i].w += 1;
                },
                Link::Blank => (),
            }
        }
    }
}