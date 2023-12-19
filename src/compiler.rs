pub struct Compiler {
    pub source: String,
}

impl Compiler {
    pub fn run(&self) {
        println!("{}", self.source);
    }
}
