use super::Measureable;
use super::super::datamodel::VertexLinkCount;
use super::write_standard_header;
use super::supper:datamodel::Point;
use super::line_out_string_from_vertex_link_count;

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

    vecotr_size: u64,
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

    pub fn new(size: &Point) -> CorrelationOriginEstimator {
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
            cur_binary_horizontal_out_correlation: Vec::new(),
            cur_binary_horizontal_in_correlation: Vec::new(),
            cur_binary_vertical_out_correlation: Vec::new(),
            cur_binary_vertical_in_correlation: Vec::new(),
            result_file_buffer_horizontal_out: result_file_buffer_horizontal_out,
            result_file_buffer_horizontal_in: result_file_buffer_horizontal_in,
            result_file_buffer_vertical_out: result_file_buffer_vertical_out,
            result_file_buffer_vertical_in: result_file_buffer_vertical_in,
            vector_size: 0,
        };
        correlation_origin_estimator.vector_size = ((size.x * size.y)/2) as u64; 

        write_standard_header(
            &correlation_origin_estimator.result_file_buffer_horizontal_in)
        write_standard_header(
            &correlation_origin_estimator.result_file_buffer_horizontal_out)
        write_standard_header(
            &correlation_origin_estimator.result_file_buffer_vertical_out)
        write_standard_header(
            &correlation_origin_estimator.result_file_buffer_vertical_out)

        println!("Done initilizing origin correlation estimator.")

    }

    impl Measureable for CorrelationOriginEstimator {
        fn clear(&mut self) {
            for i in 0..self.vecotr_size {
                let cur_index = i as usize;
                self.cur_binary_horizontal_out_correlation[cur_index].clear();
                self.cur_binary_horizontal_in_correlation[cur_index].clear();
                self.cur_binary_vertical_out_correlation[cur_index].clear();
                self.cur_binary_vertical_in_correlation[cur_index].clear();
            }
        }

        fn finalize_bin_and_write(&mut self, denominator: u64) {
            // Devide all of the counts by `denominator`, which is the 
            // number of measurments per bin, and write the result.
            let float_denominator = denominator as f64;
            let mut ho_out_string = String::new();
            let mut hi_out_string = String::new();
            let mut vo_out_string = String::new();
            let mut vi_out_string = String::new();

            for i in 0..self.vecotr_size {
                let cur_index = i as usize;

                let ho_formatted_line = line_out_string_from_vertex_link_count(
                    &self.cur_binary_horizontal_out_correlation[cur_index], 
                    float_denominator
                );
                let hi_formatted_line = line_out_string_from_vertex_link_count(
                    &self.cur_binary_horizontal_in_correlation[cur_index], 
                    float_denominator
                );
                let vo_formatted_line = line_out_string_from_vertex_link_count(
                    &self.cur_binary_vertical_out_correlation[cur_index], 
                    float_denominator
                );
                let vi_formatted_line = line_out_string_from_vertex_link_count(
                    &self.cur_binary_vertical_in_correlation[cur_index], 
                    float_denominator
                );

                ho_out_string.push_str(&ho_formatted_line);
                hi_out_string.push_str(&hi_formatted_line);
                vo_out_string.push_str(&vo_formatted_line);
                vi_out_string.push_str(&vi_formatted_line);

            }

            ho_out_string.push_str("\n");
            hi_out_string.push_str("\n");
            vo_out_string.push_str("\n");
            vi_out_string.push_str("\n");

            match self.result_file_buffer_horizontal_out
                    .write(ho_out_string.as_bytes()){
                Err(err) => panic!("Can't write to origin estimator buff: {}", err.description()),
                Ok(_) => println!("Wrote measurment to origin estimator buffer.") ,
            }
            match self.result_file_buffer_horizontal_in
                    .write(hi_out_string.as_bytes()){
                Err(err) => panic!("Can't write to origin estimator buff: {}", err.description()),
                Ok(_) => println!("Wrote measurment to origin estimator buffer.") ,
            }
            match self.result_file_buffer_vertical_out
                    .write(vo_out_string.as_bytes()){
                Err(err) => panic!("Can't write to origin estimator buff: {}", err.description()),
                Ok(_) => println!("Wrote measurment to origin estimator buffer.") ,
            }
            match self.result_file_buffer_vertical_in
                    .write(vi_out_string.as_bytes()){
                Err(err) => panic!("Can't write to origin estimator buff: {}", err.description()),
                Ok(_) => println!("Wrote measurment to origin estimator buffer.") ,
            }
        }

        fn measure(&mut self, lat: &Lattice) {

        }
    }
}