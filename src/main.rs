use macroquad::prelude::*;
use core::f32::consts::PI;
use fastrand;

#[derive(Clone, Copy)]
struct Player {
    position: Vec2,
    rotation: f32,
    radius: f32,
}

#[derive(Clone, Copy)]
struct Bullet {
    position: Vec2,
    rotation: f32,
    length: f32,
}

#[derive(Clone)]
struct Astroid {
    position: Vec2,
    points: Vec<Vec2>,
    radius: f32,
}

const PLAYER_SPEED: f32 = 1.0;
const BULLET_SPEED: f32 = 2.0;
const BUMPYNESS: f32 = 20.0;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut player: Player = Player {
        position: Vec2{x: 200.0, y: 200.0},
        rotation: 2.0,
        radius: 20.0,
    };
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut example_astroid: Astroid = astroid_generate(player.position, 40.0);
    let mut astroids:Vec<Astroid> = Vec::new();
    loop {
        clear_background(WHITE);

        player = player_movement(player);

        if is_mouse_button_down(MouseButton::Right) {
            bullets.push(add_bullet(player));
        }
        if is_mouse_button_down(MouseButton::Left) {
            astroids.push(astroid_generate(get_random_offscree_pos(), 40.0));
            example_astroid = astroid_generate(player.position, 40.0)
        }

        bullets = bullet_move_or_kill(bullets);


        for i in 0..astroids.len() {
            astroids[i].position += Vec2{x:1.0, y:1.0};
        }
        println!("{}", astroids.len());


        example_astroid.position = player.position;

        for i in 0..astroids.len(){
            let astro = astroids[i].clone();
            astroid_draw(astro);
        }

        astroid_draw(example_astroid.clone());
        draw_bullets(bullets.clone());
        draw_player(player);
        next_frame().await
    }
}

fn player_movement(mut player:Player) -> Player {
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

fn add_bullet(player:Player) -> Bullet {
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

fn bullet_forward(mut bullet:Bullet) -> Bullet {
    bullet.position.x += BULLET_SPEED * bullet.rotation.cos();
    bullet.position.y += BULLET_SPEED * bullet.rotation.sin();
    return bullet;
}

fn draw_bullet(bullet:Bullet) {
    let pos = bullet.position;
    let rotation = bullet.rotation;
    let length = bullet.length;
    draw_line(pos.x, pos.y, pos.x-(rotation.cos()*length) , pos.y-(rotation.sin()*length), 2.0, RED)
}

fn bullet_move_or_kill(mut bullets:Vec<Bullet>) -> Vec<Bullet> {
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

fn is_bullet_offscreen(bullet:Bullet) -> bool {
    let rotation = bullet.rotation;
    let length = bullet.length;
    let pos = (bullet.position.x-(rotation.cos()*length) , bullet.position.y-(rotation.sin()*length));

    if (pos.0 > screen_width() || pos.1 > screen_height()) || (pos.0 < 0.0 || pos.1 < 0.0) {
        return true;
    }
    false
}

fn draw_bullets(bullets:Vec<Bullet>) {
    for i in bullets {
        draw_bullet(i);
    }
}

fn astroid_draw(mut astroid:Astroid) {
    astroid.points.push(astroid.points[0]);
    for i in 0..(astroid.points.len()-1) {
        let pos = astroid.position;
        let current_point = Vec2{x: astroid.points[i].x * astroid.points[i].y.cos(), y: astroid.points[i].x * astroid.points[i].y.sin()};
        let next_point = Vec2{x: astroid.points[i+1].x * astroid.points[i+1].y.cos(), y: astroid.points[i+1].x * astroid.points[i+1].y.sin()};
        draw_line(
            current_point.x + pos.x, current_point.y + pos.y,
            next_point.x + pos.x, next_point.y + pos.y,
            2.0, BLACK);
    }
    
}

fn astroid_generate(pos: Vec2, radius: f32) -> Astroid {
    let mut list_of_points:Vec<Vec2> = vec![];
    for i in 0..12{
        let point:Vec2 = Vec2 {x: radius - (fastrand::f32()*BUMPYNESS) + (BUMPYNESS/2.0), y: (PI * i as f32)/6.0};
        list_of_points.push(point);
    }

    let astroid: Astroid = Astroid {
        position: pos,
        points: list_of_points,
        radius: radius
    };

    astroid
}

fn get_random_offscree_pos() -> Vec2 {
    let distance_from_screen:f32 = 50.0;
    let location = fastrand::i32(0..4);
    let fraction = fastrand::f32();
    if location == 0 {
        return Vec2{x:((screen_width()+(2.0*distance_from_screen)) * fraction)-distance_from_screen ,y: -distance_from_screen};
    }
    if location == 1 {
        return Vec2{x:screen_width()+distance_from_screen,y: ((screen_height()+(2.0*distance_from_screen)) * fraction)-distance_from_screen};
    }
    if location == 2 {
        return Vec2{x:((screen_width()+(2.0*distance_from_screen)) * fraction)-distance_from_screen ,y: screen_height()+distance_from_screen};
    }
    if location == 3 {
        return Vec2{x:-distance_from_screen,y: ((screen_height()+(2.0*distance_from_screen)) * fraction)-distance_from_screen};
    }
    return Vec2{x:0.0, y:0.0};
}
