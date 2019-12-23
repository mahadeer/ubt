use crate::logger;

use crate::tasks;
use crate::utils;
use roxmltree::Node;
use std::collections::HashMap;
use std::io::Write;

pub fn build_project(&project: &Node, log: &logger::Logger) {
    let default = project.attribute("default").unwrap_or("");
    if project.has_attribute("default") && default != "" {
        let targets: Vec<Node> = project
            .children()
            .filter(|n| {
                n.is_element()
                    && n.tag_name().name() != "blocks"
                    && n.tag_name().name() != "block"
                    && n.attribute("name").unwrap_or("") == default
            })
            .collect();
        let mut properties = HashMap::new();
        properties.insert(
            "__project__basedir".to_owned(),
            project.attribute("basedir").unwrap_or("").to_owned(),
        );
        utils::get_properties(&project, &mut properties, log); // Global properties are assigned before target run
        let mut blocks: HashMap<String, Node> = HashMap::new();
        // Get blocks from the Blocks node
        let blocks_node: Vec<Node> = project
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "blocks")
            .collect();
        if blocks_node.len() > 0 {
            blocks_node[0].children().for_each(|block| {
                if block.tag_name().name() == "block" {
                    let block_name = block.attribute("name").expect("blocks must have a name");
                    blocks.insert(format!("{}", block_name), block);
                }
            });
        }
        // Get blocks defined globally
        project
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "block")
            .for_each(|block| {
                let block_name = block.attribute("name").expect("blocks must have a name");
                blocks.insert(format!("{}", block_name), block);
            });
        resolve_target(
            &project,
            &targets,
            &mut properties.clone(),
            &mut blocks,
            log,
        );
    }
}

fn resolve_target(
    project: &Node,
    targets: &Vec<Node>,
    properties: &mut HashMap<String, String>,
    blocks: &mut HashMap<String, Node>,
    log: &logger::Logger,
) {
    for target in targets {
        let mut scoped_blocks = blocks.clone();
        if target.has_attribute("depends") {
            let mut dependencies: Vec<&str> =
                target.attribute("depends").unwrap().rsplit(",").collect();
            dependencies.reverse();
            for dependency in dependencies {
                let dependency_targets: Vec<Node> = project
                    .children()
                    .filter(|n| n.is_element() && n.attribute("name").unwrap_or("") == dependency.trim())
                    .collect();
                resolve_target(
                    &project,
                    &dependency_targets,
                    &mut properties.clone(),
                    &mut blocks.clone(),
                    log,
                );
            }
        }
        target
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "block")
            .for_each(|block| {
                let block_name = block.attribute("name").expect("blocks must have a name");
                scoped_blocks.insert(format!("{}", block_name), block);
            });
        println!("{}:", target.attribute("name").unwrap());
        build_target(&target, &mut properties.clone(), &mut scoped_blocks, log);
    }
}

fn build_target(
    &target: &Node,
    properties: &mut HashMap<String, String>,
    blocks: &mut HashMap<String, Node>,
    log: &logger::Logger,
) {
    // utils::get_properties(&target, properties, log);
    let elements: Vec<Node> = target.children().filter(|n| n.is_element()).collect();
    for element in elements {
        match element.tag_name().name() {
            "property" => {
                utils::get_property(&element, properties, log);
            }
            "echo" => {
                let message = utils::get_text(element.text().unwrap_or(""), &properties);
                if element.has_attribute("file") {
                    let filename =
                        utils::get_text(element.attribute("file").unwrap_or(""), &properties);
                    if filename != "" {
                        let file_path =
                            std::path::Path::new(properties.get("__project__basedir").unwrap())
                                .join(filename.clone());
                        let mut file_mode =
                            std::fs::File::create(file_path).expect("Unable to create file!");
                        match file_mode.write_all(message.as_bytes()) {
                            Ok(f) => f,
                            Err(_e) => {
                                log.build_failed(format!("{} not found", filename));
                                std::process::exit(0);
                            }
                        };
                    }
                } else {
                    println!("\t[echo] {}", message);
                }
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
                task_fn(&element, &log, &properties);
            }
            "condition" => {
                let result = tasks::condition::create_task(&element, &log, &properties);
                let property = element
                    .attribute("property")
                    .expect("missing property in condition");
                properties.insert(property.to_string(), result);
            }
            "if" => {
                let is_condition_valid = element.has_attribute("value")
                    && (element.has_attribute("on-true") || element.has_attribute("on-false"));
                if is_condition_valid {
                    let key = element.attribute("value").unwrap_or("");
                    let value = properties.get(key).unwrap();
                    if value == "true" {
                        let block_name = element.attribute("on-true").unwrap_or_default();
                        let block = blocks.get(block_name).unwrap();
                        build_target(&block, &mut properties.clone(), &mut blocks.clone(), &log);
                    } else if value == "false" {
                        let block_name = element.attribute("on-false").unwrap_or_default();
                        let block = blocks.get(block_name).unwrap();
                        build_target(&block, &mut properties.clone(), &mut blocks.clone(), &log);
                    }
                } else {
                    log.build_failed("if condition is not valid".to_owned());
                }
            }
            "each" => {
                tasks::each::create_task(&element, &log, properties, blocks, &build_target);
            }
            "exec" => {
                tasks::exec::create_task(&element, &log, properties);
                if element.has_attribute("block") {
                    let block_name = element.attribute("block").unwrap_or_default();
                    let block = blocks.get(block_name).unwrap();
                    build_target(&block, &mut properties.clone(), &mut blocks.clone(), &log);
                }
            }
            "string-builder" => {
                tasks::string_builder::create_task(&element, &log, properties)
            }
            _ => {}
        }
    }
}
