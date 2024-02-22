use piston_window::{PistonWindow, WindowSettings};
use brains::world::World;
use brains::types::Position;
use brains::draw::Drawable;
use piston_window::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

static TIME_STEP : Duration = Duration::from_millis(200);
fn main() {
    let my_world = Arc::new(Mutex::new(World::new(50,50,10)));

    {
        let mut world_data = my_world.lock().unwrap();
        world_data.add_human(15, 18);
        world_data.add_human(6, 5);
        world_data.add_human(36, 45);
        world_data.add_human(45, 31);
        world_data.add_forest(Position{x : 12, y : 1}, Position{x : 18, y : 4});
        world_data.add_lake(Position{x : 27, y : 24}, Position{x : 30, y : 42});
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