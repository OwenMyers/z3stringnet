use super::Measurable;
use super::super::datamodel::VertexLinkCount;
use super::super::datamodel::lattice::Lattice;
use super::super::datamodel::Link;
use super::super::datamodel::Vertex;
use super::write_standard_header;
use super::super::datamodel::Point;
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::error::Error;

fn simple_file_make_helper_function(direction_string: &str,
                                    orientation_string: &str) -> BufWriter<File> {
    println!("Opening {orientation} {direction} corrilation estimator file",
                orientation=orientation_string,
                direction=direction_string);
    let file_name_string = format!("{orientation}_correlation_origin_{direction}_estimator.csv",
                                    orientation=orientation_string,
                                    direction=direction_string);
    let path = Path::new(&file_name_string);
    let display = path.display();
    let file = match File::create(&path){
        Err(err) => panic!("could not create {}: {}", display, err.description()),
        Ok(good_file) => good_file,
    };
    return BufWriter::new(file);
}

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
    vector_size: u64,
}

impl CorrelationOriginEstimator {

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
        for i in 0..correlation_origin_estimator.vector_size {
            let cur_vertex_link_count = VertexLinkCount::new(i as i64, size);
            correlation_origin_estimator.cur_binary_horizontal_out_correlation.push(cur_vertex_link_count);
            // cur_vertex_link_count was consumed so make another for out count
            let cur_vertex_link_count = VertexLinkCount::new(i as i64, size);
            correlation_origin_estimator.cur_binary_horizontal_in_correlation.push(cur_vertex_link_count);
            let cur_vertex_link_count = VertexLinkCount::new(i as i64, size);
            correlation_origin_estimator.cur_binary_vertical_in_correlation.push(cur_vertex_link_count);
            let cur_vertex_link_count = VertexLinkCount::new(i as i64, size);
            correlation_origin_estimator.cur_binary_vertical_out_correlation.push(cur_vertex_link_count);
        }

        write_standard_header(
            &mut correlation_origin_estimator.result_file_buffer_horizontal_in);
        write_standard_header(
            &mut correlation_origin_estimator.result_file_buffer_horizontal_out);
        write_standard_header(
            &mut correlation_origin_estimator.result_file_buffer_vertical_out);
        write_standard_header(
            &mut correlation_origin_estimator.result_file_buffer_vertical_out);

        println!("Done initilizing origin correlation estimator.");

        correlation_origin_estimator

    }
}

impl Measurable for CorrelationOriginEstimator {
    fn clear(&mut self) {
        for i in 0..self.vector_size {
            let cur_index = i as usize;
            self.cur_binary_horizontal_out_correlation[cur_index].clear();
            self.cur_binary_horizontal_in_correlation[cur_index].clear();
            self.cur_binary_vertical_out_correlation[cur_index].clear();
            self.cur_binary_vertical_in_correlation[cur_index].clear();
        }
    }

    /// Some additional arithmitic and write to buffer.
    ///
    /// Devide all of the counts by `denominator`, which is the 
    /// number of measurments per bin, and by N where N is the
    /// number of verticies. Write the result.
    fn finalize_bin_and_write(&mut self, denominator: u64) {

        // You can get N with `2*vector_size`
        let float_denominator = (denominator as f64) * ((self.vector_size*2) as f64);
        let mut ho_out_string = String::new();
        let mut hi_out_string = String::new();
        let mut vo_out_string = String::new();
        let mut vi_out_string = String::new();

        for i in 0..self.vector_size {
            let cur_index = i as usize;

            let ho_formatted_line = Self::line_out_string_from_vertex_link_count(
                &self.cur_binary_horizontal_out_correlation[cur_index], 
                &float_denominator
            );
            let hi_formatted_line = Self::line_out_string_from_vertex_link_count(
                &self.cur_binary_horizontal_in_correlation[cur_index], 
                &float_denominator
            );
            let vo_formatted_line = Self::line_out_string_from_vertex_link_count(
                &self.cur_binary_vertical_out_correlation[cur_index], 
                &float_denominator
            );
            let vi_formatted_line = Self::line_out_string_from_vertex_link_count(
                &self.cur_binary_vertical_in_correlation[cur_index], 
                &float_denominator
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
        // First check each of the origin links.
        let origin_vertex: &Vertex = &lat.vertices[0];
        let origin_horizontal_link: &Link = &origin_vertex.e;
        let origin_vertical_link: &Link = &origin_vertex.n;

        let mut measure_horizontal = true;
        let mut measure_vertical = true;
        match *origin_horizontal_link {
            Link::Blank => {
                measure_horizontal = false;
            },
            _ => () 
        };
        match *origin_vertical_link {
            Link::Blank => {
                measure_vertical = false;
            },
            _ => ()
        };
        
        // We can avoid looping over vertices if both the horizontal and 
        // vertical links are `Blank`.
        if (!measure_horizontal) && (!measure_vertical) {
            return;
        }

        for (i, cur_vertex) in lat.vertices.iter().enumerate(){
            if measure_horizontal {
                match *origin_horizontal_link {
                    Link::In => {
                        match cur_vertex.e {
                            Link::In => {
                                self.cur_binary_horizontal_in_correlation[i].e += 1;
                            },
                            _ => (),
                        }
                        match cur_vertex.w {
                            Link::Out => {
                                self.cur_binary_horizontal_in_correlation[i].w += 1;
                            },
                            _ => (),
                        }
                    },
                    Link::Out => {
                        match cur_vertex.e {
                            Link::Out => {
                                self.cur_binary_horizontal_out_correlation[i].e += 1;
                            },
                            _ => (),
                        }
                        match cur_vertex.w {
                            Link::In => {
                                self.cur_binary_horizontal_out_correlation[i].w += 1;
                            },
                            _ => (),
                        }
                    },
                    Link::Blank => ()
                }
            }

            if measure_vertical {
                match *origin_vertical_link {
                    Link::In => {
                        match cur_vertex.n {
                            Link::In => {
                                self.cur_binary_vertical_in_correlation[i].n += 1;
                            },
                            _ => (),
                        }
                        match cur_vertex.s {
                            Link::Out => {
                                self.cur_binary_vertical_in_correlation[i].s += 1;
                            },
                            _ => (),
                        }
                    },
                    Link::Out => {
                        match cur_vertex.n {
                            Link::Out => {
                                self.cur_binary_vertical_out_correlation[i].n += 1;
                            },
                            _ => (),
                        }
                        match cur_vertex.s {
                            Link::In => {
                                self.cur_binary_vertical_out_correlation[i].s += 1;
                            },
                            _ => (),
                        }
                    },
                    Link::Blank => ()
                }
            }
        }
    }
}
