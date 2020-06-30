extern crate rand;
#[macro_use]
extern crate conrod_core;
extern crate glium;
extern crate conrod_winit;
extern crate conrod_glium;
pub mod datamodel;
pub mod lattice_updates;
pub mod estimators;
pub mod oio;
pub mod gui;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

