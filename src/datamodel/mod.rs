use rand;
use rand::Rng;
pub mod lattice;
pub mod cluster;
use self::lattice::x_from_vertex_vec_position;
use self::lattice::y_from_vertex_vec_position;
use std::ops::Add;
use std::slice::Iter;


//#[derive(Debug, Clone, Copy, Eq, PartialEq)]
///// An AbsolutePlaquett is a plaquett view of the surounding links where the links are specified
///// using the absolute reference of the axes. The vertex objexts have links with directions
///// that are specified relative to the vertex, ie "In", "Out". An "Absolute" object will specify
///// link values with respect to the axis.
/////
///// A plaquett view:
///// ```
///// -----
///// | + |
///// -----
///// ```
///// `|` and `--` denote the horizontal and vertical links respectively.
///// The `+` marks the center of the plaquett
//#[derive(Debug, Clone, Copy)]
//pub enum AbsolutePlaquett {
//
//}

//#[derive(Debug, Clone, Copy)]
//pub enum AbsoluteLink {
//    PlussOne,
//    MinusOne,
//    Blank
//}

//#[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Link {
    In,
    Out,
    Blank,
}
impl Link {
    pub fn flip(&self) -> Link {
        match *self{
            Link::In => {Link::Out}
            Link::Out => {Link::In}
            Link::Blank => {Link::Blank}
        }
    }
    pub fn soft_flip(link: &Link) -> Link {
        match link{
            Link::In => {Link::Out}
            Link::Out => {Link::In}
            Link::Blank => {Link::Blank}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction;  4] = [
            Direction::N, Direction::E, Direction::S, Direction::W
        ];
        DIRECTIONS.into_iter()
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
    
    pub fn clear(&mut self) {
        self.n = 0;
        self.e = 0;
        self.s = 0;
        self.w = 0;
    }
}

/// A `Lattice` is built exclusively with these objects each containing `Links` that
/// touch the vertex.
/// 
/// ```ignore
///   |
/// --+--
///   |
/// ```
/// `|` and `--` denote the horizontal and vertical links respectively.
/// 
/// `Vertex.xy` is a `Point` specifying the position of the vertex.
#[derive(Clone, Debug, Copy)]
pub struct Vertex {
    pub n: Link,
    pub e: Link,
    pub s: Link,
    pub w: Link,
    pub xy: Point,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}
impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point{x, y}
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
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
            // Be careful here: % is nod modulus but the remainder -> can be negative.
            // This looks strange because the extra stuff will insure that we get
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
