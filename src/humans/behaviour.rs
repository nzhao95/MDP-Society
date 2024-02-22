use crate::humans::{Human, Need};

pub trait Action {
    fn execute(human: &mut Human, value : i32);
}

pub struct Drink {
}

impl Action for Drink {
    fn execute(human: &mut Human, value : i32){
        for need in human.needs.iter_mut() {
            match need  {
                Need::Thirst(val) => *val = 100.min(*val + value),
                _ => ()
            }
        }
    }
}
pub struct Eat {
}

impl Action for Eat {
    fn execute(human: &mut Human, value : i32){
        for need in human.needs.iter_mut() {
            match need  {
                Need::Hunger(val) => *val = 100.min(*val + value),
                _ => ()
            }
        }
    }
}

