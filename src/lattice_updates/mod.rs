use super::datamodel::BoundPoint;
use super::datamodel::Point;
use super::datamodel::Direction;
use super::datamodel::lattice::Lattice;
use rand;
use rand::Rng;

/// This will ergodicly update the Z3 string net model.
/// 
/// A couple of choices are avaliable. 
/// * The `update` function will perform an update on random plaquets 
///   walking clockwise with the raising operator. 
/// * The `random_walk_update` will perform a random walk to produce an
///   extensive change to the configuration it operates on.
#[derive(Debug)]
pub struct Update {
    pub working_loc: BoundPoint,
    pub link_number_tuning: f64,
}
impl Update {
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
    
        let cur_direction = Direction::N;
        z3string.raise_step(&cur_direction);

        let cur_direction = Direction::E;
        z3string.raise_step(&cur_direction);

        let cur_direction = Direction::S;
        z3string.raise_step(&cur_direction);

        let cur_direction = Direction::W;
        z3string.raise_step(&cur_direction);
        
        assert!(z3string.cur_loc == z3string.start_loc);
    }
    pub fn random_walk_update(&mut self, lat: &mut Lattice) {
        self.get_rand_point();
        let mut z3string = Z3String{
            start_loc: self.working_loc.location,
            cur_loc: self.working_loc,
            lat: lat
        };
        
        // Take first step before loop so cur_loc and start_loc
        // are different.  
        let cur_direction = Direction::get_random_direction();
        z3string.raise_step(&cur_direction);

        while z3string.cur_loc != z3string.start_loc {
            let cur_direction = Direction::get_random_direction();
            z3string.raise_step(&cur_direction);
        }
        assert!(z3string.cur_loc == z3string.start_loc);
    }
}

pub struct Z3String<'a> {
    /// This borrows a mutable refference to a lattice so it is assumed that an instance of this
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
    pub fn raise_step(&mut self, direction: &Direction) {
        //println!("cur location before {:?}",self.cur_loc.location);
        //println!("cur direction before {:?}",direction);
        // This function takes a step along a path from the self.cur_loc position to a new
        // position determined by the input from the user. It onle steps across one link and it
        // CHANGES that link acording to the raising and lowing rules given the orientation of the
        // link and the direction of the step.
        
        // If the lnik is a real one (point is of the stored sub lattice) then you just need
        // to call lat `get_link_from_point` and operate on that link
        if self.lat.point_real(&self.cur_loc.location){
            //println!("real point");
            self.lat.out_raise_link(&self.cur_loc.location, &direction);
            self.increment_cur_loc(&direction);
        }
        // If the link is not real then step in `direction`, which will guarentee you are now on
        // the reall sublattice, and look back accross the link from the new vertex. You have to
        // be careful operating on the link because it will be in the reverse direcction now that
        // you have already taken the step.
        // "out lower" is would be the same as "in raise". Either works now that we have
        // shifted position without changing anything.
        else {
            //println!("not real point");
            //println!("orig dir {:?}", direction);
            self.increment_cur_loc(&direction);
            let fliped_dir = direction.flip();
            //println!("new dir {:?}", fliped_dir);
            self.lat.out_lower_link(&self.cur_loc.location, &fliped_dir);
        }
        //println!("cur location after {:?}",self.cur_loc.location);
    }
}