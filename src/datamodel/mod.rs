use rand;
use rand::Rng;

pub enum Link {
    In,
    Out,
    Blank,
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
    pub fn update(&self, lat: &mut Lattice) {
        // Get a random point.
        // Lets say the random point is the lower left
        // corner of the plaquette.
        // Clockwise walk.
        self.get_rand_point();
        // TODO
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
    pub fn point_real(p: &Point) -> bool {
        TODO
        (p.x + p.y) %
    }
    pub fn get_link_from_point(&mut self, loc: Point) -> &mut Link{
        // See if this point is on the sublattice of the stored verticies.
        // Only storing one sublattice so other verticies are implied.
        // Lets call the ones in our `vertices` vector "real" and the
        // implied ones "fake".
        let is_real = 
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

