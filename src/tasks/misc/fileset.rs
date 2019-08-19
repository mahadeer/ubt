
use glob::glob;
use roxmltree::Node;
use std::path::PathBuf;
pub fn get_files(fileset: &Vec<Node>, base_path: &String) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = vec![];
    for entry in glob(&format!("{}/**/*.*", base_path)).unwrap() {
        match entry {
            Ok(path) => {
                files.push(path);
            }
            Err(e) => {
                println!("{:?}", e);
                std::process::exit(0);
            }
        }
    }
    // Excludes all the mentioned path from list of files
    let excludes_set: Vec<&Node> = fileset
        .iter()
        .filter(|f| f.attribute("type").unwrap_or("") == "excludes")
        .collect::<Vec<_>>();
    for exclude in excludes_set {
        for entry in glob(&format!(
            "{}/{}",
            base_path,
            exclude.attribute("path").unwrap_or("")
        ))
        .unwrap()
        {
            match entry {
                Ok(path) => {
                    if let Some(index) = files.iter().position(|x| *x == path) {
                        files.remove(index);
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                    std::process::exit(0);
                }
            }
        }
    }
    // Includes all the mentioned path from list of files
    let includes_set: Vec<&Node> = fileset
        .iter()
        .filter(|f| f.attribute("type").unwrap_or("") == "includes")
        .collect::<Vec<_>>();
    for exclude in includes_set {
        for entry in glob(&format!(
            "{}/{}",
            base_path,
            exclude.attribute("path").unwrap_or("")
        ))
        .unwrap()
        {
            match entry {
                Ok(path) => {
                    if let Some(_index) = files.iter().position(|x| *x == path) {
                    } else {
                        files.push(path);
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                    std::process::exit(0);
                }
            }
        }
    }
    files
}