use crate::logger;
use regex::Regex;
use roxmltree::Document;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use roxmltree::Node;

pub fn get_text(text: &str, properties_hash: &HashMap<String, String>) -> String {
    let rex = Regex::new(r"\$[a-zA-Z0-9\{\}_]*").unwrap();
    let mut return_text = text.to_owned();
    for grp in rex.find_iter(text) {
        let key = grp.as_str().replace("${", "").replace("}", "");
        return_text = return_text.replace(
            grp.as_str(),
            properties_hash
                .get(key.as_str())
                .unwrap_or(&String::from(grp.as_str())),
        );
    }
    return_text
}

pub fn get_xml_from_file(path: PathBuf, log: &logger::Logger) -> String {
    let mut file = match fs::File::open(path) {
        Ok(content) => content,
        Err(e) => {
            log.build_failed(format!("ERROR: {:?}", e));
            std::process::exit(0);
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn get_xml_doc<F>(path: PathBuf, log: &logger::Logger, mut on_success: F) -> ()
where
    F: FnMut(Document) -> (),
{
    let xml_text: String = get_xml_from_file(path, log);
    let doc = match roxmltree::Document::parse(&xml_text) {
        Ok(doc) => doc,
        Err(e) => {
            log.build_failed(format!("ERROR: {:?}", e));
            std::process::exit(0);
        }
    };
    on_success(doc)
}

pub fn get_properties(
    &project: &Node,
    properties_hash: &mut HashMap<String, String>,
    log: &logger::Logger,
) {
    let properties_sheet: Vec<Node> = project
        .children()
        .filter(|n| n.is_element() && n.has_tag_name("loadproperty"))
        .collect();
    if properties_sheet.len() > 0 {
        for sheet in properties_sheet {
            let sheet_name = sheet.attribute("file").unwrap_or("");
            if sheet_name == "" {
                log.build_failed(String::from("Load Property file name cannot be empty"));
                std::process::exit(0);
            }
            let path = std::path::Path::new(properties_hash.get("__project__basedir").unwrap())
                .join(sheet_name);
            get_xml_doc(path, &log, |sheet: roxmltree::Document| {
                get_properties(&sheet.root_element(), properties_hash, log);
            });
        }
    }
    let properties: Vec<Node> = project
        .children()
        .filter(|n| n.is_element() && n.has_tag_name("property"))
        .collect();
    for property in properties {
        match property.attribute("type").unwrap_or("") {
            "file" => {
                let filename = property.attribute("value").unwrap_or("");
                let file_path =
                    std::path::Path::new(properties_hash.get("__project__basedir").unwrap())
                        .join(filename.clone());
                let contents = match std::fs::read_to_string(file_path) {
                    Ok(f) => f,
                    Err(_e) => {
                        log.build_failed(format!("{} not found", filename));
                        std::process::exit(0);
                    }
                };
                properties_hash.insert(
                    String::from(property.attribute("name").unwrap_or("")),
                    contents,
                );
            }
            _ => {
                properties_hash.insert(
                    String::from(property.attribute("name").unwrap_or("")),
                    String::from(property.attribute("value").unwrap_or("")),
                );
            }
        }
    }
}