use piston_window::{PistonWindow, WindowSettings};
use brains::world::World;
use brains::types::Position;
use brains::draw::Drawable;
use piston_window::*;

fn main() {
    let mut my_world = World::new(50,50,10);
    my_world.add_human(15, 18);
    my_world.add_human(6, 5);
    my_world.add_human(36, 45);
    my_world.add_human(45, 31);
    my_world.add_forest(Position{x : 12, y : 1}, Position{x : 18, y : 4});
    my_world.add_lake(Position{x : 27, y : 24}, Position{x : 30, y : 42});

    let mut window: PistonWindow =
        WindowSettings::new("My small world", [512; 2])
            .build().unwrap();
    
    while let Some(e) = window.next() {
        
        window.draw_2d(&e, |c, g, _| {
            clear([0.4, 0.8, 0.5, 1.0], g);
            my_world.draw(0.0, 0.0, 10.0, c, g);
        });
    }
}