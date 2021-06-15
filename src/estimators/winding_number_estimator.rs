use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use super::Measurable;
use std::io::prelude::*;
use super::super::datamodel::Link;
use super::super::datamodel::Point;
use super::super::datamodel::lattice::Lattice;
use super::super::datamodel::Direction;

#[derive(Debug)]
pub struct WindingNumberCountEstimatorDisplay {
    local_text: str,
    position: Point,
}

#[derive(Debug)]
pub struct WindingNumberCountEstimator {
    count_horizontal: i64,
    count_vertical: i64,
    result_file_buffer: BufWriter<File>,
    // Additions for the iterator
    iterator_location: i64,
    cur_point: Point,
    cur_grab_direction: Direction,
    vert_winding_count: i64,
    iter_done: bool,
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
                err),
            Ok(good_file) => good_file,
        };

        let result_file_buffer = BufWriter::new(file);

        let mut winding_number_count_estimator = WindingNumberCountEstimator{
            count_horizontal: 0,
            count_vertical: 0,
            result_file_buffer,
            iterator_location: 0,
            cur_point: Point {x: 0, y: 0},
            cur_grab_direction: Direction::N,
            vert_winding_count: 0,
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

    pub fn simple_add_sub_from_link_direction(num_in: &mut i64, link_in: &Link) {
        match link_in {
            Link::In => *num_in -= 1,
            Link::Out => *num_in += 1,
            _ => ()
        }
    }

    pub fn modulo_winding_number(x: i64) -> u64 {
        //println!("x {}", x);
        let modulus_by = 3;
        let mut to_return: u64;
        if x < 0 {
            //println!("x is less than zero (in if)");
            to_return = ((x % modulus_by) + modulus_by) as u64;
            //println!("to_return in if {}", to_return);
            // in case x is E.g. -3 then mod it is 0 and add 3 is 3.
            to_return = (to_return % (modulus_by as u64)) as u64;
        }
        else {
            to_return = (x % modulus_by) as u64;
        }
        //println!("to_return here 2 {}", to_return);
        to_return
    }
}

impl Iterator for WindingNumberCountEstimator {
    type Item = WindingNumberCountEstimatorDisplay;

    fn next(&mut self) -> WindingNumberCountEstimatorDisplay {
        if self.iterator_location % 2 == 0 {
            self.cur_point = Point {x: 0, y: self.iterator_location};
            self.cur_grab_direction = Direction::E;
        }
        else {
            self.cur_point = Point {x: 1, y: self.iterator_location};
            self.cur_grab_direction = Direction::W;
        }

        let cur_link: &Link = lat.safe_get_link_from_point(&self.cur_point, &self.cur_grab_direction);
        let maybe_flipped_link: Link;
        if self.iterator_location % 2 == 1 {
            maybe_flipped_link = self.cur_link.clone().flip();
        }
        else {
            maybe_flipped_link = *self.cur_link;
        }

        WindingNumberCountEstimator::simple_add_sub_from_link_direction(
            &mut self.vert_winding_count, &maybe_flipped_link
        );

        let mod_count = WindingNumberCountEstimator::modulo_winding_number(self.vert_winding_count) as i64;

        self.count_vertical = vert_winding_count;

        if self.iter_done {
            println!("Winding Number Iterator Complete")
        } else {
            self.iterator_location += 1;
        }
    }
}

impl Measurable for WindingNumberCountEstimator {

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
                //println!("************************************************************");
                //println!("In mod 0 and cur point is {:?}", cur_point);
                //println!("In mod 0 and cur point check is {:?}", cur_point_check);
                //println!("In mod 0 and cur grab direction is {:?}", cur_grab_direction);
                //println!("In mod 0 and cur grab direction check is {:?}", cur_grab_direction_check);
                //println!("************************************************************");
            }
            else {
                cur_point = Point {x: 1, y: i};
                cur_point_check = Point {x: 1, y: i};
                cur_grab_direction = Direction::W;
                cur_grab_direction_check = Direction::E;
                //println!("************************************************************");
                //println!("In mod else and cur point is {:?}", cur_point);
                //println!("In mod else and cur point check is {:?}", cur_point_check);
                //println!("In mod else and cur grab direction is {:?}", cur_grab_direction);
                //println!("In mod else and cur grab direction check is {:?}", cur_grab_direction_check);
                //println!("************************************************************");
            }

            let cur_link: &Link = lat.safe_get_link_from_point(&cur_point, &cur_grab_direction);
            //println!("cur_link {:?}", cur_link);
            let cur_link_check: &Link = lat.safe_get_link_from_point(&cur_point_check, &cur_grab_direction_check);
            let maybe_flipped_link: Link;
            let maybe_flipped_link_check: Link;
            if i % 2 == 1 {
                //println!("flipping cur_link (before) {:?}", cur_link);
                maybe_flipped_link = cur_link.clone().flip();
                //println!("flipping cur_link (after) {:?}", maybe_flipped_link);
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
            //println!("i {}", i);
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
            //println!("cur_link {:?}", cur_link);
            let cur_link_check: &Link = lat.safe_get_link_from_point(&cur_point_check, &cur_grab_direction_check);
            let maybe_flipped_link: Link;
            let maybe_flipped_link_check: Link;
            if i % 2 == 1 {
                //println!("flipping cur_link (before) {:?}", cur_link);
                maybe_flipped_link = cur_link.clone().flip();
                //println!("flipping cur_link (after) {:?}", maybe_flipped_link);
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
        self.count_horizontal = horz_winding_count;
        self.count_vertical = vert_winding_count;
    }

    fn finalize_bin_and_write(&mut self, denominator: u64) {
        let avg_count_vertical: f64 = (self.count_vertical as f64) / (denominator as f64);
        let avg_count_horizontal: f64 = (self.count_horizontal as f64) / (denominator as f64);

        let mut out_string: String = String::new();
        out_string.push_str(&format!("{},{}\n",&avg_count_horizontal,&avg_count_vertical));

        match self.result_file_buffer.write(out_string.as_bytes()) {
            Err(err) => panic!("Can not write to winding count estimator file {}",
                err
            ),
            Ok(_) => (),
        }
    }

    fn clear(&mut self) {
        self.count_vertical = 0;
        self.count_horizontal = 0;
    }
}
