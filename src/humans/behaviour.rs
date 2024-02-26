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
        human.thirst.value = 100.min(human.thirst.value + value);
    }
}
pub struct Eat;

impl Action for Eat {
    type Item = i32;
    fn execute(human: &mut Human, value : Self::Item){
        human.hunger.value = 100.min(human.hunger.value + value);
    }
}

pub struct Move;

impl Action for Move {
    type Item = Position;
    fn execute(human: &mut Human, value : Self::Item) {
        human.position = human.position + value;
    }
}
