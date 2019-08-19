
use crate::logger;
use crate::utils;

use std::error::Error;
use std::io::prelude::*;

pub fn create_task<'a, 'b>(
    element: &roxmltree::Node<'a, 'b>,
    log: &logger::Logger,
    properties_hash: &std::collections::HashMap<String, String>,
) {
    let path_text = utils::get_text(element.attribute("path").unwrap_or(""), &properties_hash);
    let path = std::path::Path::new(properties_hash.get("__project__basedir").unwrap())
        .join(path_text.clone());
    match element.attribute("type").unwrap_or("dir") {
        "dir" => match std::fs::create_dir_all(path) {
            Ok(_) => {
                println!("\t[perform: create] Created directory {:?}", path_text);
            }
            Err(e) => {
                log.build_failed(String::from(e.description()));
                std::process::exit(0);
            }
        },
        "file" => {
            let mut file = match std::fs::File::create(path) {
                Ok(f) => {
                    println!("\t[perform: create] Created file {}", path_text);
                    f
                }
                Err(e) => {
                    log.build_failed(String::from(e.description()));
                    std::process::exit(0);
                }
            };
            if element.has_children() {
                let text = utils::get_text(element.text().unwrap_or("").trim(), &properties_hash);
                match file.write_all(text.as_bytes()) {
                    Ok(_) => {}
                    Err(e) => {
                        log.build_failed(String::from(e.description()));
                        std::process::exit(0);
                    }
                };
            }
        }
        _ => {}
    };
}