use std::io::BufWriter;
use std::path::Path;
use std::fs::File;
use super::Measurable;
use super::super::datamodel::Point;
use super::super::datamodel::Direction;
use super::super::datamodel::Vertex;
use super::super::datamodel::BoundPoint;
use super::super::datamodel::lattice::Lattice;
use super::super::datamodel::cluster::directions_of_filled_links;
use super::super::datamodel::cluster::decrement_location;
use super::super::datamodel::cluster::increment_location;
use super::super::datamodel::lattice::build_blank_lat;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cluster_size_estimator_constructor() {
        let mut lat: Lattice = build_blank_lat(Point{x: 4, y: 4});
        let cluster_size_est = ClusterSizeEstimator::new(&lat);
        assert_eq!(cluster_size_est.current_location.location.x, 0);
        assert_eq!(cluster_size_est.current_location.location.y, 0);
    }
}


pub struct ClusterSizeEstimatorDisplay {
    pub local_text: String,
    pub tmp: i8,
    pub cluster_size_est_current: ClusterSizeEstimator
}

#[derive(Debug, Clone)]
pub struct ClusterSizeEstimator{
    result_file_buffer: BufWriter<File>,
    pub cluster_sizes: Vec<i64>,
    pub clustered: HashMap<BoundPoint, u64>,
    pub cluster_covered_points: Vec<BoundPoint>,
    // general stack to keep track of directions not gone in
    pub stack: Vec<Vec<Direction>>,
    // initialize vector for direction path "walk list"
    pub walk_list: Vec<Direction>,
    pub current_location: BoundPoint,
    pub available_cluster_num: u64,
    pub is_initialized: bool,
    pub starting_location: BoundPoint,
    lat: Lattice
}

impl Measurable for ClusterSizeEstimator {
    fn measure(&mut self, lat: &Lattice) {}
    fn finalize_bin_and_write(&mut self, size: u64) {}
    fn clear(&mut self) {}
}

impl Iterator for ClusterSizeEstimator {
    type Item = ClusterSizeEstimatorDisplay;

