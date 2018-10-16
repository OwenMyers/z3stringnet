
use super::datamodel::Point;
use lattice::Lattice;
use Direction;
use Vertex;

fn increment_location(location: &mut Point, direction: &Direction) {
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
        Some(inc) => location = &mut location + inc,
        None => panic!("No step taken for some reason. No increment."),
    }
    location
}

/// Return a vector of directions where each direction
/// corresponds to the non empty links of a vertex.
/// Return is done with option. Pattern match to get the actual vector
/// and if no non-empty links are found then return None.
pub fn directions_of_filled_links(vertex: &Vertex) -> Option<Vec<Direction>> {

    non_empty_links = Vec::new();
    for direction in Direction::iterator(){
        non_empty_links.push(direction)
    }
    if non_empty_links.len() > 0 {
        Some(non_empty_links)
    }
    else {
        None
    }

}

pub fn watch_cluster(lat: &Lattice) {
    // loop over the points in the lattice
    // Keep track of the points we have already visited.
    let points_visited = Vec::new();
    let mut keep_goingj= true;
    for vertex in lat.vertices{

        let mut working_loc: Point = vertex.xy.clone();
        let mut filled_directions_vec: Vec<Direction>
            = match directions_of_filled_links(&vertex) {
                None => continue,
                Some(filled_vec) => filled_vec
            }

        keep_going = true;
        let stack: Vec<(Point, Vec<Direction>)> = vec![(
            working_loc.clone(),
            filled_directions_vec.clone()
        )];
        while keep_going {

            step_in_direction = filled_directions_vec.pop();
            working_loc = increment_location(working_loc, &step_in_direction);

            // Because we are looping over vertices on only one sublattice then we just
            // need to make sure the vertex has not been clustered or looked at yet
            // Check if the vertex has links
            filled_directions_vec = match directions_of_filled_links(&vertex) {
                Some(cur_filled) => cur_filled,
                None => SOMETHING SPECIAL HAPPENS HERE. NEED TO POP;
            };
            stack.push(
                (working_loc.clone(), filled_directions_vec.clone())
            );

            // If they have not been clustered and there are links,
            cur_direction_option = filled_directions_vec.pop();
            match cur_direction {
                Some(cur_direction) => _,
            }
        }
    }
}
        let stack: Vec<(Point, Vec<Direction>)> = vec![(
            working_loc,
            filled_directions_vec
        )];

