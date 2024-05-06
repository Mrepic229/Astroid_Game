use macroquad::prelude::*;
use core::f32::consts::PI;
//use crate::player;


#[derive(Clone, Copy)]
pub struct Player {
    pub position: Vec2,
    pub rotation: f32,
    pub radius: f32,
}

const PLAYER_SPEED: f32 = 1.0;

impl Player {

    pub fn draw(&self) {
        let radius = self.radius;
        let v1: Vec2 = Vec2 {
            x: self.position.x + (radius * self.rotation.cos()),
            y: self.position.y + (radius * self.rotation.sin())
        };
        let v2: Vec2 = Vec2 {
            x: self.position.x - (radius * (self.rotation + PI/4.0).cos()),
            y: self.position.y - (radius * (self.rotation + PI/4.0).sin())
        };
        let v3: Vec2 = Vec2 {
            x: self.position.x - (radius * (self.rotation - PI/4.0).cos()),
            y: self.position.y - (radius * (self.rotation - PI/4.0).sin())
        };
        draw_triangle(v1, v2, v3, BLACK)
    }

    pub fn rotate(&mut self) {
        self.rotation = ((mouse_position().1-self.position.y)/(mouse_position().0-self.position.x)).atan();
        if (mouse_position().0-self.position.x) < 0.0 {
            self.rotation += PI
        }
    }     
        
    pub fn move_pos(&mut self) {   
        self.position.x += PLAYER_SPEED * self.rotation.cos();
        self.position.y += PLAYER_SPEED * self.rotation.sin();
    }

}


