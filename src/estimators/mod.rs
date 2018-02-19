pub mod density_estimator;
use super::datamodel::lattice::Lattice;

pub trait Measureable {
    fn measure(&mut self, lat: &Lattice);
    /// Devide the counts by the number of measurements
    /// per bin.
    fn divide_by(&mut self, );
