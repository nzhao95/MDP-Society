use rand::{self, Rng};

pub struct Policy {
    qtable : Vec<Vec<f64>> // Reward table for each state and each action
}

impl Policy {
    pub fn new() -> Policy {
        Policy {qtable : Vec::new()}
    }

    pub fn init(&mut self, nb_states : usize, nb_actons : usize) {
        assert!(self.qtable.is_empty());
        self.qtable = vec![vec![0.0; nb_actons]; nb_states];
    }

    pub fn get_value(&self, state : &State, action : usize) -> f64 {
        self.qtable[state.key][action]
    }

    pub fn set_value(&mut self, state : &State, action : usize, value : f64) {
        self.qtable[state.key][action] = value;
    }

    pub fn predict_action(&self, state : &State) -> usize {
        self.qtable[state.key]
        .iter()
        .enumerate()
        .max_by(|(_, &a), (_, &b)| a.partial_cmp(&b)
        .unwrap())
        .unwrap().0
    }

    pub fn train<A : Agent>(&mut self, agent : &mut A, iterations : usize, alpha : f64, gamma : f64, epsilon : f64) {
        
        println!("Training Begins");
        for i in 0..iterations {
            let mut current_state = agent.reset();
            let mut reward;
            let mut finished = false;

            assert_ne!(self.qtable.len(), 0);
            
            let nb_actions = self.qtable[0].len();
            assert_ne!(nb_actions, 0);
            
            let mut rng = rand::thread_rng();
            while !finished {
                let action =
                if rng.gen_range(0.0, 1.0) < epsilon {
                    rng.gen_range(0, nb_actions)
                }
                else {
                    self.predict_action(&current_state)
                };
                
                let next_max = *self.qtable[current_state.key].iter()
                .max_by(|a,b| a.partial_cmp(b).unwrap())
                .unwrap();

                let val = &mut self.qtable[current_state.key][action];
                let old_value = *val;

                (current_state, reward, finished) = agent.do_action(action);

                *val = (1.0 - alpha) * old_value 
                        + alpha * (reward + gamma + next_max);

                agent.simulation_step_time();
            }

            if i%10000 == 0 {
                println!("Completion : {}%", i as f64 * 100.0 / iterations as f64)
            }
        }

        #[cfg(debug_assertions)]
        {
            println!("Training Finished");
        }
    }

    #[cfg(debug_assertions)]
    pub fn evaluate<A : Agent>(&self, agent : &mut A, iterations : usize) {

        println!("Evaluating model");
        let mut average_lifetime = 0.0;
        let mut average_reward = 0.0;
        for i in 0..iterations {
            let mut current_state = agent.reset();
            let mut reward;
            let mut lifetime_reward = 0.0;
            let mut finished = false;
            let mut lifetime = 0;

            while !finished {
                (current_state, reward, finished) = agent.do_action(self.predict_action(&current_state));
                agent.simulation_step_time();
                lifetime += 1;
                lifetime_reward += reward;
            }

            if i%100 == 0 {
                println!("Completion : {}%", i as f64 * 100.0 / iterations as f64)
            }

            average_lifetime += lifetime as f64 / iterations as f64;
            average_reward += lifetime_reward as f64 / (lifetime as f64 * iterations as f64);
        }
        println!("Average Lifetime : {average_lifetime}");
        println!("Average Reward : {average_reward}");
    }
}

pub struct State {
    pub key : usize
}

pub trait Agent {
    // Learning
    fn reset(&mut self) -> State;
    fn do_action(&mut self, action : usize) -> (State, f64, bool);
    fn simulation_step_time(&mut self);
    fn compute_reward(&self) -> f64;

    // Execution
    fn choose_action(&self) -> usize;
    fn step(&mut self);
}
