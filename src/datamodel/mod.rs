enum Link {
    In,
    Out,
    Blank,
}

struct Vertex {
    n: Link,
    e: Link,
    s: Link,
    w: Link,
    xy: Point,
}

struct Point {
    x: u64,
    y: u64,
}

struct Lattice {
    // All links can be defined by the vertices of one sublattice.
    // This means the len of vertices will always be N/2, where N is the
    // total number of vertices.
    // TODO: Do a check or asertation to ensure the length of vertices
    // is correct given Point.
    vertices: Vec<Vertex>,
    size: Point,
}

fn build_blank_lat(size: Point) -> Lattice {
    println!("Building blank lattice of size x {}, y {}",
             size.x, size.y);

    let mut lat: Lattice = Lattice {
        vertices: Vec::new(),
        size,
    };

    let half_N = (lat.size.x * lat.size.y)/2;

    // Only need half of N because we only need vertices from one sub
    // lattice to compleatly define all links.
    println!("Filling vertex array:");
    for i in (0..half_N) {
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
fn x_from_vertex_vec_position(position: u64, size: &Point) -> u64 {
    let y = position * 2 % size.y;
    let x = position * 2 % size.x;
    if y % 2 == 0 {
        return x
    }
    else {
        return x + 1;
    }
}
fn y_from_vertex_vec_position(position: u64, size: &Point) -> u64 {
    let y = position * 2 % size.y;
    y
}
