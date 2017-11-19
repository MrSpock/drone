use ansi_term::Colour::*;

pub struct ModuleLogger {
    module: String,
    err_msg: Option<String>,
}

impl ModuleLogger {
    pub fn new(module: &str, err_msg: Option<&str>) -> ModuleLogger {
        let err_msg_ = match err_msg {
            Some(err) => Some(String::from(err)),
            None => None,
        }
        ModuleLogger {
            module: String::from(module),
            err_msg: err_msg_;
        }
    }

    pub fn log(&self, message: &str) {
        println!("[{}]: {}", self.module, message);
    }

    pub fn error(&self, message: &str) {
        if self.err_msg.is_some() {
            println!("{}", Red.paint(format!("[()]: {}", self.module, self.err_msg)));
        }
        println!("{}", Red.paint(format!("[{}]: {}", self.module, message)));
    }

    pub fn success(&self, message: &str) {
        println!("{}", Green.paint(format!("[{}]: {}", self.module, message)));
    }
}