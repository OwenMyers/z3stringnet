pub mod density_estimator;
use super::datamodel::lattice::Lattice;

pub trait Measureable {
    fn measure(&mut self, lat: &Lattice);
}