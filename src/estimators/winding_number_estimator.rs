use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::BufWriter;

#[derive(Debug)]
pub struct WindingNumberCountEstimator {
    count_horizontal: u64,
    count_vertical: u64,
    result_file_buffer: BufWriter<File>,
}

impl WindingNumberCountEstimator {
    pub fn new() -> WindingNumberCountEstimator{
        println!("Initializing WindingNumberCountEstimator");

        println!("Opening WindingNumberCountEstimator file");
        let path = Path::new("winding_number_count_estimator.csv");
        let display = path.display();
        let file = match File::create(&path) {
            Err(err) => panic!("could not create {}: {}",
                display,
                err.description()),
            Ok(good_file) => good_file,
        };

        let result_file_buffer = BufWriter::new(file);

        let mut winding_number_count_estimator = WindingNumberCountEstimator{
            count_horizontal: 0,
            count_vertical: 0,
            result_file_buffer,
        };

        let mut header_string = String::new();
        header_string.push_str("Horizontal,Vertical\n");
        match total_link_count_estimator.result_file_buffer.write(header_string.as_bytes()){
            Err(_err) => panic!("Can not write winding number count header."),
            Ok(_) => println!("Wrote total link count header."),
        };

        println!("Done initializing winding number count estimator.");

        total_link_count_estimator
    }
}
