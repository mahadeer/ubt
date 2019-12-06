use crate::logger;
use crate::utils;

pub fn create_task<'a, 'b>(
    element: &roxmltree::Node<'a, 'b>,
    log: &logger::Logger,
    properties_hash: &mut std::collections::HashMap<String, String>,
) {
    /* Check for errors */
    if element.has_attribute("name")
        && element.has_attribute("value")
        && element.has_attribute("type")
    {
        let value = utils::get_text(element.attribute("value").unwrap_or(""), &properties_hash);
        let prop_name = element.attribute("name").unwrap_or("");

        if value == "" || prop_name == "" {
            log.build_failed("Missing name/value. Refer documentation for help".to_owned());
        } else {
            match element.attribute("type").unwrap_or("") {
                "trim" => {
                    properties_hash.insert(prop_name.to_owned(), value.trim().to_owned());
                }
                "toUpperCase" => {
                    properties_hash.insert(prop_name.to_owned(), value.to_uppercase().to_owned());
                }
                "toLowerCase" => {
                    properties_hash.insert(prop_name.to_owned(), value.to_lowercase().to_owned());
                }
                _ => {
                    log.log("UNKNOWN string-builder TYPE, Refer Documentation.".to_owned());
                }
            }
        }
    } else {
        log.build_failed(
            "Missing required attributes on string-builder, please refer the documentation"
                .to_owned(),
        );
    }
}
