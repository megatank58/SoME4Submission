use crate::component::Component;

#[derive(Clone, Debug)]
pub struct Circuit {
    pub components: Vec<Component>,
    net_voltage: f32,
    net_resistance: f32,
}

impl Circuit {
    pub fn new(components: Vec<Component>) -> Self {
        Self {
            components,
            net_voltage: 0.0,
            net_resistance: 0.0,
        }
    }

    pub fn solve(&mut self) {
        self.components.sort_by(|a, b| a.from.cmp(&b.from));

        
    }

    fn solve_node(&mut self, components: Vec<Component>) {

    }
}
