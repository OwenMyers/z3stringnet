use rand;
use rand::Rng;

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

pub struct Z3String {
    /// This borrows a mutable refference to a lattice so it is assumed that an instance of this
    /// struct will be used to perform some operation on the lattice and then go out of scope so
    /// the mutable reference can be avaliabel again.
    pub start_loc: Point,
    pub cur_loc: Point,
    pub path: Vec<Point>,
    lat: &mut Lattice, 
}
impl Z3String {
    pub fn step(&self, direction: Direction) {
        /// This function takes a step along a path from the self.cur_loc position to a new
        /// position determined by the input from the user. It onle steps across one link and it
        /// CHANGES that link acording to the raising and lowing rules given the orientation of the
        /// link and the direction of the step.
        
        // If the lnik is a real one (point is of the stored sub lattice) then you just need
        // to call lat `get_link_from_point` and operate on that link
        if *lat.point_real(cur_loc){
            match direction {
                Direction::N => out_raise(cur_loc),
                Direction::E => out_raise(cur_loc),
                Direction::S => out_raise(cur_loc),
                Direction::W => out_raise(cur_loc),
            }
            advance cur_loc
        }
        // If the link is not real then step in `direction`, which will guarentee you are now on
        // the reall sublattice, and look back accross the link from the new vertex. You have to
        // be careful operating on the link because it will be in the reverse direcction now that
        // you have already taken the step.
        else {
            advance cur_loc
            match direction {
                Direction::N => in_raise(cur_loc),
                Direction::E => in_raise(cur_loc),
                Direction::S => in_raise(cur_loc),
                Direction::W => in_raise(cur_loc),
            }
        }
    }
}

pub struct Lattice {
    // All links can be defined by the vertices of one sublattice.
    // This means the len of vertices will always be N/2, where N is the
    // total number of vertices.
    // TODO: Do a check or asertation to ensure the length of vertices
    // is correct given Point. 
    pub vertices: Vec<Vertex>,
    pub size: Point,
}
impl Lattice {
    // Only storing one sublattice so other verticies are implied.
    // Lets call the ones in our `vertices` vector "real" and the
    // implied ones "fake".
    pub fn point_real(&self, p: &Point) -> bool {
        ((p.x + p.y) % 2) == 0
    }
    pub fn get_link_from_point(&mut self, loc: &Point, direction: &Direction) -> &mut Link{
        // See if this point is on the sublattice of the stored verticies.
        // Only storing one sublattice so other verticies are implied.
        // Lets call the ones in our `vertices` vector "real" and the
        // implied ones "fake".
        let is_real = self.point_real(&loc);
        // The potential location of vertex in the vector
        let vloc = loc.x + loc.y;
        if is_real {
            match *direction {
                Direction::N => return &mut (&mut self.vertices[vloc as usize]).n,
                Direction::E => return &mut (&mut self.vertices[vloc as usize]).e,
                Direction::S => return &mut (&mut self.vertices[vloc as usize]).s,
                Direction::W => return &mut (&mut self.vertices[vloc as usize]).w,
            }
        } 
        else {
            // The edges are the tough part to handle
            // TODO
            panic!("Cant handle implied vertices yet. Not sure if we need to. This functionality may never exist.");
        }
    }
}


pub fn build_blank_lat(size: Point) -> Lattice {
    println!("Building blank lattice of size x {}, y {}",
             size.x, size.y);

    let mut lat: Lattice = Lattice {
        vertices: Vec::new(),
        size,
    };

    let half_n = (lat.size.x * lat.size.y)/2;

    // Only need half of N because we only need vertices from one sub
    // lattice to compleatly define all links.
    println!("Filling vertex array:");
    for i in 0..half_n {
        println!("i {}", i);
        let cur_vertex: Vertex = Vertex{
            n: Link::Blank,
            e: Link::Blank,
            s: Link::Blank,
            w: Link::Blank,
            xy: Point{
                x: x_from_vertex_vec_position(i, &lat.size),
                y: y_from_vertex_vec_position(i, &lat.size),
            }
        };
        lat.vertices.push(cur_vertex);
    }

    lat
}

//TODO: check these
pub fn x_from_vertex_vec_position(position: u64, size: &Point) -> u64 {
    
    let y = y_from_vertex_vec_position(position, &size);
    println!("y is: {}", y);
    println!("size.x is: {}", size.x);

    if y % 2 == 0 {
        return (position * 2) % size.x;
    }
    else {
        return (position * 2 + 1) % size.x;
    }
}

pub fn y_from_vertex_vec_position(position: u64, size: &Point) -> u64 {
    if position > (size.x*size.y)/2{
        panic!("The position specified is greater
               than the number of unique vetices in the Lattice");
    }
    let y = position * 2 / size.x;
    y
}

