use crate::types::Position;

use crate::humans::{Human, Need};

pub trait Action {
    type Item;
    fn execute(human: &mut Human, value : Self::Item);
}

pub struct Drink;

impl Action for Drink {
    type Item = i32;
    fn execute(human: &mut Human, value : Self::Item){
        for need in human.needs.iter_mut() {
            match need  {
                Need::Thirst(val) => *val = 100.min(*val + value),
                _ => ()
            }
        }
    }
}
pub struct Eat;

impl Action for Eat {
    type Item = i32;
    fn execute(human: &mut Human, value : Self::Item){
        for need in human.needs.iter_mut() {
            match need  {
                Need::Hunger(val) => *val = 100.min(*val + value),
                _ => ()
            }
        }
    }
}

pub struct Move;

impl Action for Move {
    type Item = Position;
    fn execute(human: &mut Human, value : Self::Item) {
        human.position = human.position + value;
    }
}