    /// I think the HashMap `clustered` will only ever contain points that are clustered. Mostly
    /// we will just be checking to see if a key exists but I think it is nice to have the option
    /// of having a key that specifically denotes a point is not clustered. Until this
    /// algorithm is complete I won't really know. Just making that note in case there ends up
    /// being a much better way to handle that in the future.
    ///
    /// # Important attributes:
    /// * `vertex` -
    /// * `lat_size` -
    /// * `clustered` -
    /// * `available_cluster_num` - Every time we successfully find a cluster we need to give it
    ///   a unique name. That identifier will just be an incrementing integer. We need to keep track
    ///   of the value for adding new clusters to the `HashMap` `clustered`.
    ///   The "available" cluster number passed in is a cluster number that has "NOT BEEN USED YET".
    ///   It will become the cluster number of the cluster we are trying to cluster in this function.
    fn next(&mut self) -> Option<ClusterSizeEstimatorDisplay>{
        let mut local_text = "Running Cluster Estimator";
        // pop off vec off stack
        let mut filled_directions: Vec<Direction> = match self.stack.pop() {
            Some(to_return_directions) => to_return_directions,
            None => panic!("Stack should not be empty right after len > 0 check.")
        };
        // if vec empty:
        if filled_directions.len() == 0 {
            let reverse_step_dir: Direction = match self.walk_list.pop() {
                Some(to_return_direction) => to_return_direction,
                None => {
                    if self.stack.len() != 0 {
                        panic!("If the stack is not empty neither should the walk list be empty")
                    }
                    else {
                        self.is_initialized = false;
                        return Some(
                            ClusterSizeEstimatorDisplay {
                                local_text: "Completed sizing of this cluster!".to_string(),
                                tmp: 17,
                                cluster_size_est_current: self.clone()
                            }
                        )
                    }
                }
            };
            //  -> reverse step direction (change current location)
            //  This function handles flipping the direction to reverse the step.
            self.current_location = decrement_location(self.current_location, &reverse_step_dir);
            local_text = "Hit a reverse condition.\nNo directions, or at start loc.\nto Backing up\nExpect \
                no visualization of available directions.";
        }
        else {
            // pop direction off vec
            let direction: Direction = match filled_directions.pop() {
                Some(to_return_direction) => to_return_direction,
                None => panic!("Filled directions should be full")
            };
            // push modified vec to stack even if empty
            self.stack.push(filled_directions);
            // step direction
            self.current_location = increment_location(self.current_location, &direction);
            // push direction to walk list
            self.walk_list.push(direction);
            // check if vertex belongs to other cluster
            // if yes:
            if self.clustered.contains_key(&self.current_location) {
                let cur_loc_cluster_num: u64 = match self.clustered.get(&self.current_location) {
                    Some(to_return_cluster_num) => *to_return_cluster_num,
                    None => panic!("There should be something because we just checked contains_key.")
                };
                // if it belong to the current cluster:
                //    That's good. Time to start backtracking
                //    pop direction from walk list and
                //    -> reverse step direction (change current location)
                if cur_loc_cluster_num == self.available_cluster_num {
                    let last_direction: Direction = match self.walk_list.pop() {
                        Some(to_return_direction) => to_return_direction,
                        None => panic!("Walk list should not be empty at this point.")
                    };
                    self.current_location = decrement_location(self.current_location, &last_direction);
                    local_text = "Hit a reverse condition.\nFound vertex already part of a cluster";
                }
                // else if not the current cluster but part of a cluster
                //    panic because you did something wrong
                else {
                    panic!("Something has gone horribly wrong in the clustering algorithm. A link
                           has been found that belongs to a different cluster. This should not
                           be possible.");
                }
            }
            // -> else:
            //   mark new vertex as this cluster
            //   call direction_of_filled_links
            //   if not none: add to stack
            //   if none: panic
            else {
                self.clustered.insert(self.current_location, self.available_cluster_num);
                match directions_of_filled_links(&self.lat.get_vertex_from_point(&self.current_location)) {
                    Some(to_return_directions) => self.stack.push(to_return_directions),
                    None => panic!("If we moved in this direction we expect there to be at least
                                   two filled links at this vertex.")
                };
                local_text = "Found un-marked vertex."
            }
        }
        return Some(
            ClusterSizeEstimatorDisplay {
                local_text: local_text.to_string(),
                tmp: 18,
                cluster_size_est_current: self.clone()
            }
        )
    }
}

impl ClusterSizeEstimator {
    /// Set the working location, push that vertex to the stack, add it to the covered points
    pub fn init_calculation_location(&mut self, point: Point, lat: &mut Lattice) {
        self.current_location = BoundPoint {
            size: lat.size.clone(),
            location: point
        };
        self.clustered.insert(self.current_location, self.available_cluster_num);
        self.starting_location = BoundPoint {
            size: lat.size.clone(),
            location: point
        };
        let vertex = lat.get_vertex_from_point(&self.current_location);
        let vertex_available: Vec<Direction> = match directions_of_filled_links(&vertex) {
            Some(to_return_directions) => to_return_directions,
            None => Vec::new()
        };
        self.stack.push(vertex_available);
        self.cluster_covered_points.push(self.current_location);
        self.is_initialized = true;
    }
    pub fn new(lat: &Lattice) -> ClusterSizeEstimator {
        println!("Opening WindingNumberCountEstimator file");
        let path = Path::new("winding_number_count_estimator.csv");
        let display = path.display();
        let file = match File::create(&path) {
            Err(err) => panic!("could not create {}: {}",
                               display,
                               err),
            Ok(good_file) => good_file,
        };
        let result_file_buffer = BufWriter::new(file);
        ClusterSizeEstimator{
            result_file_buffer,
            cluster_sizes: Vec::new(),
            clustered: Default::default(),
            cluster_covered_points: Vec::new(),
            // general stack to keep track of directions not gone in
            stack: Vec::new(),
            // initialize vector for direction path "walk list"
            walk_list: Vec::new(),
            current_location: BoundPoint{
                size: lat.size.clone(),
                location: Point{x: 0, y: 0}
            },
            available_cluster_num: 0,
            is_initialized: false,
            starting_location: BoundPoint {
                size: lat.size.clone(),
                location: Point{x: 0, y: 0}
            },
            lat: lat.clone()
        }
    }
}
