use rand;
use rand::Rng;
pub mod lattice;
use self::lattice::Lattice;
use self::lattice::x_from_vertex_vec_position;
use self::lattice::y_from_vertex_vec_position;
use std::ops::Add;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;

#[derive(Debug)]
pub enum Link {
    In,
    Out,
    Blank,
}

#[derive(Debug)]
pub enum Direction {
    N,
    E,
    S,
    W,
}
impl Direction {
    pub fn flip(&self) -> Direction {
        match *self{
            Direction::N => { Direction::S }
            Direction::E => { Direction::W }
            Direction::S => { Direction::N }
            Direction::W => { Direction::E }
        }
    }
    pub fn get_random_direction() -> Direction {
        let direction_int = rand::thread_rng().gen_range(0, 4);
        assert!((direction_int < 4) && (direction_int >= 0));
        match direction_int {
            0 => { Direction::N }
            1 => { Direction::E }
            2 => { Direction::S }
            3 => { Direction::W }
            _ => panic!("Not a valid random integer for random direction.")
        }
    }
}

#[derive(Debug)]
pub struct VertexLinkCount {
    pub n: u64,
    pub e: u64,
    pub s: u64,
    pub w: u64,
    pub xy: Point,
}
impl VertexLinkCount {
    // pass in the "vec_position" which is the position of the "real"
    // vertex in the 1D vector storing all of the real vertices.
    pub fn new(vec_position: i64, size: &Point) -> VertexLinkCount {
        VertexLinkCount {
            n: 0, e: 0, s: 0, w: 0,
            xy: Point{
                        x: x_from_vertex_vec_position(vec_position, size),
                        y: y_from_vertex_vec_position(vec_position, size)
                     }
 
        }
    }
}

#[derive(Debug)]
pub struct DensityEstimator {
    cur_link_in_count: Vec<VertexLinkCount>,
    cur_link_out_count: Vec<VertexLinkCount>,
    cur_total_count: Vec<VertexLinkCount>,
}
impl DensityEstimator{
    // We are just going to count "in" and "out" for each link of
    // the real vertices.

    pub fn count_in_out(&mut self, lat: &Lattice){
        // for each direction add to the cur_in_count, cur_out_count
        // vectors if you find those directions.
        // loop over real vertices
        for (i, cur_vertex) in lat.vertices.iter().enumerate(){
            match cur_vertex.n {
                Link::In  => {
                    self.cur_link_in_count[i].n += 1;
                    self.cur_total_count[i].n += 1;
                },
                Link::Out => {
                    self.cur_link_out_count[i].n += 1;
                    self.cur_total_count[i].n += 1;
                },
                Link::Blank => (),
            }
            match cur_vertex.e {
                Link::In  => {
                    self.cur_link_in_count[i].e += 1;
                    self.cur_total_count[i].n += 1;
                },
                Link::Out => {
                    self.cur_link_out_count[i].e += 1;
                    self.cur_total_count[i].n += 1;
                },
                Link::Blank => (),
            }
            match cur_vertex.s {
                Link::In  => {
                    self.cur_link_in_count[i].s += 1;
                    self.cur_total_count[i].n += 1;
                },
                Link::Out => {
                    self.cur_link_out_count[i].s += 1;
                    self.cur_total_count[i].n += 1;
                },
                Link::Blank => (),
            }
            match cur_vertex.w {
                Link::In  => {
                    self.cur_link_in_count[i].w += 1;
                    self.cur_total_count[i].n += 1;
                },
                Link::Out => {
                    self.cur_link_out_count[i].w += 1;
                    self.cur_total_count[i].n += 1;
                },
                Link::Blank => (),
            }
        }
    }
    // static "constructor" method.
    pub fn new(size: &Point) -> DensityEstimator{
        println!("Initilizing DensityEstimator"); 
        
        let mut density_estimator = DensityEstimator{
            cur_link_in_count: Vec::new(),
            cur_link_out_count: Vec::new(),
            cur_total_count: Vec::new(),
        };

        let half_n = (size.x * size.y)/2; 
        for i in 0..half_n {
            let cur_vertex_link_count = VertexLinkCount::new(i, size);
            density_estimator.cur_link_in_count.push(cur_vertex_link_count);
            // cur_vertex_link_count was consumed so make another for out count
            let cur_vertex_link_count = VertexLinkCount::new(i, size);
            density_estimator.cur_link_out_count.push(cur_vertex_link_count);
        }

        println!("Done initilizing density estimator.");
        return density_estimator
    }
    pub fn write_total_count(&self, f_str: String) {
        println!("Writing density estimator total count");
        let path = Path::new(&f_str);
        let display = path.display();

        let mut file = match File::create(&path){
            Err(err) => panic!("could not create {}: {}",
                            display,
                            err.description()),
            Ok(good_file) => good_file,
        };
        
        let mut out_string = String::new();
        out_string.push_str("x,y,N,E,S,W\n");

        for vertex in &self.cur_total_count{
            out_string.push_str(
                    &format!("{},{},{},{},{},{}\n",
                            vertex.xy.x,
                            vertex.xy.y,
                            &vertex.n,
                            &vertex.e,
                            &vertex.s,
                            &vertex.w,
                            )
                    );
        }
        out_string.push_str("\n");
        println!("{}", out_string);

        match file.write_all(out_string.as_bytes()){
            Err(err) => panic!("could not create {}: {}",
                            display,
                            err.description()),
            Ok(_) => println!("file out worked"),
        }
    }
}

