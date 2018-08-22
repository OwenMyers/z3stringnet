use super::datamodel::BoundPoint;
use super::datamodel::Point;
use super::datamodel::Direction;
use super::datamodel::lattice::Lattice;
use super::datamodel::Link;
use rand;
use rand::Rng;

/// This will ergodicly update the Z3 string net model.
/// 
/// A couple of choices are available.
/// * The `update` function will perform an update on random plaquets 
///   walking clockwise with the raising operator. 
/// * The `random_walk_update` will perform a random walk to produce an
///   extensive change to the configuration it operates on.
#[derive(Debug)]
pub struct Update {
    pub working_loc: BoundPoint,
    pub link_number_tuning: f64,
    pub link_number_change: i64,
}
impl Update {
    /// Determine by how much the number of non blank links has changed
    /// after a raise step.
    /// Possibilities 
    ///     1) if a raise operation just switches the orientation it returns 0.
    ///     2) if it brings an occupied link to a blank link -> -1
    ///     3) Blank to occupied -> +1
    /// We don't know which way the raise step went so we have to check all possibilities
    fn find_increase_or_decrease(before_after_links: (Link, Link)) -> i8 {
        let (before_link, after_link) = before_after_links;
        match before_link {
            Link::In => {
                match after_link {
                    Link::Blank => {
                        return -1
                    },
                    Link::Out => {
                        return 0
                    },
                    Link::In => {
                         panic!("In link can't change to In from raise or lower")
                    }
                }
            },
            Link::Out => {
                match after_link {
                    Link::In => {
                        return 0
                    },
                    Link::Blank => {
                        return -1
                    },
                    Link::Out => {
                        panic!("Out link can't change to Out from raise or lower")
                    }
                }
            },
            Link::Blank => {
                match after_link {
                    Link::In => {
                        return 1
                    },
                    Link::Out => {
                        return 1
                    },
                    Link::Blank => {
                        panic!("Blank link can't change to Blank from raise or lower")
                    }
                }

            },
        }
    }

    pub fn get_rand_point(&mut self) {

        self.working_loc.location = Point {
            x: rand::thread_rng()
                .gen_range(0, self.working_loc.size.x),
            y: rand::thread_rng()
                .gen_range(0, self.working_loc.size.y)
                
        };
        // for testing
        //self.working_loc.location = Point { x: 0, y: 0 }
    }
    pub fn update(&mut self, lat: &mut Lattice) {
        // Get a random point.
        // Lets say the random point is the lower left
        // corner of the plaquette.
        // Clockwise walk.
        self.get_rand_point();
        let mut z3string = Z3String{
            start_loc: self.working_loc.location,
            cur_loc: self.working_loc,
            lat: lat
        };
        let mut totatal_link_number_change: i64 = 0;

        let cur_direction = Direction::N;
        let before_after_links: (Link, Link) = z3string.raise_step(&cur_direction);
        let number_increase_or_decrease = Update::find_increase_or_decrease(before_after_links);
        totatal_link_number_change += number_increase_or_decrease as i64;

        let cur_direction = Direction::E;
        let before_after_links: (Link, Link) = z3string.raise_step(&cur_direction);
        let number_increase_or_decrease = Update::find_increase_or_decrease(before_after_links);
        totatal_link_number_change += number_increase_or_decrease as i64;

        let cur_direction = Direction::S;
        let before_after_links: (Link, Link) = z3string.raise_step(&cur_direction);
        let number_increase_or_decrease = Update::find_increase_or_decrease(before_after_links);
        totatal_link_number_change += number_increase_or_decrease as i64;

        let cur_direction = Direction::W;
        let before_after_links: (Link, Link) = z3string.raise_step(&cur_direction);
        let number_increase_or_decrease = Update::find_increase_or_decrease(before_after_links);
        totatal_link_number_change += number_increase_or_decrease as i64;

        assert_eq!(z3string.cur_loc, z3string.start_loc);
        self.link_number_change = totatal_link_number_change;
    }

