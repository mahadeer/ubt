use crate::logger;
use regex::Regex;
use roxmltree::Document;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

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