pub struct Vertex {
    pub n: Link,
    pub e: Link,
    pub s: Link,
    pub w: Link,
    pub xy: Point,
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Copy, Clone)]
pub struct BoundPoint {
    pub size: Point, 
    pub location: Point,
}
impl<'a> Add <Point> for &'a BoundPoint{
    // overload + here to make it modulus `size`
    type Output = BoundPoint;
    fn add(self, input: Point) -> BoundPoint {
        let new_x = self.location.x + input.x;
        let new_y = self.location.y + input.y;
        BoundPoint {
            size: Point {
                x: self.size.x,
                y: self.size.y,
            },
            // Be carful here: % is nod modulus but the remainder -> can be negative.
            // This looks strange becase the extra stuff will insure that we get 
            // the modulus.
            location: Point {
                x: ((new_x % self.size.x) + self.size.x) % self.size.x,
                y: ((new_y % self.size.y) + self.size.y) % self.size.y,
            }
        }
    }
}
impl PartialEq <Point> for BoundPoint{
    fn eq(&self, rhs: &Point) -> bool {
        (self.location.x == rhs.x) && (self.location.y == rhs.y)
    }
}
// Confused: I thought I had to have this so #derive does not have to be used. If #derive is
// used, all fields must be equal, not just a subset
//impl Eq <Point> for BoundPoint {}
// PartialEq seams to compile just fine the way it is.


#[derive(Debug)]
pub struct Update {
    pub working_loc: BoundPoint,
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
        println!("cur location before {:?}",self.cur_loc.location);
        println!("cur direction before {:?}",direction);
        // This function takes a step along a path from the self.cur_loc position to a new
        // position determined by the input from the user. It onle steps across one link and it
        // CHANGES that link acording to the raising and lowing rules given the orientation of the
        // link and the direction of the step.
        
        // If the lnik is a real one (point is of the stored sub lattice) then you just need
        // to call lat `get_link_from_point` and operate on that link
        if self.lat.point_real(&self.cur_loc.location){
            println!("real point");
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
            println!("not real point");
            println!("orig dir {:?}", direction);
            self.increment_cur_loc(&direction);
            let fliped_dir = direction.flip();
            println!("new dir {:?}", fliped_dir);
            self.lat.out_lower_link(&self.cur_loc.location, &fliped_dir);
        }
        println!("cur location after {:?}",self.cur_loc.location);
    }
}



