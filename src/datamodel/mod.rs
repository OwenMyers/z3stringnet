
use rand;
use rand::Rng;
pub mod lattice;
use self::lattice::Lattice;

#[derive(Debug)]
pub enum Link {
    In,
    Out,
    Blank,
}
pub enum Direction {
    N,
    E,
    S,
    W,
}

pub struct Vertex {
    pub n: Link,
    pub e: Link,
    pub s: Link,
    pub w: Link,
    pub xy: Point,
}

#[derive(Debug)]
pub struct Point {
    pub x: u64,
    pub y: u64,
}


#[derive(Debug)]
pub struct Update {
    pub lat_size: Point,
    pub working_loc: Point,
}
impl Update {
    pub fn get_rand_point(&mut self) {
        self.working_loc = Point {
            x: rand::thread_rng()
                .gen_range(0, self.lat_size.x),
            y: rand::thread_rng()
                .gen_range(0, self.lat_size.y)
                
        };
    }
    pub fn update(&mut self, lat: &mut Lattice) {
        // Get a random point.
        // Lets say the random point is the lower left
        // corner of the plaquette.
        // Clockwise walk.
        self.get_rand_point();
        // TODO
    }
}

//pub struct Z3String {
//    /// This borrows a mutable refference to a lattice so it is assumed that an instance of this
//    /// struct will be used to perform some operation on the lattice and then go out of scope so
//    /// the mutable reference can be avaliabel again.
//    pub start_loc: Point,
//    pub cur_loc: Point,
//    pub path: Vec<Point>,
//    lat: &mut Lattice, 
//}
//impl Z3String {
//    pub fn step(&self, direction: Direction) {
//        /// This function takes a step along a path from the self.cur_loc position to a new
//        /// position determined by the input from the user. It onle steps across one link and it
//        /// CHANGES that link acording to the raising and lowing rules given the orientation of the
//        /// link and the direction of the step.
//        
//        // If the lnik is a real one (point is of the stored sub lattice) then you just need
//        // to call lat `get_link_from_point` and operate on that link
//        if *lat.point_real(cur_loc){
//            match direction {
//                Direction::N => out_raise(cur_loc),
//                Direction::E => out_raise(cur_loc),
//                Direction::S => out_raise(cur_loc),
//                Direction::W => out_raise(cur_loc),
//            }
//            advance cur_loc
//        }
//        // If the link is not real then step in `direction`, which will guarentee you are now on
//        // the reall sublattice, and look back accross the link from the new vertex. You have to
//        // be careful operating on the link because it will be in the reverse direcction now that
//        // you have already taken the step.
//        else {
//            advance cur_loc
//            match direction {
//                Direction::N => in_raise(cur_loc),
//                Direction::E => in_raise(cur_loc),
//                Direction::S => in_raise(cur_loc),
//                Direction::W => in_raise(cur_loc),
//            }
//        }
//    }
//}



