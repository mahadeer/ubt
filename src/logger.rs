pub struct Logger {
    start_time: std::time::Instant,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            start_time: std::time::Instant::now(),
        }
    }

    pub fn log(&self, message: String) {
        println!("{}", message);
    }

    pub fn build_failed(&self, message: String) {
        println!("Error: {}", message);
        let end_time = self.start_time.elapsed();
        println!("\nBUILD FAILED \n\tTotal Time: {:?}", end_time);
    }

    pub fn build_sucessful(&self) {
        let end_time = self.start_time.elapsed();
        println!("\nBUILD SUCCESSFUL \nTotal Time: {:?}", end_time);
    }
}