extern crate fs_extra;
extern crate glob;
extern crate regex;
extern crate roxmltree;

mod builder;
mod logger;
mod tasks;
mod utils;

use std::env;

fn main() {
    let build_path: &str = &env::args().nth(1).unwrap_or("".to_string());
    let log = logger::Logger::new();
    if build_path == "" {
        log.log(format!("Error: Build xml path must be provided"));
    } else {
        log.log(format!("\nBuildfile: {:?} \n", build_path));
        utils::get_xml_doc(
            std::path::Path::new(build_path).to_path_buf(),
            &log,
            |doc: roxmltree::Document| {
                builder::build_project(&doc.root_element(), &log);
            },
        );
        log.build_sucessful();
    }
}
