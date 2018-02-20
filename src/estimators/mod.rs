pub mod density_estimator;
use super::datamodel::lattice::Lattice;

pub trait Measureable {
    fn measure(&mut self, lat: &Lattice);
    /// Devide the counts by the number of measurements
    /// per bin and write the file.
    fn finalize_bin_and_write(&mut self, denominator: u64);
}
