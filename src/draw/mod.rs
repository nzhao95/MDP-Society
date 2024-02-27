use crate::world::{Element, World};
use crate::humans::{Human, Need};
use piston_window::*;
pub trait Drawable {
    fn draw(&self, x : f64, y : f64, cell_size : f64, c: Context, g : &mut G2d);
}

impl Drawable for Element {
    fn draw(&self, x : f64, y : f64, cell_size : f64, c: Context, g : &mut G2d) {
        match self {
            Element::Tree(val) =>
                    rectangle(
                        [0.2 , 0.8 * (1.0 - 0.2* val) as f32, 
                                0.3 * (2.0 - val) as f32, 
                                1.0],
                        [x as f64 * cell_size, y as f64 *cell_size, cell_size, cell_size], // rectangle
                        c.transform, g),
            Element::Water(val) => 
                    rectangle(
                        [0.3, 0.3, 0.7 * (1.0 - val * 0.2) as f32, 1.0],
                        [x as f64 * cell_size, y as f64 *cell_size, cell_size, cell_size], // rectangle
                        c.transform, g),
            Element::Grass(val) => 
                    rectangle(
                        [0.0, 0.7 * (1.0 - val * 0.8) as f32, 0.0, 1.0],
                        [x as f64 * cell_size, y as f64 *cell_size, cell_size, cell_size], // rectangle
                        c.transform, g),
            Element::House(_) => 
                    rectangle(
                        [0.3, 0.1, 0.2, 1.0],
                        [x as f64 * cell_size, y as f64 *cell_size, cell_size, cell_size], // rectangle
                        c.transform, g),
            Element::None => 
                    rectangle(
                        [0.0, 0.0, 0.0, 0.0],
                        [x as f64 * cell_size, y as f64 *cell_size, cell_size, cell_size], // rectangle
                        c.transform, g)
        }
    }
}

impl  Drawable for Human {
    fn draw(&self, x : f64, y : f64, cell_size : f64, c: Context, g : &mut G2d) {
        let color = if !self.alive {[0.0, 0.0, 0.0, 0.3]} 
        else {
            let mut out_color = [0.0, 0.0, 0.0, 1.0];
            out_color[0] = self.thirst.value as f32 * 0.01;
            out_color[2] = self.hunger.value as f32 * 0.01;
            out_color
        };
        ellipse(
            color,
            [x * cell_size, 
            y * cell_size, 
            cell_size, 
            cell_size], // rectangle
            c.transform, g);
    }
}

impl Drawable for World {

    fn draw(&self, _ : f64 , _ : f64, cell_size : f64, c: Context, g : &mut G2d) {
        
        let width = self.environment.read().unwrap().cells.len();
        if width == 0 {
            return;
        }
        let height = self.environment.read().unwrap().cells[0].len();
        if height == 0 {
            return;
        }

        for x in 0..width {
            for y in 0..height {
                self.environment.read().unwrap().cells[x][y].draw(x as f64, y as f64, cell_size, c, g);
            }
        }
        for human in self.humans.iter() {
            human.draw(human.position.x as f64, human.position.y as f64, cell_size, c, g);
        }
    }
}