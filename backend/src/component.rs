#[derive(Clone, Copy, Debug)]
pub struct Component {
    pub from: usize,
    pub to: usize,
    pub r#type: ComponentType,
    pub value: f32,
}

impl Component {
    pub fn new(from: usize, to: usize, r#type: ComponentType, value: f32) -> Self {
        Self {
            from,
            to,
            r#type,
            value,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ComponentType {
    Cell,
    Resistor,
    Capacitor,
    Inductor,
}
