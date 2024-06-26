use macroquad::prelude::*;
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

    pub fn new(position: Vec2, rotation: f32) -> Bullet {
        let mut bullet: Bullet = Bullet {
            position: Vec2 {
                x: position.x,
                y: position.y
            },
            rotation: rotation,
            length: 20.0, 
        };
        bullet.move_pos(); // spawns laser infront
        bullet.move_pos(); // of ship and not in ship
        bullet
    }
    
    pub fn move_pos(&mut self) {
        self.position.x += BULLET_SPEED * self.rotation.cos();
        self.position.y += BULLET_SPEED * self.rotation.sin();
    }

    pub fn bullets_move_or_kill(bullets:Vec<Bullet>) -> Vec<Bullet> {
        let mut new_total: Vec<Bullet> = bullets.clone();
        let mut new_total_mut: Vec<&mut Bullet> = new_total.iter_mut().collect();
    
        let mut to_remove = vec![];
        for (j, i) in new_total_mut.iter_mut().enumerate() {
            i.move_pos();
            if i.is_offscreen() {
                to_remove.push(j);
            }
        }
    
        for j in to_remove.into_iter().rev() {
            new_total.remove(j);
        }
        new_total
    }
}