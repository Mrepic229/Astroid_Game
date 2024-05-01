use macroquad::prelude::*;
use crate::player::Player;
//use core::f32::consts::PI;

const BULLET_SPEED: f32 = 2.0;

#[derive(Clone, Copy)]
pub struct Bullet {
    pub position: Vec2,
    pub rotation: f32,
    pub length: f32,
}

impl Bullet {
    pub fn is_offscreen(&self) -> bool {
        let rotation = self.rotation;
        let length = self.length;
        let pos = (self.position.x-(rotation.cos()*length) , self.position.y-(rotation.sin()*length));
    
        if (pos.0 > screen_width() || pos.1 > screen_height()) || (pos.0 < 0.0 || pos.1 < 0.0) {
            return true;
        }
        false
    }
    
    pub fn draw(&self) {
        let pos = self.position;
        let rotation = self.rotation;
        let length = self.length;
        draw_line(pos.x, pos.y, pos.x-(rotation.cos()*length) , pos.y-(rotation.sin()*length), 2.0, RED)
    }

    pub fn add(player:Player) -> Bullet {
        let bullet: Bullet = Bullet {
            position: Vec2 {
                x: player.position.x,
                y: player.position.y
            },
            rotation: player.rotation,
            length: 20.0, 
        };
        bullet
    }
    
    pub fn move_pos(&mut self) {
        self.position.x += BULLET_SPEED * self.rotation.cos();
        self.position.y += BULLET_SPEED * self.rotation.sin();
    }

}