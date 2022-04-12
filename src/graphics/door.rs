use super::Rectangle;

#[derive(Clone, Debug)]
pub struct Door {
    pub destination: String,
    pub position: Rectangle,
}

impl Door {
    pub fn new(destination: String, position: Rectangle) -> Self {
        Door { destination, position }
    }
}
