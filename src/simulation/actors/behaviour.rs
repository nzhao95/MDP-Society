
use crate::learning::reinforcement::{Agent, Policy, State};
use crate::simulation::actors::humans::Human;
use crate::simulation::world::{Element, Environment};
use crate::types::Position;

use rand::Rng;
use std::cmp::max;

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
        self.policy.init(nb_states(&train_agent), 7);
    }

    pub fn train(&mut self, train_agent: &mut Human, iterations: usize, alpha: f64, gamma: f64, epsilon: f64) {
        self.init(train_agent);
        self.policy.train(train_agent, iterations, alpha, gamma, epsilon)
    }

    pub fn evaluate(&self, test_agent : &mut Human, iterations: usize) {
        self.policy.evaluate(test_agent, iterations)
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
        self.age = 0;
        self.hunger.value = 100;
        self.thirst.value = 100;
        self.energy.value = 100;
        self.money.value = 0;
        self.alive = true;

        encode(&self)
    }

    fn simulation_step_time(&mut self) {
        {
            self.hunger.value = max(self.hunger.value - 1, 0);
            // if self.hunger.value <= 0 {
            //     self.alive = false;
            // }
        }
        {
            self.thirst.value = max(self.thirst.value - 1, 0);
            // if self.thirst.value <= 0 {
            //     self.alive = false;
            // }
        }
        {
            self.energy.value = max(self.energy.value - 1, 0);
        }
        {
            self.money.value = max(self.money.value - 1, 0);
        }
        
        self.age += 1;
    }

    fn compute_reward(&self) -> f64{
        if !self.alive {
            return -50.0;
        }

        if self.age > 10000 {
            return 100.0;
        }

        let hunger_reward = if self.hunger.value > 80 { 20.0 }
        else if self.hunger.value > 30 { 1.0 }
        else if self.hunger.value > 10 { -1.0 } 
        else { -5.0 };

        let thirst_reward = if self.thirst.value > 80 { 20.0 }
        else if self.thirst.value > 30 { 1.0 }
        else if self.thirst.value > 10 { -1.0 } 
        else { -5.0 };

        let env = self.environment.read().unwrap();
        let resource_reward = match env.get_element(self.position.x as usize, self.position.y as usize) {
            Element::Water(_) => 10000.0,
            Element::Tree(_) => 10000.0,
            _ => 0.0
        };

        hunger_reward + thirst_reward +resource_reward+ 1.0
    }

    fn simulate_action(&mut self, action : usize) -> (State, f64, bool) {
        let mut reward = 0.0;
        match action {
            0 => reward = Move::execute(self, Position::new(1, 0)),
            1 => reward = Move::execute(self, Position::new(-1, 0)),
            2 => reward = Move::execute(self, Position::new(0, 1)),
            3 => reward = Move::execute(self, Position::new(0, -1)),
            4 => reward = Drink::execute(self, 100),
            5 => reward = Eat::execute(self, 100),
            _ => reward = {
                let env = &self.environment.read().unwrap();
                let resource = env.get_element(self.position.x as usize, self.position.y as usize); 
                if matches!(resource, Element::Water(_)) {
                    1000.0;
                }
                else if matches!(resource, Element::Tree(_)) {
                    1000.0;
                }
                -100.0
            }
            
        }
        self.simulation_step_time();
        if !self.alive {
            reward = -100.0;
        }
        
        (encode(&self), reward, !self.alive || self.age > 10000)
    }

    fn choose_action(&self) -> usize {
        let behaviour = self.behaviour.read().unwrap();
        behaviour.predict_action(self)
    }

    fn do_action(&mut self, action : usize) {
        match action {
            0 => Move::execute(self, Position::new(1, 0)),
            1 => Move::execute(self, Position::new(-1, 0)),
            2 => Move::execute(self, Position::new(0, 1)),
            3 => Move::execute(self, Position::new(0, -1)),
            4 => Drink::execute(self, 100),
            5 => Eat::execute(self, 100),
            _ => 0.0
        };
    }

    fn step(&mut self) {
        self.do_action(self.choose_action());
    }
}

pub trait Action {
    type Item;
    fn execute(human: &mut Human, value : Self::Item) -> f64;
}

pub struct Drink;

