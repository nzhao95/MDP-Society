use crate::types::Position;

use crate::humans::{Human, Need};
use crate::learning::reinforcement::{Agent, Policy, State};
use crate::world::World;

pub struct RL_Behaviour<'a>  {
    policy : Policy,
    actions : Vec<Box<dyn FnOnce()>>,
    world : &'a World,
}

impl <'a> RL_Behaviour<'a> {
    fn encode(human : &Human, world: &World) -> State {
        let world_height = world.cells.len();
        assert_ne!(world_height, 0);
        let world_width = world.cells[0].len();
        assert_ne!(world_width, 0);
        let key = ((((human.position.x * world_width as i32 
        + human.position.y) * human.hunger.max_value
        + human.hunger.value) * human.thirst.max_value
        + human.thirst.value) * human.energy.max_value
        + human.energy.value) * human.money.max_value
        + human.money.value;
        
        State{key : key as usize}
    }
}

pub trait Behaviour {
    fn predict_action(&self, human : &Human) -> usize;
}
impl <'a> Behaviour for RL_Behaviour<'a>  {
    fn predict_action(&self, human : &Human) -> usize {
        let current_state = RL_Behaviour::encode(human, self.world);
        self.policy.predict_action(&current_state)
    }
}


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