    pub fn random_walk_update(&mut self, lat: &mut Lattice) {
        self.get_rand_point();
        let mut z3string = Z3String{
            start_loc: self.working_loc.location,
            cur_loc: self.working_loc,
            lat: lat
        };
        let mut total_link_number_change: i64 = 0;
        // Take first step before loop so cur_loc and start_loc
        // are different.  
        let cur_direction = Direction::get_random_direction();
        let before_after_links: (Link, Link) = z3string.raise_step(&cur_direction);
        let number_increase_or_decrease = Update::find_increase_or_decrease(before_after_links);
        total_link_number_change += number_increase_or_decrease as i64;

        while z3string.cur_loc != z3string.start_loc {
            //println!("In while loop: cur_loc {:?}, start_loc {:?}", z3string.cur_loc, z3string.start_loc);
            let cur_direction = Direction::get_random_direction();
            //println!("  direction {:?}", cur_direction);
            let before_after_links: (Link, Link) = z3string.raise_step(&cur_direction);
            let number_increase_or_decrease = Update::find_increase_or_decrease(before_after_links);
            total_link_number_change += number_increase_or_decrease as i64;
        }
        assert_eq!(z3string.cur_loc, z3string.start_loc);
        self.link_number_change = total_link_number_change;
    }
}

pub struct Z3String<'a> {
    /// This borrows a mutable reference to a lattice so it is assumed that an instance of this
    /// struct will be used to perform some operation on the lattice and then go out of scope so
    /// the mutable reference can be avaliabel again.
    pub start_loc: Point,
    pub cur_loc: BoundPoint,
    lat: &'a mut Lattice, 
    //pub path: Vec<Point>,
}
impl<'a> Z3String<'a> {
    fn increment_cur_loc(&mut self, direction: &Direction) {
        let increment: Option<Point>; 
        match *direction {
            Direction::N => {
                increment = Some(Point {x: 0, y: 1});
            },
            Direction::E => {
                increment = Some(Point {x: 1, y: 0});
            },
            Direction::S => {
                increment = Some(Point {x: 0, y: -1});
            },
            Direction::W => {
                increment = Some(Point {x: -1, y: 0});
            },
        }
        match increment {
            Some(inc) => self.cur_loc = &self.cur_loc + inc,
            None => panic!("No step taken for some reason. No increment."),    
        }
    }
    pub fn raise_step(&mut self, direction: &Direction) -> (Link, Link) {
        //println!("cur location before {:?}",self.cur_loc.location);
        //println!("cur direction before {:?}",direction);
        // This function takes a step along a path from the self.cur_loc position to a new
        // position determined by the input from the user. It only steps across one link and it
        // CHANGES that link according to the raising and lowing rules given the orientation of the
        // link and the direction of the step.
        //println!("    raise step self.cur_loc.location{:?}", &self.cur_loc.location);
        //println!("    direction {:?}", direction);
        let pre_raise_link: Link;
        let post_raise_link: Link;
        // If the link is a real one (point is of the stored sub lattice) then you just need
        // to call lat `get_link_from_point` and operate on that link
        if self.lat.point_real(&self.cur_loc.location){
            //println!("    real point");
            pre_raise_link = self.lat.get_link_from_point(
                &self.cur_loc.location,
                &direction
            ).clone();

            post_raise_link =
                self.lat.out_raise_link(&self.cur_loc.location, &direction);

            self.increment_cur_loc(&direction);
        }
        // If the link is not real then step in `direction`, which will guarantee you are now on
        // the real sublattice, and look back across the link from the new vertex. You have to
        // be careful operating on the link because it will be in the reverse direction now that
        // you have already taken the step.
        // "out lower" is would be the same as "in raise". Either works now that we have
        // shifted position without changing anything.
        else {
            //println!("    not real point");
            //println!("orig dir {:?}", direction);
            self.increment_cur_loc(&direction);
            let fliped_dir: Direction = direction.flip();
            //println!("new dir {:?}", flipped_dir);

            pre_raise_link = self.lat.get_link_from_point(
                &self.cur_loc.location,
                &fliped_dir
            ).clone().flip();

            post_raise_link =
                self.lat.out_lower_link(&self.cur_loc.location, &fliped_dir).flip();


        }
        //println!("    cur location after {:?}",self.cur_loc.location);
        //println!("    pre raise: {:?}, post raise {:?}", pre_raise_link, post_raise_link);
        (pre_raise_link, post_raise_link)
    }
}