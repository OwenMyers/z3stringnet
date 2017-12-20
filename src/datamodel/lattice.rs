use super::*;

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
            // TODO: Or I think it is fine if this is never implemented and the
            // case of implied sublattice points is handled else where like in the 
            // string operator.
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