use super::Link;
use super::Point;
use super::BoundPoint;
use super::lattice::Lattice;
use super::Direction;
use super::Vertex;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_location() {
        let direction = Direction::E;
        let bound_point = BoundPoint{
            size: Point{x:4, y: 4},
            location: Point{x: 1, y: 2}
        };
        let post_increment_bound_point: BoundPoint;
        post_increment_bound_point = increment_location(bound_point, &direction);
        let should_be = Point{x: 2, y: 2};
        assert_eq!(post_increment_bound_point, should_be);
    }
    #[test]
    fn test_decrement_location() {
        let direction = Direction::E;
        let bound_point = BoundPoint{
            size: Point{x:4, y: 4},
            location: Point{x: 1, y: 2}
        };
        let post_decrement_bound_point: BoundPoint;
        post_decrement_bound_point = decrement_location(bound_point, &direction);
        let should_be = Point{x: 0, y: 2};
        assert_eq!(post_decrement_bound_point, should_be);
    }
    #[test]
    fn test_add_to_direction_vec_if_filled_if_filled() {
        let direction = Direction::N;
        let link = Link::In;
        let mut direction_vec = Vec::new();
        add_to_direction_vec_if_filled(&mut direction_vec, &direction, &link);
        assert_eq!(vec![Direction::N], direction_vec);
    }
    #[test]
    fn test_add_to_direction_vec_if_filled_if_empty() {
        let direction = Direction::N;
        let link = Link::Blank;
        let mut direction_vec = Vec::new();
        add_to_direction_vec_if_filled(&mut direction_vec, &direction, &link);
        assert_eq!(direction_vec.len(), 0);
    }
    #[test]
    fn test_directions_of_filled_links() {
        let test_vertex = Vertex{
            n: Link::In,
            e: Link::Blank,
            s: Link::Out,
            w: Link::Blank,
            xy: Point{x: 1, y: 1}
        };
        let dir_vec_option: Option<Vec<Direction>> = directions_of_filled_links(&test_vertex);
        match dir_vec_option {
            // See implementaiton of directions_of_filled_links() for order but it will be 
            // N E S W, removing blanks
            Some(dir_vec) => assert_eq!(vec![Direction::N, Direction::S], dir_vec),
            None => panic!("In test_directions_of_filled_links we recived None.")
        };
    }
    #[test]
    fn test_directions_of_filled_links_when_empty() {
        let test_vertex = Vertex{
            n: Link::Blank,
            e: Link::Blank,
            s: Link::Blank,
            w: Link::Blank,
            xy: Point{x: 1, y: 1}
        };
        let dir_vec_option: Option<Vec<Direction>> = directions_of_filled_links(&test_vertex);
        let post_match = match dir_vec_option {
            // See implementaiton of directions_of_filled_links() for order but it will be 
            // N E S W, removing blanks
            Some(dir_vec) => panic!("In test_directions_of_filled_links we recived Some."),
            None => (),
        };
        assert_eq!(post_match, ());
    }
}

pub fn increment_location(location: BoundPoint, direction: &Direction) -> BoundPoint {
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
    let post_increment_bound_point: BoundPoint;
    match increment {
        Some(inc) => post_increment_bound_point = &location + inc,
        None => panic!("No step taken for some reason. No increment."),
    }
    post_increment_bound_point
}

/// Reverse the input direction and increment the bound point in that
/// reversed direction.
pub fn decrement_location(location: BoundPoint, direction: &Direction) -> BoundPoint {
    let flipped_direction = direction.flip();
    let post_decrement_bound_point: BoundPoint = increment_location(location, &flipped_direction);
    post_decrement_bound_point
}

pub fn add_to_direction_vec_if_filled(
    keep_vec: &mut Vec<Direction>, direction: &Direction, link: &Link
) {
    match *link {
        Link::In => (*keep_vec).push(direction.clone()),
        Link::Out => (*keep_vec).push(direction.clone()),
        Link::Blank => ()
    }
}

/// Return a vector of directions where each direction
/// corresponds to the non empty links of a vertex.
/// Return is done with Option. Pattern match to get the actual vector
/// and if no non-empty links are found then return None.
pub fn directions_of_filled_links(vertex: &Vertex) -> Option<Vec<Direction>> {

    let mut non_empty_links = Vec::new();
    add_to_direction_vec_if_filled(&mut non_empty_links, &Direction::N, &vertex.n);
    add_to_direction_vec_if_filled(&mut non_empty_links, &Direction::E, &vertex.e);
    add_to_direction_vec_if_filled(&mut non_empty_links, &Direction::S, &vertex.s);
    add_to_direction_vec_if_filled(&mut non_empty_links, &Direction::W, &vertex.w);
    if non_empty_links.len() > 0 {
        Some(non_empty_links)
    }
    else {
        None
    }
}
