
use crate::logger;
use crate::utils;

use std::io::prelude::*;
use std::process::{Command, Stdio};
pub fn create_task<'a, 'b>(
    element: &roxmltree::Node<'a, 'b>,
    _log: &logger::Logger,
    properties_hash: &std::collections::HashMap<String, String>,
) {
    let mut args: Vec<String> = element
        .children()
        .filter(|n| n.is_element() & n.has_tag_name("arg"))
        .map(|n| format!("{}", utils::get_text(n.attribute("value").unwrap_or(""), &properties_hash)))
        .collect();

    match if element.has_attribute("executable") {
        "executable"
    } else if element.has_attribute("cmd") {
        "cmd"
    } else {
        ""
    } {
        "cmd" => {
            let mut args_vec: Vec<String> = vec![
                String::from("/C"),
                format!("{}", utils::get_text(element.attribute("cmd").unwrap_or(""), &properties_hash)),
            ];
            args_vec.append(&mut args);
            let mut cmd_process = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(&args_vec)
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("failed to execute process")
            } else {
                Command::new("sh")
                    .args(&args_vec)
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("failed to execute process")
            };
            let output_buffer = std::io::BufReader::new(cmd_process.stdout.as_mut().unwrap());
            let stdout_lines = output_buffer.lines();

            for line in stdout_lines {
                println!("{}", line.unwrap());
            }
        }
        "executable" => {            
            let process_name = utils::get_text(element.attribute("executable").unwrap_or(""), &properties_hash);
            let mut executable = Command::new(process_name)
                .args(&args)
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to execute process");
            let output_buffer = std::io::BufReader::new(executable.stdout.as_mut().unwrap());
            let stdout_lines = output_buffer.lines();

            for line in stdout_lines {
                println!("{}", line.unwrap());
            }
        }
        _ => {}
    };
}