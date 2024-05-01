mod astroid;
mod util;
mod bullet;
mod player;

use macroquad::prelude::*;
//use core::f32::consts::PI;
//use fastrand;

use crate::astroid::Astroid;
use crate::bullet::Bullet;
use crate::player::Player;



#[macroquad::main("BasicShapes")]
async fn main() {
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

        bullets = bullets_move_or_kill(bullets);

        astroids = astroids_move(astroids, player.position);

        example_astroid.position = player.position;

        
        for i in &bullets {i.draw()}
        for i in &astroids {i.draw()}
        example_astroid.draw();
        player.draw();
        next_frame().await;
    }
}

fn bullets_move_or_kill(bullets:Vec<Bullet>) -> Vec<Bullet> {
    

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

fn astroids_move(mut astroids:Vec<Astroid>, target:Vec2) -> Vec<Astroid> {
    for i in &mut astroids {
        i.move_pos(target);
    }
    astroids
}






