use rand::Rng;

use crate::types::Position;

use crate::humans::Human;
use crate::learning::reinforcement::{Agent, Policy, State};

pub struct RlBehaviour {
    policy : Policy
}

impl RlBehaviour {
    pub fn new() -> RlBehaviour {
        RlBehaviour {
            policy : Policy::new(0,0)
        }
    }
}

pub trait Behaviour {
    fn predict_action(&self, human : &Human) -> usize;
    fn encode(&self, human : &Human) -> State;
}
impl Behaviour for RlBehaviour {
    fn predict_action(&self, human : &Human) -> usize {
        let current_state = self.encode(human);
        self.policy.predict_action(&current_state)
    }
    
    fn encode(&self, human : &Human) -> State {
        let key = ((((
          human.position.x    * human.environment.read().unwrap().world_limits.1 as i32 
        + human.position.y  ) * human.hunger.max_value
        + human.hunger.value) * human.thirst.max_value
        + human.thirst.value) * human.energy.max_value
        + human.energy.value) * human.money.max_value
        + human.money.value;
        
        State{key : key as usize}
    }
}

impl Agent for Human{
    fn reset(&mut self) -> State {
        let mut rng = rand::thread_rng();
        let behaviour = &self.behaviour;
        self.position = Position{x : rng.gen_range(0, self.environment.read().unwrap().world_limits.0 as i32), 
                                 y : rng.gen_range(0, self.environment.read().unwrap().world_limits.1 as i32)};
        self.hunger.value = 100;
        self.thirst.value = 100;
        self.energy.value = 100;
        self.money.value = 0;
        self.alive = true;

        behaviour.lock().unwrap().encode(&self)
    }

    fn step(&mut self, action : usize) -> (State, f64, bool) {
        let behaviour = &self.behaviour;
        match action {
            0 => (),
            _ => ()
        }

        (behaviour.lock().unwrap().encode(&self), 0.0, false)
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