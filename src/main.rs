use brains::humans::behaviour::{self, RlBehaviour};
use piston_window::{PistonWindow, WindowSettings};
use brains::world::World;
use brains::humans::Human;
use brains::types::Position;
use brains::draw::Drawable;
use piston_window::*;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

static TIME_STEP : Duration = Duration::from_millis(200);
fn main() {
    let my_world = Arc::new(Mutex::new(World::new(50,50,10)));
    let mut behaviour = Arc::new(Mutex::new(RlBehaviour::new()));

    {
        let mut world_data = my_world.lock().unwrap();
        let new_human = Human::new(15, 18, Some(behaviour.clone()), world_data.environment.clone());
        world_data.add_human(new_human);
        let new_human = Human::new(6, 5, Some(behaviour.clone()), world_data.environment.clone());
        world_data.add_human(new_human);
        let new_human = Human::new(36, 45, Some(behaviour.clone()), world_data.environment.clone());
        world_data.add_human(new_human);
        let new_human = Human::new(45, 31, Some(behaviour.clone()), world_data.environment.clone());
        world_data.add_human(new_human);
        
        world_data.add_forest( Position{x : 12, y : 1}, Position{x : 18, y : 4});
        world_data.add_lake( Position{x : 27, y : 24}, Position{x : 30, y : 42});
    }
    
    let simulation_world = Arc::clone(&my_world);
    let simulation_thread = thread::spawn(move || {
        loop {
            {
                let mut world = simulation_world.lock().unwrap();
                world.step_time();
            }
            thread::sleep(TIME_STEP);
        }
    });


    let mut window: PistonWindow =
    WindowSettings::new("My small world", [512; 2])
        .build().unwrap();

    while let Some(e) = window.next() {
        let world = my_world.lock().unwrap();
        window.draw_2d(&e, |c, g, _| {
            clear([0.4, 0.8, 0.5, 1.0], g);
            world.draw(0.0, 0.0, 10.0, c, g);
        });
    }

    simulation_thread.join().unwrap();

    println!("Simulation Done");
}