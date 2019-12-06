use crate::logger;
use crate::utils;
use roxmltree::Node;
use std::collections::HashMap;

pub fn create_task<'a, 'b>(
    element: &roxmltree::Node<'a, 'b>,
    log: &logger::Logger,
    properties_hash: &mut std::collections::HashMap<String, String>,
    blocks: &mut HashMap<String, Node>,
    callback: &dyn Fn(
        &Node,
        &mut HashMap<String, String>,
        &mut HashMap<String, Node>,
        &logger::Logger,
    ),
) {
    /* Check for errors */
    if element.has_attribute("list") && element.has_attribute("on-loop") {
        let list = utils::get_text(element.attribute("list").unwrap_or(""), &properties_hash);
        let delimiter = element.attribute("delimiter").unwrap_or(",");
        let param = element.attribute("as").unwrap_or("param");
        let block_name = element.attribute("on-loop").unwrap_or("");

        if block_name == "" || list == "" {
            log.build_failed(
                "Missing list/on-loop values. Refer documentation for help".to_owned(),
            );
        } else {
            let block = blocks.get(block_name).unwrap();
            let list_values = list.split(delimiter);
            for value in list_values {
                properties_hash.insert(param.to_string(), value.to_string());
                callback(
                    &block,
                    &mut properties_hash.clone(),
                    &mut blocks.clone(),
                    &log,
                );
            }
        }
    } else {
        log.build_failed(
            "Missing required attributes on each, please refer the documentation".to_owned(),
        );
    }
}
