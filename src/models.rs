use crate::logger;

pub struct PerformTask<'a, 'b> {
    pub element: &'a roxmltree::Node<'a, 'b>,
    pub properties_hash: &'a std::collections::HashMap<String, String>,
    pub log: &'a logger::Logger,
}

impl<'a, 'b> PerformTask<'a, 'b> {
    pub fn new(
        element: &'a roxmltree::Node<'_, 'b>,
        log: &'a logger::Logger,
    ) -> &'a PerformTask<'a, 'b> {
        &PerformTask {
            element: element,
            log: log,
            properties_hash: &std::collections::HashMap::new(),
        }
    }

    pub fn set_props(&mut self, properties_hash: &'a std::collections::HashMap<std::string::String, std::string::String>) {
        self.properties_hash = properties_hash;
    }
}

pub struct TaskError {
    pub message: String,
    pub task_name: String,
}