use super::Direction;
use super::Link;
use super::Point;
use super::Vertex;

/// Stores the representation of the sytem
///
///
/// All links can be defined by the vertices of one sublattice.
/// This means the len of vertices will always be N/2, where N is the
/// total number of vertices.
/// TODO: Do a check or asertation to ensure the length of vertices
/// is correct given Point.
///
///     |   |   |   |
///     +---6---+---7---
///     |   |   |   |
///     4---+---5---+---
///     |   |   |   |
///     +---2---+---3---
///     |   |   |   |
///     0---+---1---+---
#[derive(Clone, Debug)]
pub struct Lattice {
    pub vertices: Vec<Vertex>,
    pub size: Point,
    pub number_filled_links: i64,
}
impl Lattice {
    // Only storing one sublattice so other vertices are implied.
    // Lets call the ones in our `vertices` vector "real" and the
    // implied ones "fake".
    pub fn point_real(&self, p: &Point) -> bool {
        assert!((p.x >= 0) && (p.y >= 0), "Function point_real requires positive x and y");
        ((p.x + p.y) % 2) == 0
    }
    pub fn get_link_from_point(&mut self, loc: &Point, direction: &Direction) -> &mut Link{
        // See if this point is on the sublattice of the stored vertices.
        // Only storing one sublattice so other vertices are implied.
        // Lets call the ones in our `vertices` vector "real" and the
        // implied ones "fake".
        let is_real = self.point_real(&loc);
        // The location of vertex in the vector. This works becuase integers division rounds down.
        let vloc = loc.y * (self.size.x/2) + loc.x/2;
        //println!("vector location: {}",vloc);
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
            panic!(format!("Cant handle implied vertices yet. Not sure if we need to. \
                This functionality may never exist. The location is x: {} y: {} and the \
                direction is {:?}", loc.x, loc.y, *direction));
        }
    }
    pub fn safe_get_link_from_point(&self, loc: &Point, direction: &Direction) -> &Link{
        // See if this point is on the sublattice of the stored vertices.
        // Only storing one sublattice so other vertices are implied.
        // Lets call the ones in our `vertices` vector "real" and the
        // implied ones "fake".
        let is_real = self.point_real(&loc);
        // The location of vertex in the vector. This works becuase integers division rounds down.
        let vloc = loc.y * (self.size.x/2) + loc.x/2;
        //println!("vector location: {}",vloc);
        if is_real {
            match *direction {
                Direction::N => return &(&self.vertices[vloc as usize]).n,
                Direction::E => return &(&self.vertices[vloc as usize]).e,
                Direction::S => return &(&self.vertices[vloc as usize]).s,
                Direction::W => return &(&self.vertices[vloc as usize]).w,
            }
        } 
        else {
            // The edges are the tough part to handle
            // TODO: Or I think it is fine if this is never implemented and the
            // case of implied sublattice points is handled else where like in the 
            // string operator.
            panic!(format!("Cant handle implied vertices yet. Not sure if we need to. \
                This functionality may never exist. The location is x: {} y: {} and the \
                direction is {:?}", loc.x, loc.y, *direction));
        }
    }
    pub fn out_raise_link(&mut self, loc: &Point, direction: &Direction) -> Link {
        //println!("in lat out raise, loc: {:?}",loc);
        //println!("in lat out raise, dir: {:?}",direction);
        // Raies a link traveling outward from the specified vertex
        // This function, because of get_link_from_point(), will only will only work
        // on real verticies. Thats the way we want it
        let link: &mut Link = self.get_link_from_point(loc, direction);
        match *link {
            Link::In    => {*link = Link::Blank;
                            Link::Blank},
            Link::Out   => {*link = Link::In;
                            Link::In},
            Link::Blank => {*link = Link::Out;
                            Link::Out},
        }
    }
    pub fn out_lower_link(&mut self, loc: &Point, direction: &Direction) -> Link{
        // Lower a link traveling outward from the specified vertex. Also see raise 
        // description.
        //println!("in out_lower_link. ---> location is: {:?}", loc);
        //println!("in out_lower_link. ---> directio is: {:?}", direction);
        let link: &mut Link = self.get_link_from_point(loc, direction);
        match *link {
            Link::In    => {*link = Link::Out;
                            Link::Out},
            Link::Out   => {*link = Link::Blank;
                            Link::Blank},
            Link::Blank => {*link = Link::In;
                            Link::In},
        }
    }

    pub fn count_non_blank_links(&mut self) -> u64{
        let mut count: u64 = 0;
        for (_, cur_vertex) in self.vertices.iter().enumerate(){
            match cur_vertex.n {
                Link::In  => {count += 1},
                Link::Out => {count += 1},
                Link::Blank => (),
            }
            match cur_vertex.e {
                Link::In  => {count += 1},
                Link::Out => {count += 1},
                Link::Blank => (),
            }
            match cur_vertex.s {
                Link::In  => {count += 1},
                Link::Out => {count += 1},
                Link::Blank => (),
            }
            match cur_vertex.w {
                Link::In  => {count += 1},
                Link::Out => {count += 1},
                Link::Blank => (),
            }
        }
        count
    }
}


pub fn build_blank_lat(size: Point) -> Lattice {
    println!("Building blank lattice of size x {}, y {}",
             size.x, size.y);

    let mut lat: Lattice = Lattice {
        vertices: Vec::new(),
        size,
        number_filled_links: 0
    };

    let half_n = (lat.size.x * lat.size.y)/2;

    // Only need half of N because we only need vertices from one sub
    // lattice to compleatly define all links.
    println!("Filling vertex array:");
    for i in 0..half_n {
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

///
///     |   |   |   |
///     +->-6->-+->-7->-
///     |   |   |   |
///     4->-+->-5->-+->-
///     |   |   |   |
///     +->-2->-+->-3->-
///     |   |   |   |
///     0->-+->-1->-+->-
///
pub fn build_z3_striped_lat(size: Point) -> Lattice {
    println!("Building stagard lattice of size x {}, y {}",
             size.x, size.y);

    let mut lat: Lattice = Lattice {
        vertices: Vec::new(),
        size,
        number_filled_links: (size.y / 2 * size.x) as i64
    };

    let half_n = (lat.size.x * lat.size.y)/2;

    // Only need half of N because we only need vertices from one sub
    // lattice to compleatly define all links.
    println!("Filling vertex array:");
    for i in 0..half_n {
        let cur_vertex: Vertex = Vertex{
            n: Link::Blank,
            e: Link::Out,
            s: Link::Blank,
            w: Link::In,
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
pub fn x_from_vertex_vec_position(position: i64, size: &Point) -> i64 {
    assert!(position >= 0, "No negative numbers may be passed into x_from_vertex_vec_position");
    
    let y = y_from_vertex_vec_position(position, &size);
    //println!("y is: {}", y);
    //println!("size.x is: {}", size.x);

    if y % 2 == 0 {
        return (position * 2) % size.x;
    }
    else {
        return (position * 2 + 1) % size.x;
    }
}

pub fn y_from_vertex_vec_position(position: i64, size: &Point) -> i64 {
    assert!(position >= 0, "No negative numbers may be passed into x_from_vertex_vec_position");

    if position > (size.x*size.y)/2{
        panic!("The position specified is greater
               than the number of unique vetices in the Lattice");
    }
    let y = position * 2 / size.x;
    y
}
