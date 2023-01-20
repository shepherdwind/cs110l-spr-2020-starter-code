pub struct Stone {
    pub x: f32,
    pub y: f32,
    pub active: bool,
}

impl Stone {
    pub fn new(x: f32, y: f32) -> Self {
        Stone {
            x,
            y,
            active: false,
        }
    }
}
impl Clone for Stone {
    fn clone(&self) -> Self {
        Stone {
            x: self.x,
            y: self.y,
            active: self.active,
        }
    }
}
