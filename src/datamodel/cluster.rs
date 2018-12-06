
use super::Point;
use super::BoundPoint;
use super::lattice::Lattice;
use super::Direction;
use super::Vertex;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_int() -> u8 {
        let test_int: u8 = 8;
        test_int
    }

    #[test]
    fn test_outside_var() {
        let test_int = get_test_int();
        assert_eq!(8, test_int);
    }

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

///// Return a vector of directions where each direction
///// corresponds to the non empty links of a vertex.
///// Return is done with option. Pattern match to get the actual vector
///// and if no non-empty links are found then return None.
//pub fn directions_of_filled_links(vertex: &Vertex) -> Option<Vec<Direction>> {
//
//    non_empty_links = Vec::new();
//    for direction in Direction::iterator(){
//        non_empty_links.push(direction)
//    }
//    if non_empty_links.len() > 0 {
//        Some(non_empty_links)
//    }
//    else {
//        None
//    }
//
//}

//pub fn watch_cluster(lat: &Lattice) {
//    // loop over the points in the lattice
//    // Keep track of the points we have already visited.
//    let points_visited = Vec::new();
//    let mut keep_going= true;
//    let mut completed_vertices = HashMap::new();
//    for vertex in lat.vertices{
//
//        let mut working_point: Point = vertex.xy.clone();
//        let mut working_bound_point = {
//            size: lat.size,
//            location: working_point
//        }
//        let mut filled_directions_vec: Vec<Direction>
//            = match directions_of_filled_links(&vertex) {
//                None => continue,
//                Some(filled_vec) => filled_vec
//            };
//        let completed_bool
//            = match completed_vertices.get(&working_loc) {
//                None => (),
//                Some(completed_bool) => completed_bool
//            };
//        if completed_bool {continue};
//
//        keep_going = true;
//        let stack: Vec<(Point, Vec<Direction>)> = vec![(
//            working_loc.clone(),
//            filled_directions_vec.clone()
//        )];
//        while keep_going {
//
//            (working_loc, filled_directions_vec) = stack.pop();
//
//            step_in_direction = filled_directions_vec.pop();
//            working_loc = increment_location(working_loc, &step_in_direction);
//
//            // Because we are looping over vertices on only one sublattice then we just
//            // need to make sure the vertex has not been clustered or looked at yet
//            // Check if the vertex has links
//            filled_directions_vec = match directions_of_filled_links(&vertex) {
//                Some(cur_filled) => cur_filled,
//                None => {
//                    // SOMETHING SPECIAL HAPPENS HERE. NEED TO POP
//                    println!("SOMETHING SPECIAL HAPPENS HERE. NEED TO POP");
//                }
//            };
//            stack.push(
//                (working_loc.clone(), filled_directions_vec.clone())
//            );
//
//            // If they have not been clustered and there are links,
//            cur_direction_option = filled_directions_vec.pop();
//            match cur_direction {
//                Some(cur_direction) => (),
//            }
//        }
//    }
//}

// Because Point object implements Clone trait we can pass it in and it will
// be cloned. Ownership will not be taken
