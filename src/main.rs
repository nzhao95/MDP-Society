use brains::simulation::{
        actors:: { behaviour::RlBehaviour,
                    humans::Human},
        world::World};
            
use brains::display::draw::Drawable;
use brains::types::Position;

use piston_window::{PistonWindow, WindowSettings};
use piston_window::*;

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

static TIME_STEP : Duration = Duration::from_millis(200);
fn main() {
    let my_world = Arc::new(Mutex::new(World::new(20,20,10)));
    let behaviour = Arc::new(RwLock::new(RlBehaviour::new()));
    {
        let mut world_data = my_world.lock().unwrap();
        world_data.add_forest( Position{x : 12, y : 1}, Position{x : 18, y : 4});
        world_data.add_lake( Position{x : 1, y : 15}, Position{x : 3, y : 19});
    }
    
    {
        let world_data = my_world.lock().unwrap();
        let mut train_human = Human::new(0, 0, behaviour.clone(), world_data.environment.clone());
        behaviour.write().unwrap()
        .train(&mut train_human, 1000, 0.2, 0.6, 0.8);
    }
    
    {
        
        let world_data = my_world.lock().unwrap();
        let mut test_human = Human::new(15, 18, behaviour.clone(), world_data.environment.clone());
        behaviour.read().unwrap().evaluate(&mut test_human, 1000);
    }

    {
        let mut world_data = my_world.lock().unwrap();
        let new_human = Human::new(15, 18, behaviour.clone(), world_data.environment.clone());
        world_data.add_human(new_human);
        let new_human = Human::new(6, 5, behaviour.clone(), world_data.environment.clone());
        world_data.add_human(new_human);
        let new_human = Human::new(6, 5, behaviour.clone(), world_data.environment.clone());
        world_data.add_human(new_human);
        let new_human = Human::new(5, 13, behaviour.clone(), world_data.environment.clone());
        world_data.add_human(new_human);
        
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
    WindowSettings::new("My small world", [200; 2])
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