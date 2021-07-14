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

pub struct RecursiveishClusterOutput {
    pub tmp: i8,
}

/// I think the HashMap `clustered` will only ever contain points that are clustered. Mostly
/// we will just be checking to see if a key exists but I think it is nice to have the option
/// of having a key that specifically denotes a point is not clustered. Until this
/// algorithm is complete I won't really know. Just making that note in case there ends up
/// being a much better way to handle that in the future.
///
/// # Arguments:
/// * `vertex` -
/// * `lat_size` -
/// * `clustered` -
/// * `available_cluster_num` - Every time we successfully find a cluster we need to give it
///   a unique name. That identifier will just be an incrementing integer. We need to keep track
///   of the value for adding new clusters to the `HashMap` `clustered`.
///   The "available" cluster number passed in is a cluster number that has "NOT BEEN USED YET".
///   It will become the cluster number of the cluster we are trying to cluster in this function.
pub fn recusiveish_cluster(vertex: &Vertex,
                           lat_size: &Point,
                           clustered: &mut HashMap<BoundPoint, u64>,
                           available_cluster_num: &u64,
                           lattice: &Lattice
                           ) -> Option<RecursiveishClusterOutput> {//-> Option<&mut HashMap<BoundPoint, u64>> {

    // general stack to keep track of directions not gone in
    let mut stack: Vec<Vec<Direction>> = Vec::new();
    // initialize vector for direction path "walk list"
    let mut walk_list: Vec<Direction> = Vec::new();
    let mut current_location: BoundPoint = BoundPoint {
        size: lat_size.clone(),
        location: vertex.xy.clone()
    };

    //let mut clustered: HashMap<BoundPoint, u64> = HashMap::new();
    let vertex_available: Vec<Direction> = match directions_of_filled_links(vertex) {
        Some(to_return_directions) => to_return_directions,
        None => return None
    };

    // If we got some directions add direction vec to stack
    stack.push(vertex_available);
    while stack.len() > 0 {
        // pop off vec off stack
        let mut filled_directions: Vec<Direction> = match stack.pop() {
            Some(to_return_directions) => to_return_directions,
            None => panic!("Stack should not be empty right after len > 0 check.")
        };
        // if vec empty:
        if filled_directions.len() == 0 {
            // pop direction from walk list
            let reverse_step_dir: Direction = match walk_list.pop() {
                Some(to_return_direction) => to_return_direction,
                None => panic!("If the stack is not empty neither should the walk list be empty")
            };
            //  -> reverse step direction (change current location)
            //  This function handles flipping the direction to reverse the step.
            current_location = decrement_location(current_location, &reverse_step_dir);
        }
        else {
            // pop direction off vec
            let direction: Direction = match filled_directions.pop() {
                Some(to_return_direction) => to_return_direction,
                None => panic!("Filled directions should be full")
            };
            // push modified vec to stack even if empty
            stack.push(filled_directions);
            // step direction
            current_location = increment_location(current_location, &direction);
            // push direction to walk list
            walk_list.push(direction);
            // check if vertex belongs to other cluster
            // if yes:
            if clustered.contains_key(&current_location) {
                let cur_loc_cluster_num: u64 = match clustered.get(&current_location) {
                    Some(to_return_cluster_num) => *to_return_cluster_num,
                    None => panic!("There should be something because we just checked contains_key.")
                };
                // if it belong to the current cluster:
                //    Thats good. Time to start backtracking
                //    pop direction from walk list and
                //    -> reverse step direction (change current location)
                if cur_loc_cluster_num == *available_cluster_num {
                    let last_direction: Direction = match walk_list.pop() {
                        Some(to_return_direction) => to_return_direction,
                        None => panic!("Walk list should not be empty at this point.")
                    };
                    current_location = decrement_location(current_location, &last_direction);
                }
                // else if not the current cluster but part of a cluster
                //    panic because you did something wrong
                else {
                    panic!("Something has gone horribly wrong in the clustering algorithm. A link
                           has been found that belongs to a different cluster. This should not
                           be possible.");
                }
            }
            // else:
            //   mark new vertex as this cluster
            //   call direction_of_filled_links
            //   if not none: add to stack
            //   if none: panic
            //else {
            //    clustered.insert(current_location, available_cluster_num);
            //    match directions_of_filled_links(TODO) {
            //        Some(to_return_directions) => stack.push(to_return_directions),
            //        None => panic!("If we moved in this direction we expect there to be at least
            //                       two filled links at this vertex.")
            //    };
            //}
        }
        // assert len stack == len walk list
    }
    //Some(clustered)
    Some(RecursiveishClusterOutput{ tmp: 2})
}

// gotten_map = map verticies to "gotten"
// Loop over all verticies in lattice
//   Check if vertex is "gotten" with gotten map.
//   If gotten:
//     continue
//   Else:
//     recursive search function on vertex (pass in gotten map)

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
