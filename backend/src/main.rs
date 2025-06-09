mod circuit;
mod component;

use crate::{
    circuit::Circuit,
    component::{Component, ComponentType},
};

fn main() {
    let cell = Component::new(0, 1, ComponentType::Cell, 5.0);

    let resistor = Component::new(2, 3, ComponentType::Resistor, 5.0);

    let circuit = Circuit::new(vec![cell, resistor]).solve();
}
