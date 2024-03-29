use rand::{self, Rng};

pub struct Policy {
    pub(crate) qtable : Vec<Vec<f64>> // Reward table for each state and each action
}

impl Policy {
    pub fn new() -> Policy {
        Policy {qtable : Vec::new()}
    }

    pub fn init(&mut self, nb_states : usize, nb_actions : usize) {
        assert!(self.qtable.is_empty());
        let mut rng = rand::thread_rng();
        self.qtable = (0..nb_states)
        .map(|_| 
            (0..nb_actions)
            .map(|_| 
                rng.gen_range(-1.0, 1.0)
            )
            .collect()
        )
        .collect();
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
        
        let percent_step = iterations / 100;
        println!("Training Begins");
        assert_ne!(self.qtable.len(), 0);
        let nb_actions = self.qtable[0].len();
        assert_ne!(nb_actions, 0);
        let mut rng = rand::thread_rng();

        let mut average_reward = 0.0;
        let mut action_count = vec![0; self.qtable[0].len()];
        
        for i in 0..iterations {
            let mut current_state = agent.reset();
            let mut lifetime_reward = 0.0;
            let mut lifetime = 0;
            let mut finished = false;

            while !finished {
                let action =
                if rng.gen_range(0.0, 1.0) < epsilon {
                    rng.gen_range(0, nb_actions)
                }
                else {
                    self.predict_action(&current_state)
                };
                
                action_count[action as usize] += 1;
                
                let old_value = self.qtable[current_state.key][action];

                let (next_state, reward, dead) = agent.simulate_action(action);
                lifetime_reward += reward;

                let next_max = *self.qtable[next_state.key].iter()
                .max_by(|a,b| a.partial_cmp(b).unwrap())
                .unwrap();
            
                self.qtable[current_state.key][action] = (1.0 - alpha) * old_value 
                        + alpha * (reward + gamma * next_max);

                current_state = next_state;
                finished = dead;
                lifetime +=1;
            }
            
            average_reward += lifetime_reward as f64 / (lifetime as f64 * iterations as f64);

            if i%percent_step == 0 {
                println!("Completion : {}%", i as f64 * 100.0 / iterations as f64)
            }
        }

        #[cfg(debug_assertions)]
        {
            println!("Training Finished");
            for i in 0..20 {
                println!("{:?}", self.qtable[i]);
            }
            
            println!("Training Average Total Reward : {average_reward}");
            println!("Training Action counts : {:?}", action_count);
        }
    }

    #[cfg(debug_assertions)]
    pub fn evaluate<A : Agent>(&self, agent : &mut A, iterations : usize) {

        println!("Evaluating model");
        let mut average_lifetime = 0.0;
        let mut average_reward = 0.0;
        let mut action_count = vec![0; self.qtable[0].len()];
        for i in 0..iterations {
            let mut current_state = agent.reset();
            let mut reward;
            let mut lifetime_reward = 0.0;
            let mut finished = false;
            let mut lifetime = 0;

            while !finished {
                let action = self.predict_action(&current_state);
                action_count[action as usize] += 1;
                (current_state, reward, finished) = agent.simulate_action(action);
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
        println!("Average Total Reward : {average_reward}");
        println!("Action counts : {:?}", action_count);
    }
}

pub struct State {
    pub key : usize
}

pub trait Agent {
    // Learning
    fn reset(&mut self) -> State;
    fn simulate_action(&mut self, action : usize) -> (State, f64, bool);
    fn simulation_step_time(&mut self);
    fn compute_reward(&self) -> f64;

    // Execution
    fn choose_action(&self) -> usize;
    fn do_action(&mut self, action : usize);
    fn step(&mut self);
}
