use super::super::datamodel::Point;


pub struct ClusterSizeEstimatorDisplay {
    tmp: i8
}

//#[derive(Debug)]
pub struct ClusterSizeEstimator{
    cluster_sizes: Vec<i64>,
    cluster_covered_points: Vec<Point>
}

impl Iterator for ClusterSizeEstimator {
    type Item = ClusterSizeEstimatorDisplay;

    fn next(&mut self) -> Option<ClusterSizeEstimatorDisplay>{
        let x = 2;
        return Some(ClusterSizeEstimatorDisplay {tmp: 18})
    }
}