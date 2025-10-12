pub struct Ctx {
    pub debug: bool
}

impl Ctx {
    pub fn new(debug: bool) -> Self {
        Self {
            debug
        }
    }
}
