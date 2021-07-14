use std::path::Path;


fn main () {
    if !Path::new("./static/NotoSans-Regular.ttf").exists() {
        // We will use rosoto to grab this file from s3 if it doesn't exist.
        panic!("Need a font for the GUI. Build script will eventually be responsible for collecting if it DNE");
    }
}