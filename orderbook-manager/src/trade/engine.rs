pub struct Engine {
    pub orderbooks: Vec<String>,
}

impl Engine {
    pub fn new() -> Self {
        Engine { orderbooks: vec![] }
    }
}
