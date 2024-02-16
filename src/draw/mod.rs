use crate::world::{Environment, World};
use crate::humans::Human;
use piston_window::*;
pub trait Drawable {
    fn draw(&self, x : f64, y : f64, cell_size : f64, c: Context, g : &mut G2d);
}

impl Drawable for Environment {
    fn draw(&self, x : f64, y : f64, cell_size : f64, c: Context, g : &mut G2d) {
        match self {
            Environment::Tree(val) =>
                    rectangle(
                        [0.4 * (1.0 - val) as f32, 0.8 * (1.0 - 0.9* val) as f32, 
                                0.5 * (1.0 - val) as f32, 
                                1.0],
                        [x as f64 * cell_size, y as f64 *cell_size, cell_size, cell_size], // rectangle
                        c.transform, g),
            Environment::Water(val) => 
                    rectangle(
                        [0.0, 0.0, 0.7 * (1.0 - val * 0.8) as f32, 1.0],
                        [x as f64 * cell_size, y as f64 *cell_size, cell_size, cell_size], // rectangle
                        c.transform, g),
            Environment::Grass(val) => 
                    rectangle(
                        [0.0, 0.7 * (1.0 - val * 0.8) as f32, 0.0, 1.0],
                        [x as f64 * cell_size, y as f64 *cell_size, cell_size, cell_size], // rectangle
                        c.transform, g),
            Environment::House(_) => 
                    rectangle(
                        [0.3, 0.1, 0.2, 1.0],
                        [x as f64 * cell_size, y as f64 *cell_size, cell_size, cell_size], // rectangle
                        c.transform, g),
            Environment::None => 
                    rectangle(
                        [0.0, 0.0, 0.0, 0.0],
                        [x as f64 * cell_size, y as f64 *cell_size, cell_size, cell_size], // rectangle
                        c.transform, g)
        }
    }
}

impl Drawable for Human {
    fn draw(&self, x : f64, y : f64, cell_size : f64, c: Context, g : &mut G2d) {
        ellipse(
            [1.0, 0.0, 0.0, 1.0],
            [x * cell_size, 
            y * cell_size, 
            cell_size, 
            cell_size], // rectangle
            c.transform, g);
    }
}

impl Drawable for World {

    fn draw(&self, _ : f64 , _ : f64, cell_size : f64, c: Context, g : &mut G2d) {
        
        let width = self.cells.len();
        if width == 0 {
            return;
        }
        let height = self.cells[0].len();
        if height == 0 {
            return;
        }

        for x in 0..width {
            for y in 0..height {
                self.cells[x][y].draw(x as f64, y as f64, cell_size, c, g);
            }
        }
        for human in self.humans.iter() {
            human.draw(human.position.x as f64, human.position.y as f64, cell_size, c, g);
        }
    }
}