use rand::Rng;

use crate::types::Position;

use crate::humans::Human;
use crate::learning::reinforcement::{Agent, Policy, State};

fn encode(human : &Human) -> State {
    let key = ((((
      human.position.x as usize    * human.environment.read().unwrap().world_limits.1 
    + human.position.y as usize  ) * human.hunger.max_value as usize
    + human.hunger.value as usize) * human.thirst.max_value as usize
    + human.thirst.value as usize) * human.energy.max_value as usize
    + human.energy.value as usize) * human.money.max_value as usize
    + human.money.value as usize;
    
    State{key : key as usize}
}

fn nb_states(human : &Human) -> usize {
    let env = human.environment.read().unwrap();
    env.world_limits.0 * env.world_limits.1 
        * human.hunger.max_value as usize
        * human.thirst.max_value as usize
        * human.energy.max_value as usize
        * human.money.max_value as usize
}
pub struct RlBehaviour {
    policy : Policy
}

impl RlBehaviour {
    pub fn new() -> RlBehaviour {
        RlBehaviour {
            policy : Policy::new()
        }
    }

    fn init(&mut self, train_agent : &mut Human) {
        self.policy.init(nb_states(&train_agent), 6);
    }

    pub fn train(&mut self, train_agent: &mut Human, iterations: usize, alpha: f64, gamma: f64, epsilon: f64) {
        self.init(train_agent);
        self.policy.train(train_agent, iterations, alpha, gamma, epsilon)
    }
}


pub trait Behaviour {
    fn predict_action(&self, human : &Human) -> usize;    
    fn step(&self, human : &mut Human);
}

impl Behaviour for RlBehaviour {
    fn predict_action(&self, human : &Human) -> usize {
        let current_state = encode(human);
        self.policy.predict_action(&current_state)
    }
    
    fn step(&self, human : &mut Human) {
        human.do_action(self.predict_action(human));
    }
}

impl Agent for Human{
    fn reset(&mut self) -> State {
        let mut rng = rand::thread_rng();
        self.position = Position{x : rng.gen_range(0, self.environment.read().unwrap().world_limits.0 as i32), 
                                 y : rng.gen_range(0, self.environment.read().unwrap().world_limits.1 as i32)};
        self.hunger.value = 100;
        self.thirst.value = 100;
        self.energy.value = 100;
        self.money.value = 0;
        self.alive = true;

        encode(&self)
    }

    fn do_action(&mut self, action : usize) -> (State, f64, bool) {
        (encode(&self), 0.0, false)
    }

    fn choose_action(&self) -> usize {
        let behaviour = self.behaviour.read().unwrap();
        behaviour.predict_action(self)
    }

    fn step(&mut self) {
        self.do_action(self.choose_action());
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