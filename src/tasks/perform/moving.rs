
use crate::logger;
use crate::utils;
use roxmltree::Node;
use std::error::Error;

pub fn create_task<'a, 'b>(
    element: &Node<'a, 'b>,
    log: &logger::Logger,
    properties_hash: &std::collections::HashMap<String, String>,
) {
    let from_path_text = utils::get_text(element.attribute("from").unwrap_or(""), &properties_hash);
    let to_path_text = utils::get_text(element.attribute("to").unwrap_or(""), &properties_hash);
    let from_path = std::path::Path::new(properties_hash.get("__project__basedir").unwrap())
        .join(from_path_text.clone());
    let to_path = std::path::Path::new(properties_hash.get("__project__basedir").unwrap())
        .join(to_path_text.clone());
    match element.attribute("type").unwrap_or("dir") {
        "dir" => {
            let mut options = fs_extra::dir::CopyOptions::new();
            options.overwrite = match element.attribute("overwrite") {
                Some(t) => {
                    if t == "true" {
                        true
                    } else {
                        false
                    }
                }
                None => true,
            };
            match fs_extra::dir::move_dir(from_path, to_path, &options) {
                Ok(_) => {
                    println!(
                        "\t[perform: move] Moved directory {:?} to {:?}",
                        from_path_text, to_path_text
                    );
                }
                Err(e) => {
                    log.build_failed(String::from(e.description()));
                    std::process::exit(0);
                }
            };
        }
        "file" => {
            let mut options = fs_extra::file::CopyOptions::new();
            options.overwrite = match element.attribute("overwrite") {
                Some(t) => {
                    if t == "true" {
                        true
                    } else {
                        false
                    }
                }
                None => true,
            };
            match fs_extra::file::move_file(from_path, to_path, &options) {
                Ok(_) => {
                    println!(
                        "\t[perform: move] Moved file {:?} to {:?}",
                        from_path_text, to_path_text
                    );
                }
                Err(e) => {
                    log.build_failed(String::from(e.description()));
                    std::process::exit(0);
                }
            };
        }
        _ => {}
    };  
}