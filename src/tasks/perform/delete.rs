
use crate::logger;
use crate::tasks;
use crate::utils;
use roxmltree::Node;
use std::error::Error;

pub fn create_task<'a, 'b>(
    element: &Node<'a, 'b>,
    log: &logger::Logger,
    properties_hash: &std::collections::HashMap<String, String>,
) {
    let path_text = utils::get_text(element.attribute("path").unwrap_or(""), &properties_hash);
    let path = std::path::Path::new(properties_hash.get("__project__basedir").unwrap())
        .join(path_text.clone());
    match element.attribute("type").unwrap_or("dir") {
        "dir" => {
            match std::fs::remove_dir_all(path) {
                Ok(_) => {
                    println!("\t[perform: delete] Deleted directory {:?}", path_text);
                }
                Err(e) => {
                    log.build_failed(String::from(e.description()));
                    std::process::exit(0);
                }
            };
        }
        "file" => {
            match std::fs::remove_file(path) {
                Ok(_) => {
                    println!("\t[perform: delete] Deleted file {}", path_text);
                }
                Err(_e) => {
                    log.build_failed(String::from("Cannot delete a file"));
                    std::process::exit(0);
                }
            };
        }
        "fileset" => {
            let fileset: Vec<Node> = element
                .children()
                .filter(|n| n.is_element() && n.has_tag_name("fileset"))
                .collect();
            if fileset.len() > 0 {
                // This will be treated as removing files
                let path_text = format!(
                    "{}/{}",
                    properties_hash.get("__project__basedir").unwrap(),
                    path_text.clone()
                );
                let files_to_delete = tasks::misc::fileset::get_files(&fileset, &path_text);
                for file in files_to_delete {
                    // let file_path = file.display().to_string();
                    match std::fs::remove_file(file) {
                        Ok(_) => {
                            //println!("\t[perform: delete] Deleted file {}", file_path);
                        }
                        Err(_e) => {
                            log.build_failed(String::from("Cannot delete a file"));
                            std::process::exit(0);
                        }
                    }
                }
            }
        }
        _ => {}
    };
}