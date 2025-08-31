pub enum GloxError {
    UnexpectedToken { message: String, line: usize },
}