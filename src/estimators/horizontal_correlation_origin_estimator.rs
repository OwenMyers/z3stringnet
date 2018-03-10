use super::Measureable;
use super::super::datamodel::VertexLinkCount

/// Measures the string correlation function from the horizontal 
/// and vertical links at the origin seperatly, 
/// in the out and in direcction (from origin vertex) seperatly
/// as well
/// 
/// This is really 4 correlation functions set up as one.  
#[derive(Debug)]
pub struct CorrelationOriginEstimator {
    // Keep track of a link if it is in the same direction of
    // the origin link (1 if out and origin link is out)
    cur_binary_horizontal_out_correlation: Vec<VertexLinkCount>,
    cur_binary_horizontal_in_correlation: Vec<VertexLinkCount>,
    cur_binary_vertical_out_correlation: Vec<VertexLinkCount>,
    cur_binary_vertical_in_correlation: Vec<VertexLinkCount>,

    result_file_buffer_horizontal_out: BufWriter<File>,
    result_file_buffer_horizontal_in: BufWriter<File>,
    result_file_buffer_vertical_out: BufWriter<File>,
    result_file_buffer_vertical_in: BufWriter<File>,
}

impl CorrelationOriginEstimator {

    fn simple_file_make_helper_function(direction_string: &str,
                                        orientation_string: &str) -> BufWriter<File> {
        println!("Opening {orientation} {direction} corrilation estimator file",
                    orientation=orientation_string,
                    direction=direction_string);
        let file_name_string = format!("{orientation}_correlation_origin_{direction}_estimator.csv",
                                        orientation=orientation_string,
                                        direction=direction_string);
        let path = Path::new(file_name_string);
        let display = path.display();
        let file = match File::create(&path){
            Err(err) => panic!("could not create {}: {}", display, err.description()),
            Ok(good_file) => good_file,
        };
        return BufWriter::new(file);

    }

    pub fn new() -> HorizontalCorrelationFunctionOrigin {
        println!("Initilizing HorizontalCorrelationOriginEstimator");
        let result_file_buffer_horizontal_out = 
            simple_file_make_helper_function("out", "horizontal");
        let result_file_buffer_horizontal_in = 
            simple_file_make_helper_function("in", "horizontal");
        let result_file_buffer_vertical_out = 
            simple_file_make_helper_function("out", "vertical");
        let result_file_buffer_vertical_in = 
            simple_file_make_helper_function("in", "vertical");

        let mut correlation_origin_estimator = CorrelationOriginEstimator {
            cur_binary_horizontal_out_correlation = Vec::new(),
            cur_binary_horizontal_in_correlation = Vec::new(),
            cur_binary_vertical_out_correlation = Vec::new(),
            cur_binary_vertical_in_correlation = Vec::new(),
            result_file_buffer_horizontal_out = result_file_buffer_horizontal_out,
            result_file_buffer_horizontal_in = result_file_buffer_horizontal_in,
            result_file_buffer_vertical_out = result_file_buffer_vertical_out,
            result_file_buffer_vertical_in = result_file_buffer_vertical_in,
        };
    }
}