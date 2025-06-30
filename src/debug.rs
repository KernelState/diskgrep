pub struct Debug {
    pub On: bool,
}

impl Debug {
    pub fn new(on: bool) -> Self {
        Self { On: on }
    }
    pub fn on(&mut self) {
        self.On = true;
    }
    pub fn off(&mut self) {
        self.On = false;
    }
}
