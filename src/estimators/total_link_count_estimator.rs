use super::Measurable;
use std::path::Path;
use std::error::Error;
use std::io::BufWriter;

#[derive(Debug)]
pub struct TotalLinkCountEstimator {
    count: u64,
    result_file_buffer: BufWriter<File>,
}

impl TotalLinkCountEstimator {

    pub fn new() -> TotalLinkCountEstimator {
        println!("Initializing TotalLinkCountEstimator");

        println!("Opening density estimator file");
        let path = Path::new("total_link_count_estimator.csv")
        let display = path.display();
        let file = match File::create(&path) {
            Err(err) => panic!("could not create {}: {}",
                display,
                err.description()),
            Ok(good_file) => good_file,
        };

        let result_file_buffer = BufWriter::new(file);

        let mut total_link_count_estimator = TotalLinkCountEstimator{
            count: 0,
            result_file_buffer,
        };
        let mut header_string = String::new();
        header_string.push_str("Average (per bin) Total Counts\n")
        match result_file_buffer.write(header_string.as_bytes()){
            Err(err) => panic!("Can now write total link count header."),
            Ok(_) => println!("Wrote total link count header."),
        }
    }
}

impl Measurable for TotalLinkCountEstimator {
    fn clear(&mut self){
        self.count = 0;
    }

    fn finalize_bin_and_write(&mut self, denominator: u64) {
        let avg_count: f64 = (count as f64) / (denominator as f64);
        let mut out_string: String = String::new();
        out_string.push_str(format!("{}\n", avg_count))

        match self.result_file_buffer.write(out_string.as_bytes()){
            Err(err) => panic!("Can not write to total link count estimator buffer {}",
                err.description()),
            Ok(_) => (),
        }
    }

    fn measure(&mut self, lat: &Lattice){
        for (i, cur_vertex) in lat.vertices.iter().enumerate(){
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
