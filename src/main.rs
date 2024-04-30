mod astroid;
mod util;
mod bullets;
mod player;

use macroquad::prelude::*;
use core::f32::consts::PI;
use fastrand;

use crate::astroid::Astroid;
use crate::bullets::Bullet;
use crate::player::Player;



const PLAYER_SPEED: f32 = 1.0;

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

        player = player_rotation(player);

        // Spawn Bullets
        if is_mouse_button_down(MouseButton::Right) {
            bullets.push(add_bullet(player));
        }
        if is_mouse_button_down(MouseButton::Left) {
            astroids.push(astroid_generate(get_random_offscree_pos(), 40.0));
        }

        bullets = bullets_move_or_kill(bullets);

        astroids = astroids_move(astroids, player);

        example_astroid.position = player.position;

        astroid_draw(example_astroid.clone());
        astroids_draw(astroids.clone());
        draw_bullets(bullets.clone());
        draw_player(player);
        next_frame().await;
    }
}

fn player_rotation(mut player:Player) -> Player {
    player.rotation = ((mouse_position().1-player.position.y)/(mouse_position().0-player.position.x)).atan();
    if (mouse_position().0-player.position.x) < 0.0 {
        player.rotation += PI
    }
        
    if !is_mouse_button_down(MouseButton::Left) {
        player = player_forward(player.clone());
    }  
    player
}

fn draw_player(player:Player) {
    let radius = player.radius;
    let v1: Vec2 = Vec2 {
        x: player.position.x + (radius * player.rotation.cos()),
        y: player.position.y + (radius * player.rotation.sin())
    };
    let v2: Vec2 = Vec2 {
        x: player.position.x - (radius * (player.rotation + PI/4.0).cos()),
        y: player.position.y - (radius * (player.rotation + PI/4.0).sin())
    };
    let v3: Vec2 = Vec2 {
        x: player.position.x - (radius * (player.rotation - PI/4.0).cos()),
        y: player.position.y - (radius * (player.rotation - PI/4.0).sin())
    };

    draw_triangle(v1, v2, v3, BLACK)
}

fn player_forward(mut player:Player) -> Player {
    player.position.x += PLAYER_SPEED * player.rotation.cos();
    player.position.y += PLAYER_SPEED * player.rotation.sin();
    return player;
}

fn bullets_move_or_kill(mut bullets:Vec<Bullet>) -> Vec<Bullet> {
    let mut i = 0;
    while i < bullets.len() {
        bullets[i] = bullet_forward(bullets[i]);
        if is_bullet_offscreen(bullets[i]) {
            bullets.remove(i);
        }
        i += 1;
    }
    bullets
}

fn draw_bullets(bullets:Vec<Bullet>) {
    for i in bullets {
        Bullet::draw(&i);
    }
}

fn astroids_draw(astroids:Vec<Astroid>) {
    for i in 0..astroids.len(){
        let mut astro = astroids[i].clone();
        astro.draw();
    }
}

fn astroids_move(mut astroids:Vec<Astroid>,player:Player) -> Vec<Astroid> {
    for i in 0..astroids.len() {
        
    }
    astroids
}

pub fn move_pos(&mut self) {
    let rotation = util::get_angle(astroids[i].position, player.position);
    astroids[i].position += Vec2{x: rotation.cos(), y: rotation.sin()} * ASTROID_SPEED;
}




