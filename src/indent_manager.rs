pub struct IndentManager {
    level: usize,
    indent_str: String,
}

impl IndentManager {
    pub fn new(indent_str: &str) -> Self {
        Self {
            level: 0,
            indent_str: indent_str.to_string(),
        }
    }

    pub fn increase(&mut self) {
        self.level += 1;
    }

    pub fn decrease(&mut self) {
        if self.level > 0 {
            self.level -= 1;
        }
    }

    pub fn get_indent(&self) -> String {
        self.indent_str.repeat(self.level)
    }
}
