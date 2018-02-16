extern crate rand;
pub mod datamodel;
pub mod lattice_updates;
pub mod estimators;
pub mod oio;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

