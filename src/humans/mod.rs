use crate::types::Position;

pub struct Human {
    pub position : Position,
    needs : Vec<Need>
}

impl Human {
    pub fn new(x : i32, y : i32) -> Self{
        Human{
            position : Position{x, y},
            needs : vec![Need::Hunger(100),
            Need::Thirst(100),
            Need::Energy(100),
            Need::Money(0)]
        }
    }

    pub fn step_time(&mut self) {
        for need in self.needs.iter_mut() {
            match need {
                Need::Hunger(mut val) => val -=1,
                Need::Thirst(mut val) => val -=1,
                Need::Energy(mut val) => val -=1,
                Need::Money(mut val) => val -=1
            }
        }
    }
}

pub enum Need {
    Hunger(u32), 
    Thirst(u32),
    Energy(u32),
    Money(u32)
}