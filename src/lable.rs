use macroquad::prelude::*;
use core::f32::consts::PI;

#[derive(Clone)]
pub struct TextLable {
    pub position: Vec2,
    pub offset: Vec2,
    pub text: String,
    pub size: f32,
    pub color: Color,
    pub timeout: f32,
    spawntime: f32,
}

pub struct VectorTextLable {
    pub vector: Vec<TextLable>
}

impl TextLable {

    pub fn new(offset: Vec2, text: String, color: Color) -> TextLable {
        let ammo_lable = TextLable {
            position: Vec2 { x: 0.0, y: 0.0 },
            offset: offset,
            text: text,
            size: 30.0,
            color: color,
            timeout: 10000.0,
            spawntime: get_time() as f32,
        };
        ammo_lable
    }

    pub fn draw(&self) {
        let new_position:Vec2 = Vec2 { x: self.offset.x+self.position.x, y: self.offset.y+self.position.y };

        let size: TextDimensions = measure_text(&self.text, None, self.size as u16, 1.0);
        
        draw_text(&self.text, new_position.x-(size.width/2.0), new_position.y-(size.height/2.0), self.size, self.color)
    }

    fn update_opacity(&mut self) {
        self.color.a = 1.0 - ((get_time() as f32 - self.spawntime) / self.timeout);
    }
    
    fn is_timed_out(&self) -> bool{
        if get_time() as f32 - self.spawntime > self.timeout {
            return true;
        }
        return false;
    }
}


impl VectorTextLable {
    pub fn new() -> VectorTextLable {
        VectorTextLable{
            vector: Vec::new()
        }
    }

    pub fn update_timeout(&mut self) {
        let mut to_kill: Vec<usize> = Vec::new();
        for i in 0..self.vector.len() {
            if self.vector[i].is_timed_out() {
                to_kill.push(i);
            }
        }
        for i in to_kill.clone().into_iter().rev() {
            self.vector.remove(i);
            to_kill.pop();
        }
    }
    
    pub fn update_opacity(&mut self) {
        for i in 0..self.vector.clone().into_iter().len() {
            self.vector[i].update_opacity();
        }
    }
}





