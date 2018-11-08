pub mod density_estimator;
pub mod correlation_origin_estimator;
pub mod total_link_count_estimator;
pub mod winding_number_estimator;
use super::datamodel::lattice::Lattice;
use std::io::BufWriter;
use std::fs::File;
use super::datamodel::VertexLinkCount;
use std::io::prelude::*;

    
/// Write what should be the header for all 
/// estimator files.
pub fn write_standard_header(writer: &mut BufWriter<File>) {
    let mut out_string = String::new();
    out_string.push_str("x,y,N,E,S,W\n");
    match writer.write(out_string.as_bytes()){
        Err(_) => panic!("Can not write estimator's header"),
        Ok(_) => println!("Wrote header to estimator buffer."),
    }
}



pub trait Measurable {
    fn measure(&mut self, lat: &Lattice);
    /// Divide the counts by the number of measurements
    /// per bin and write the file.
    fn finalize_bin_and_write(&mut self, denominator: u64);
    /// Clear out counts before taking a series of measurements to 
    /// be bined.
    fn clear(&mut self);

    fn line_out_string_from_vertex_link_count(vertex: &VertexLinkCount,
                                                  denominator: &f64) -> String {
                                                  
        let formatted_line = format!("{},{},{},{},{},{}\n",
                vertex.xy.x,
                vertex.xy.y,
                (vertex.n as f64) / denominator,
                (vertex.e as f64) / denominator,
                (vertex.s as f64) / denominator,
                (vertex.w as f64) / denominator,
                );
    
        formatted_line
    }

}
