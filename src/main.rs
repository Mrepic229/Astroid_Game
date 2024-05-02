mod astroid;
mod util;
mod bullet;
mod player;

use std::vec;

use macroquad::prelude::*;
//use core::f32::consts::PI;
//use fastrand;

use crate::astroid::Astroid;
use crate::bullet::Bullet;
use crate::player::Player;

#[macroquad::main("BasicShapes")]
async fn main() {
    game_loop().await;
    
}

async fn game_loop() {
    let mut player: Player = Player {
        position: Vec2{x: 200.0, y: 200.0},
        rotation: 2.0,
        radius: 20.0,
    };
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut example_astroid: Astroid = Astroid::new(player.position, 40.0);
    let mut astroids:Vec<Astroid> = Vec::new();

    loop {
        clear_background(WHITE);

        player.rotate();
        player.move_pos();

        
        if is_mouse_button_down(MouseButton::Right) {
            bullets.push(Bullet::add(player));
        }
        if is_mouse_button_down(MouseButton::Left) {
            astroids.push(Astroid::new(util::get_random_offscree_pos(), 40.0));
        }

        bullets = Bullet::bullets_move_or_kill(bullets);

        for i in &mut astroids {
            i.move_pos(player.position);
        }

        let mut to_kill: Vec<Astroid> = vec![];
        for (j,i) in astroids.iter().enumerate() {
            if i.is_intersected(bullets) {
                to_kill.push(i.clone());
            }
        }
        


        example_astroid.position = player.position;
        
        for i in &bullets {i.draw()}
        for i in &astroids {i.draw()}
        example_astroid.draw();
        player.draw();
        next_frame().await;
    }
}







