mod astroid;
mod util;
mod bullet;
mod player;
mod lable;

use std::fmt::format;
use std::vec;

use macroquad::prelude::*;
//use core::f32::consts::PI;
//use fastrand;

use crate::astroid::Astroid;
use crate::bullet::Bullet;
use crate::player::Player;
use crate::util::get_distance;

#[macroquad::main("BasicShapes")]
async fn main() {
    main_menu().await;
    loop{
        let score = game_loop().await;
        you_lose(score).await;
    }
}

async fn game_loop() -> i32 {
    const ASTROID_SPAWN_SCALER: f32 = 0.03;
    let mut astroids_per_second: f32 = 1.0;

    let mut player: Player = Player {
        position: Vec2{x: 200.0, y: 200.0},
        rotation: 2.0,
        radius: 20.0,
    };
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut example_astroid: Astroid = Astroid::new(player.position, 40.0);
    let mut astroids:Vec<Astroid> = Vec::new();
    let mut score:i32 = 0;

    let mut time_since_astroid_spawn: f32 = 0.0;
    'main: loop {
        clear_background(WHITE);

        player.rotate();
        player.move_pos();
        
        if is_key_pressed(KeyCode::Space) && bullets.len() < 5 {
            bullets.push(Bullet::add(player.position.clone(), player.rotation.clone()));
        }

        bullets = Bullet::bullets_move_or_kill(bullets);

        for i in &mut astroids {
            i.move_pos(player.position);
        }

        let mut to_kill: Vec<usize> = vec![];
        for j in 0..astroids.len() {
            if astroids[j].is_intersected(bullets.clone()) {
                to_kill.push(j);
            }
        }
        for i in to_kill.clone().into_iter().rev() {
            astroids.remove(i);
            to_kill.pop();
            score += 1;

        }

        if get_time() as f32 - time_since_astroid_spawn > astroids_per_second {
            time_since_astroid_spawn = get_time() as f32;
            astroids.push(Astroid::new(util::get_random_offscree_pos(), 40.0));
            astroids_per_second = astroids_per_second * (1.0-ASTROID_SPAWN_SCALER)
        }

        for i in &astroids {
            if get_distance(player.position, i.position) < player.radius + i.radius {
                break 'main;
            }
        }

        example_astroid.position = player.position;
        
        for i in &bullets {i.draw()}
        for i in &astroids {i.draw()}
        player.draw();

        next_frame().await;
    }
    return score
}

async fn you_lose(score: i32) {
    println!("{}", score);
    let say_score: String = format!("Score: {}", score);

    'main: loop {
        clear_background(WHITE);
        
        draw_text("YOU LOSE", screen_width()/3.0, screen_height()/3.0, 32.0, BLACK);
        draw_text(&say_score, screen_width()/3.0 ,  screen_height()/2.0, 32.0, BLACK);
        draw_text("right click to play again", screen_width()/3.0, screen_height()*2.0/3.0, 32.0, BLACK);

        if is_mouse_button_pressed(MouseButton::Right) {
            break 'main;
        }

        next_frame().await;
    }
}

async fn main_menu() {
    'main: loop {
        clear_background(WHITE);

        draw_text("ASTROIDS", screen_width()/3.0, screen_height()/3.0, 32.0, BLACK);
        draw_text("left click to play", screen_width()/3.0, screen_height()*2.0/3.0, 32.0, BLACK);
        draw_text("game by D-Pops", screen_width()/3.0, screen_height()*3.0/4.0, 32.0, BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            break 'main;
        }

        next_frame().await;
    }
}



