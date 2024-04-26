#[derive(Debug)]
pub struct TikError {
    line: usize,
    message: String,
}

impl TikError {
    pub fn error(line: usize, message: String) -> TikError {
        TikError { line, message }
    }

    pub fn report(&self, loc: String) {
        eprintln!("[line {}] Erro {}: {}", self.line, loc, self.message);
    }
}
