use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::BufWriter;
use super::Measurable;
use super::super::datamodel::Link;
use super::super::datamodel::Point;
use super::super::datamodel::lattice::Lattice;
use super::super::datamodel::Direction;

#[derive(Debug)]
pub struct WindingNumberCountEstimator {
    count_horizontal: i64,
    count_vertical: i64,
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
        match winding_number_count_estimator.result_file_buffer.write(header_string.as_bytes()){
            Err(_err) => panic!("Can not write winding number count header."),
            Ok(_) => println!("Wrote total link count header."),
        };

        println!("Done initializing winding number count estimator.");

        winding_number_count_estimator
    }

    fn simple_add_sub_from_link_direction(&self, num_in: &mut i64, link_in: &Link) {
        match link_in {
            Link::In => *num_in -= 1,
            Link::Out => *num_in += 1,
            Link::Blank => 0,
        }
    }
}

impl Measurable for WindingNumberCountEstimator {
    fn measure(&mut self, lat: &Lattice) {
        // First count winding number in vertical direction along column at origin.
        // Also count winding number in vertical direction along column at origin + 1.
        // We can assert that this needs to be the same winding number as that found from the
        // origin column as a safety check.

        // Direction of horizontal links to get
        let vert_winding_count: i64 = 0;
        let vert_winding_count_check: i64 = 0;

        let cur_point: Point;
        let cur_point_check: Point;
        let cur_grab_direction: Direction;
        let cur_grab_direction_check: Direction;
        for i in 0..lat.size.y {
            if (i % 2 == 0) {
                let cur_point: Point = Point {x: 0, y: i};
                let cur_point_check: Point = Point {x: 2, y: i};
                let cur_grab_dirction = Direction::E;
                let cur_grab_direction_check = Direction::W;
            }
            else {
                let cur_point: Point = Point {x: 1, y: i};
                let cur_point_check: Point = Point {x: 1, y: i};
                let cur_grab_dirction = Direction::W;
                let cur_grab_direction_check = Direction::E;
            }

            let cur_link: &Link = lat.safe_get_link_from_point(&cur_point, &cur_grab_direction);
            self.simple_add_sub_from_link_direction(&mut vert_winding_count, cur_link);
            let cur_link_check: &Link = lat.safe_get_link_from_point(&cur_point_check, &cur_grab_direction_check);
            self.simple_add_sub_from_link_direction(&mut vert_winding_count_check, cur_link_check);

        }
        assert_eq!(vert_winding_count, vert_winding_count_check);

        // Do the same for the horizontal direction
        HERE
    }

    fn finalize_bin_and_write(&mut self, denominator: u64) {
        let avg_count_vertical: f64 = (self.count_vertical as f64) / (denominator as f64);
        let avg_count_horizontal: f64 = (self.count_horizontal as f64) / (denominator as f64);

        let mut out_string: String = String::new();
        out_string.push_str(&format!("{},{}\n",&avg_count_horizontal,&avg_count_vertical));

        match self.result_file_buffer.write(out_string.as_bytes()) {
            Err(err) => panic!("Can not write to winding count estimator file {}",
                err.description()
            ),
            Ok(_) => (),
        }
    }

    fn clear(&mut self) {
        self.count_vertical = 0;
        self.count_horizontal = 0;
    }
}
