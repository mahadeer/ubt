extern crate regex;
extern crate roxmltree;
extern crate fs_extra;
extern crate glob;

mod logger;
mod utils;
mod builder;
mod tasks;

use std::env;

fn main() {
    let build_path: &str = &env::args().nth(1).expect("Build path xml must be provided");
    let log = logger::Logger::new();
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