impl Action for Drink {
    type Item = i32;
    fn execute(human: &mut Human, value : Self::Item) -> f64{
        if let Element::Water(_) = human.environment.read().unwrap()
            .get_element(human.position.x as usize, human.position.y as usize) {
                human.thirst.value = 100.min(human.thirst.value + value);
                return 50.0;
            }
        -100.0
    }
}
pub struct Eat;

impl Action for Eat {
    type Item = i32;
    fn execute(human: &mut Human, value : Self::Item) -> f64{
        if let Element::Tree(_) = human.environment.read().unwrap()
                                .get_element(human.position.x as usize, human.position.y as usize) {
            human.hunger.value = 100.min(human.hunger.value + value);
            return 50.0;
        }
        -100.0
    }
}

pub struct Move;

impl Action for Move {
    type Item = Position;
    fn execute(human: &mut Human, value : Self::Item) -> f64 {
        human.position = human.position + value;
        let world_limits = human.environment.read().unwrap().world_limits;

        if human.position.x < 0 || human.position.x >= world_limits.0 as i32 {
            human.position.x = human.position.x.clamp(0, world_limits.0 as i32 - 1);
            return -1.0;
        }
        if human.position.y < 0 || human.position.y >= world_limits.1 as i32 {
            human.position.y = human.position.y.clamp(0, world_limits.1 as i32 - 1);
            return -1.0
        }
        
        let env = &human.environment.read().unwrap();
        let resource = env.get_element(human.position.x as usize, human.position.y as usize); 
        if human.thirst.value < 80 && matches!(resource, Element::Water(_)) {
            return 1000.0;
        }
        else if human.hunger.value < 80  && matches!(resource, Element::Tree(_)) {
            return 1000.0;
        }
        else if human.thirst.value < 80 {
            let closest_lake = 
                env.lakes
                .iter()
                .min_by(|x,y| (&x).manhattan_dist(&human.position).cmp(&y.manhattan_dist(&human.position)))
                .unwrap();
            if (closest_lake.x - human.position.x) * value.x > 0 
            || (closest_lake.y - human.position.y) * value.y > 0 {
                return 1000.0;
            }
            else {
                return 0.0;
            }
        }
        else if human.hunger.value < 80 {
            let closest_forest = 
                env.forests
                .iter()
                .min_by(|x,y| (&x).manhattan_dist(&human.position).cmp(&y.manhattan_dist(&human.position)))
                .unwrap();
            if (closest_forest.x - human.position.x) * value.x > 0 
            || (closest_forest.y - human.position.y) * value.y > 0 {
                return 1000.0;
            }
            else {
                return 0.0;
            }
        }

        100.0
    }
} 


fn encode(human: &Human) -> State {
    let env = human.environment.read().unwrap();
    let thirst_state = match human.thirst.value {
        v if v > 80 => 0,
        v if v > 30 => 1,
        v if v > 10 => 2,
        _ => 3,
    };

    let hunger_state = match human.hunger.value {
        v if v > 80 => 0,
        v if v > 30 => 1,
        v if v > 10 => 2,
        _ => 3,
    };

    let distance_to_lake = env.distance_to_lake(human);
    let closeness_to_lake = match distance_to_lake {
        v if v > 20 => 0,
        v if v > 5 => 1,
        _ => 2,
    };

    let distance_to_forest = env.distance_to_forest(human);
    let closeness_to_forest = match distance_to_forest {
        v if v > 20 => 0,
        v if v > 5 => 1,
        _ => 2,
    };

    let current_element = match env.get_element(human.position.x as usize, human.position.y as usize) {
        Element::Water(_) => 0,
        Element::Tree(_) => 1,
        _ => 2,
    };

    // Calculate the key using the encoded states
    let key = (((((human.position.x as usize * env.world_limits.1 + human.position.y as usize) * 4
        + thirst_state as usize) * 4
        + hunger_state as usize) * 3
        + closeness_to_lake as usize) * 3
        + closeness_to_forest as usize) * 3
        + current_element as usize;

    State { key: key as usize }
}


fn nb_states(human : &Human) -> usize {
    let env = human.environment.read().unwrap();
    env.world_limits.0      // World Height
    * env.world_limits.1    // World Width
    * 4                     // Thirst States
    * 4                     // Hunger States    
    * 3                     // Closeness to Forest States
    * 3                     // Closeness to Lake States
    * 3                     // Current Element
}