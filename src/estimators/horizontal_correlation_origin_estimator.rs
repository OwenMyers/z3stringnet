use super::Measureable;
use super::super::datamodel::VertexLinkCount

/// Measures the string correlation function from the horizontal link
/// at the origin in the out and in direcction (from origin vertex) 
/// seperatly.
/// 
/// This is really two correlation functions set up as one for 
/// efficiency
#[derive(Debug)]
pub struct HorizontalCorrelationOriginEstimator {
    // Keep track of a link if it is in the same direction of
    // the origin link (1 if out and origin link is out)
    cur_binary_out_correlation: Vec<VertexLinkCount>,
    cur_binary_in_correlation: Vec<VertexLinkCount>,
}

impl HorizontalCorrelationOriginEstimator {

    pub fn new() -> HorizontalCorrelationFunctionOrigin {
        println!("Initilizing HorizontalCorrelationOriginEstimator");

        println!("Opening horizontal corrilation estimator file");
        let path = Path::new("")
    }
}