use super::datamodel::BoundPoint;
use super::datamodel::Point;
use super::datamodel::Direction;
use super::datamodel::lattice::Lattice;
use super::datamodel::Link;
extern crate rand;
use rand::prelude::*;

pub enum UpdateType {
    Local,
    Walk,
}
#[derive(Debug)]
pub enum AcceptReject {
    Accept,
    Reject,
}

/// This will ergodicly update the Z3 string net model.
/// 
/// A couple of choices are available.
/// * The `update` function will perform an update on random plaquets 
///   walking clockwise with the raising operator. 
/// * The `random_walk_update` will perform a random walk to produce an
///   extensive change to the configuration it operates on.
///
/// Currently the random walk update will modify an attribute in lattice.
/// This is hidden when using the update method so I'm pointing it out here.
/// `number_filled_links` will be modified by adding (subtracting) the
/// `link_number_change` determined by the update function.
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
            x: thread_rng().gen_range(0, self.working_loc.size.x),
            y: thread_rng().gen_range(0, self.working_loc.size.y)
                
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
            lat,
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
        // This strange scoping makes sure we can add to the number filled links at the bottom.
        // Have to un-borrow the lat ref which is borrowed by the Z3String.
        {
            let mut z3string = Z3String {
                start_loc: self.working_loc.location,
                cur_loc: self.working_loc,
                lat
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
            //println!("total_link_number_change {:?}", total_link_number_change);
            self.link_number_change = total_link_number_change;
        };

        lat.number_filled_links += self.link_number_change;
        assert!(lat.number_filled_links >= 0);
    }

    /// Accept or reject an update based on the number of links and the size of the lattice.
    pub fn accept_or_reject_update(&mut self, lattice_size: Point, number_filled_links: i64) -> AcceptReject{
        assert!(lattice_size.x >= 0);
        assert!(lattice_size.y >= 0);
        assert!(number_filled_links >= 0);

        let total_possible: u64 = (lattice_size.x * lattice_size.y * 2) as u64;
        // normalization factor for the weights.
        let mut normalization_factor: f64 = 0.0;
        for i in 0..total_possible {
            normalization_factor += f64::powf(self.link_number_tuning, i as f64);
        }
        let check_against = f64::powf(self.link_number_tuning, number_filled_links as f64);
        let mut rng = thread_rng();
        let rand_number: f64 = rng.gen_range(0.0, normalization_factor);

        //println!("number_filled_links {:?}", number_filled_links);
        //println!("normalization_factor {:?}", normalization_factor);
        //println!("rng {:?}", rng);
        //println!("rand_number {:?}", rand_number);
        //println!("check_against {:?}", check_against);
        //println!("self.link_number_tuning {:?}", self.link_number_tuning);
        //println!("AcceptReject {:?}", AcceptReject::Reject);
        if rand_number < check_against{
            return AcceptReject::Accept
        }
        return  AcceptReject::Reject
    }

    /// Organizes the calling of the update functions while taking care of high level
    /// accept reject decisions.
    pub fn main_update(&mut self, lat: &mut Lattice, update_type: &UpdateType) {

        // Save the original configuration of the lattice because moves might end up being
        // rejected.
        let original_lat: Lattice = lat.clone();

        match update_type {
            UpdateType::Local => self.update(lat),
            UpdateType::Walk => self.random_walk_update(lat)
        };

        // How many links on the new configuration.
        let new_number_links: i64 = lat.number_filled_links;
        // How many links on the old configuration.
        //let old_number_links: u64 = original_lat.number_filled_links;

        // Determine accept or reject. This function will return AcceptReject enum
        match self.accept_or_reject_update(lat.size, new_number_links) {
            AcceptReject::Reject => {*lat = original_lat},
            AcceptReject::Accept => {},
        };
    }
}

pub struct Z3String<'a> {
    /// This borrows a mutable reference to a lattice so it is assumed that an instance of this
    /// struct will be used to perform some operation on the lattice and then go out of scope so
    /// the mutable reference can be available again.
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