use rand;
use rand::Rng;
pub mod lattice;
use self::lattice::Lattice;
use std::ops::Add;

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
    pub x: i64,
    pub y: i64,
}

#[derive(Debug)]
pub struct BoundPoint {
    pub size: Point, 
    pub location: Point,
}
impl<'a> Add <Point> for &'a BoundPoint{
    // overload + here to make it modulus `size`
    type Output = BoundPoint;
    fn add(self, input: Point) -> BoundPoint {
        BoundPoint {
            size: Point {
                x: self.size.x,
                y: self.size.y,
            },
            location: Point {
                x: (self.location.x + input.x) % self.size.x,
                y: (self.location.y + input.y) % self.size.y,
            }
        }
    }
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

pub struct Z3String<'a> {
    /// This borrows a mutable refference to a lattice so it is assumed that an instance of this
    /// struct will be used to perform some operation on the lattice and then go out of scope so
    /// the mutable reference can be avaliabel again.
    pub start_loc: Point,
    pub cur_loc: BoundPoint,
    pub path: Vec<Point>,
    lat: &'a mut Lattice, 
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
        // This function takes a step along a path from the self.cur_loc position to a new
        // position determined by the input from the user. It onle steps across one link and it
        // CHANGES that link acording to the raising and lowing rules given the orientation of the
        // link and the direction of the step.
        
        // If the lnik is a real one (point is of the stored sub lattice) then you just need
        // to call lat `get_link_from_point` and operate on that link
        if self.lat.point_real(&self.cur_loc.location){
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
            self.increment_cur_loc(&direction);
            self.lat.out_lower_link(&self.cur_loc.location, &direction);
        }
    }
}



