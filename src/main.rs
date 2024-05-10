mod astroid;
mod util;
mod bullet;
mod player;
mod lable;

use std::vec;

use macroquad::{audio, prelude::*};
use core::f32::consts::PI;
//use fastrand;

use crate::astroid::Astroid;
use crate::bullet::Bullet;
use crate::player::Player;
use crate::lable::TextLable;
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
    const MAX_BULLETS: i32 = 5;
    let mut astroids_per_second: f32 = 1.0;
    let sound_laser = audio::load_sound("../sound/blaster.mp3").await.unwrap();
    let sound_explosion = audio::load_sound("../sound/explosion.mp3").await.unwrap();

    let mut score_lable = TextLable {
        position: Vec2 { x: 0.0, y: 0.0 },
        offset: Vec2 { x: 0.0, y: -20.0 },
        rotation: PI,
        text: format!("Space to shoot laser"),
        size: 30.0,
        color: BLACK,
    };
    let mut ammo_lable = TextLable {
        position: Vec2 { x: 0.0, y: 0.0 },
        offset: Vec2 { x: 0.0, y: 50.0 },
        rotation: PI,
        text: format!("|||||"),
        size: 30.0,
        color: RED,
    };

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
        
        if is_key_pressed(KeyCode::Space) && bullets.len() < MAX_BULLETS as usize {
            bullets.push(Bullet::add(player.position.clone(), player.rotation.clone()));
            audio::play_sound_once(&sound_laser);
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
            score_lable.text = format!("Score: {}", score)
        }

        if get_time() as f32 - time_since_astroid_spawn > astroids_per_second {
            time_since_astroid_spawn = get_time() as f32;
            astroids.push(Astroid::new(util::get_random_offscree_pos(), 40.0));
            if astroids.len() > 10{
                astroids_per_second = astroids_per_second * (1.0-ASTROID_SPAWN_SCALER)
            }
        }

        for i in &astroids {
            if get_distance(player.position, i.position) < player.radius + i.radius {
                audio::play_sound_once(&sound_explosion);
                break 'main;
            }
        }

        let mut ammo_count_at_frame: String = format!("");
        for _i in 0..MAX_BULLETS as usize-bullets.len(){ 
            ammo_count_at_frame = format!("{}|", ammo_count_at_frame)
        }
        ammo_lable.text = ammo_count_at_frame;
        
        score_lable.position = player.position;
        ammo_lable.position = player.position;
        example_astroid.position = score_lable.position;

        for i in &bullets {i.draw()}
        for i in &astroids {i.draw()}
        score_lable.draw();
        ammo_lable.draw();
        player.draw();
        //example_astroid.draw();

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
        draw_text("left click to play again", screen_width()/3.0, screen_height()*2.0/3.0, 32.0, BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
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



