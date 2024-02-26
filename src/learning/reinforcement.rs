use rand;

struct Environment {
    pub states : Vec<State>, //All Possible States
    pub actions : Vec<Action>, // All Possible Actions
    pub world_size : (usize, usize)
}

struct Policy {
    qtable : Vec<Vec<f64>> // Reward table for each state and each action
}

impl Policy {
    pub fn new(environment : Environment) -> Policy {
        Policy {qtable : vec!(vec![0.0; environment.actions.len()]; environment.states.len())}
    }

    pub fn get_reward(&self, state : State, action : Action) -> f64 {
        self.qtable[state.key][action.index]
    }
}

struct State {
    pub key : usize
}

struct Agent {
    policy : Policy
}

struct Action {
    pub index : usize,
    nextstate : Box<dyn FnOnce(State) -> State>,
    probability : f32,
    reward : f32,
    done : bool
}

