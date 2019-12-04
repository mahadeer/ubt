use crate::logger;
use crate::utils;
use roxmltree::Node;

pub fn create_task<'a, 'b>(
    element: &roxmltree::Node<'a, 'b>,
    _log: &logger::Logger,
    properties_hash: &std::collections::HashMap<String, String>,
) -> std::string::String {
    let operations: Vec<Node> = element
        .children()
        .filter(|n| n.is_element() & n.has_tag_name("operation"))
        .collect();
    let mut results: Vec<bool> = vec![];
    for operation in operations {
        let arg1 = utils::get_text(
            operation.attribute("arg1").expect("argument 1 missing"),
            &properties_hash,
        );
        let arg2 = utils::get_text(
            operation.attribute("arg2").expect("argument 2 missing"),
            &properties_hash,
        );
        // gt, lt, gte, lte, eq, neq are numbers
        // eqS, neqS, in, not in are strings
        let result = match operation.attribute("type").expect("agreement type missing") {
            "lt" => arg1.parse::<f32>().unwrap() < arg2.parse::<f32>().unwrap(),
            "lte" => arg1.parse::<f32>().unwrap() <= arg2.parse::<f32>().unwrap(),
            "eq" => arg1.parse::<f32>().unwrap() == arg2.parse::<f32>().unwrap(),
            "neq" => arg1.parse::<f32>().unwrap() != arg2.parse::<f32>().unwrap(),
            "gt" => arg1.parse::<f32>().unwrap() > arg2.parse::<f32>().unwrap(),
            "gte" => arg1.parse::<f32>().unwrap() >= arg2.parse::<f32>().unwrap(),
            // for strings
            "eqS" => arg1 == arg2,
            "neqS" => arg1 != arg2,
            "in" => arg2.contains(&arg1),
            "not in" => !arg2.contains(&arg1),
            _ => false,
        };
        results.push(result);
    }
    let result = match element
        .attribute("type")
        .expect("type of condition required")
    {
        "and" => {
            let true_count: Vec<bool> = results.clone().into_iter().filter(|&n| n).collect();
            true_count.len() == results.len()
        },
        "or" => {
            let true_count: Vec<bool> = results.into_iter().filter(|&n| n).collect();
            true_count.len() > 0
        },
        _ => false,
    };
    format!("{}", result)
}
