use crate::logger;

use crate::tasks;
use crate::utils;
use roxmltree::Node;
use std::collections::HashMap;


pub fn build_project(&project: &Node, log: &logger::Logger) {
    let default = project.attribute("default").unwrap_or("");
    if project.has_attribute("default") && default != "" {
        let targets: Vec<Node> = project
            .children()
            .filter(|n| n.is_element() && n.attribute("name").unwrap_or("") == default)
            .collect();
        let mut properties_hash = HashMap::new();
        properties_hash.insert(
            "__project__basedir".to_owned(),
            project.attribute("basedir").unwrap_or("").to_owned(),
        );
        get_properties(&project, &mut properties_hash, log);
        resolve_target(&project, &targets, &mut properties_hash.clone(), log);
    }
}

fn resolve_target(
    project: &Node,
    targets: &Vec<Node>,
    properties_hash: &mut HashMap<String, String>,
    log: &logger::Logger,
) {
    for target in targets {
        if target.has_attribute("depends") {
            let mut dependencies: Vec<&str> =
                target.attribute("depends").unwrap().rsplit(",").collect();
            dependencies.reverse();
            for dependency in dependencies {
                let dependency_targets: Vec<Node> = project
                    .children()
                    .filter(|n| n.is_element() && n.attribute("name").unwrap_or("") == dependency)
                    .collect();
                resolve_target(
                    &project,
                    &dependency_targets,
                    &mut properties_hash.clone(),
                    log,
                );
            }

        }
        build_target(&target, &mut properties_hash.clone(), log);
    }
}

fn build_target(
    &target: &Node,
    properties_hash: &mut HashMap<String, String>,
    log: &logger::Logger,
) {
    println!("{}:", target.attribute("name").unwrap());
    get_properties(&target, properties_hash, log);
    let elements: Vec<Node> = target.children().filter(|n| n.is_element()).collect();
    for element in elements {
        match element.tag_name().name() {
            "echo" => {
                println!(
                    "\t[echo] {}",
                    utils::get_text(element.text().unwrap_or(""), &properties_hash)
                );
            }
            "perform" => {
                let task_fn: fn(
                    &roxmltree::Node,
                    &logger::Logger,
                    &std::collections::HashMap<String, String>,
                ) = match element.attribute("operation").unwrap_or("") {
                    "create" => tasks::perform::create::create_task,
                    "delete" => tasks::perform::delete::create_task,
                    "copy" => tasks::perform::copy::create_task,
                    "move" => tasks::perform::moving::create_task,
                    _ => {
                        std::process::exit(0);
                    }
                };
                task_fn(&element, &log, &properties_hash);
            }
            "exec" => {
                tasks::exec::create_task(&element, &log, &properties_hash);
            }
            _ => {}
        }
    }
}

fn get_properties(
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
            utils::get_xml_doc(path, &log, |sheet: roxmltree::Document| {
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
