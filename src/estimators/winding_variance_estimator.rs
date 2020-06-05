use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use super::Measurable;
use std::io::prelude::*;
use super::super::datamodel::Link;
use super::super::datamodel::Point;
use super::super::datamodel::lattice::Lattice;
use super::super::datamodel::Direction;
use super::winding_number_estimator::WindingNumberCountEstimator;
use std::vec::Vec;


#[derive(Debug)]
pub struct WindingNumberVarianceEstimator {
    counts_horizontal: Vec<i64>,
    counts_vertical: Vec<i64>,
    result_file_buffer: BufWriter<File>,
}

impl WindingNumberVarianceEstimator {
    pub fn new() -> WindingNumberVarianceEstimator{
        println!("Initializing WindingNumberVarianceEstimator");

        println!("Opening WindingNumberVarianceEstimator file");
        let path = Path::new("winding_number_variance_estimator.csv");
        let display = path.display();
        let file = match File::create(&path) {
            Err(err) => panic!("could not create {}: {}",
                display,
                err),
            Ok(good_file) => good_file,
        };

        let result_file_buffer = BufWriter::new(file);

        let mut winding_number_variance_estimator = WindingNumberVarianceEstimator{
            counts_horizontal: Vec::new(),
            counts_vertical: Vec::new(),
            result_file_buffer,
        };

        let mut header_string = String::new();
        header_string.push_str("Horizontal,Vertical\n");
        match winding_number_variance_estimator.result_file_buffer.write(header_string.as_bytes()){
            Err(_err) => panic!("Can not write winding number variance header."),
            Ok(_) => println!("Wrote total link count header."),
        };

        println!("Done initializing winding number variance estimator.");

        winding_number_variance_estimator
    }

}

impl Measurable for WindingNumberVarianceEstimator {

    fn measure(&mut self, lat: &Lattice) {
        // First count winding number in vertical direction along column at origin.
        // Also count winding number in vertical direction along column at origin + 1.
        // We can assert that this needs to be the same winding number as that found from the
        // origin column as a safety check.
        let mut cur_point: Point;
        let mut cur_point_check: Point;
        let mut cur_grab_direction: Direction;
        let mut cur_grab_direction_check: Direction;

        // Direction of horizontal links to get
        let mut vert_winding_count: i64 = 0;
        let mut vert_winding_count_check: i64 = 0;

        for i in 0..lat.size.y {
            //println!("i {}", i);
            if i % 2 == 0 {
                cur_point = Point {x: 0, y: i};
                cur_point_check = Point {x: 2, y: i};
                cur_grab_direction = Direction::E;
                cur_grab_direction_check = Direction::W;
            }
            else {
                cur_point = Point {x: 1, y: i};
                cur_point_check = Point {x: 1, y: i};
                cur_grab_direction = Direction::W;
                cur_grab_direction_check = Direction::E;
            }

            let cur_link: &Link = lat.safe_get_link_from_point(&cur_point, &cur_grab_direction);
            let cur_link_check: &Link = lat.safe_get_link_from_point(&cur_point_check, &cur_grab_direction_check);
            let maybe_flipped_link: Link;
            let maybe_flipped_link_check: Link;
            if i % 2 == 1 {
                maybe_flipped_link = cur_link.clone().flip();
            }
            else {
                maybe_flipped_link = *cur_link;
            }

            if i % 2 == 0 {
                maybe_flipped_link_check = cur_link_check.clone().flip();
            }
            else {
                maybe_flipped_link_check = *cur_link_check;
            }

            WindingNumberCountEstimator::simple_add_sub_from_link_direction(
                &mut vert_winding_count, &maybe_flipped_link
            );
            WindingNumberCountEstimator::simple_add_sub_from_link_direction(
                &mut vert_winding_count_check, &maybe_flipped_link_check
            );
        }

        let mod_count = WindingNumberCountEstimator::modulo_winding_number(vert_winding_count) as i64;
        let mod_count_check= WindingNumberCountEstimator::modulo_winding_number(vert_winding_count_check) as i64;

        assert_eq!(mod_count, mod_count_check);

        // Do the same for the horizontal direction
        let mut horz_winding_count: i64 = 0;
        let mut horz_winding_count_check: i64 = 0;

        for i in 0..lat.size.x {
            if i % 2 == 0 {
                cur_point = Point {x: i, y: 0};
                cur_point_check = Point {x: i, y: 2};
                cur_grab_direction = Direction::N;
                cur_grab_direction_check = Direction::S;
            }
            else {
                cur_point = Point {x: i, y: 1};
                cur_point_check = Point {x: i, y: 1};
                cur_grab_direction = Direction::S;
                cur_grab_direction_check = Direction::N;
            }

            let cur_link: &Link = lat.safe_get_link_from_point(&cur_point, &cur_grab_direction);
            let cur_link_check: &Link = lat.safe_get_link_from_point(&cur_point_check, &cur_grab_direction_check);
            let maybe_flipped_link: Link;
            let maybe_flipped_link_check: Link;
            if i % 2 == 1 {
                maybe_flipped_link = cur_link.clone().flip();
            }
            else {
                maybe_flipped_link = *cur_link;
            }

            if i % 2 == 0 {
                maybe_flipped_link_check = cur_link_check.clone().flip();
            }
            else {
                maybe_flipped_link_check = *cur_link_check;
            }

            WindingNumberCountEstimator::simple_add_sub_from_link_direction(&mut horz_winding_count, &maybe_flipped_link);
            WindingNumberCountEstimator::simple_add_sub_from_link_direction(&mut horz_winding_count_check, &maybe_flipped_link_check );
        }
        let mod_count= WindingNumberCountEstimator::modulo_winding_number(horz_winding_count) as i64;
        let mod_count_check= WindingNumberCountEstimator::modulo_winding_number(horz_winding_count_check) as i64;

        assert_eq!(mod_count, mod_count_check);
        self.counts_horizontal.push(horz_winding_count);
        self.counts_vertical.push(vert_winding_count);
    }

    fn finalize_bin_and_write(&mut self, denominator: u64) {
        let mut horz_squared_avg: f64 = 0.0;
        let mut horz_avg_squared: f64 = 0.0;
        let mut vert_squared_avg: f64 = 0.0;
        let mut vert_avg_squared: f64 = 0.0;

        for i in 0..self.counts_vertical.len() {
            horz_squared_avg += f64::powf(self.counts_horizontal[i] as f64, 2.0);
            vert_squared_avg += f64::powf(self.counts_vertical[i] as f64, 2.0);

            horz_avg_squared += self.counts_horizontal[i] as f64;
            vert_avg_squared += self.counts_vertical[i] as f64;
        }
        horz_squared_avg /= denominator as f64;
        vert_squared_avg /= denominator as f64;

        horz_avg_squared /= denominator as f64;
        horz_avg_squared = f64::powf(horz_avg_squared, 2.0);
        vert_avg_squared /= denominator as f64;
        vert_avg_squared = f64::powf(vert_avg_squared, 2.0);

        let variance_vertical: f64 = vert_squared_avg - vert_avg_squared;
        let variance_horizontal: f64 = horz_squared_avg - horz_avg_squared;

        let mut out_string: String = String::new();
        out_string.push_str(&format!("{},{}\n",&variance_horizontal,&variance_vertical));

        match self.result_file_buffer.write(out_string.as_bytes()) {
            Err(err) => panic!("Can not write to winding variance estimator file {}",
                err
            ),
            Ok(_) => (),
        }
    }

    fn clear(&mut self) {
        self.counts_vertical = Vec::new();
        self.counts_horizontal = Vec::new();
    }
}
