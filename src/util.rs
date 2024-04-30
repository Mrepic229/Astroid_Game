use macroquad::prelude::*;
use core::f32::consts::PI;


pub fn get_random_offscree_pos() -> Vec2 {
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

pub fn get_distance(point1:Vec2,point2:Vec2) -> f32 {
    ((point1.x-point2.x).powf(2.0)+(point1.y-point2.y).powf(2.0)).sqrt()
}

pub fn get_angle(point1:Vec2, point2:Vec2) -> f32 {
    let mut rotation = ((point2.y-point1.y)/(point2.x-point1.x)).atan();
    if (point2.x- point1.x) < 0.0 {
        rotation += PI;
    }
    return rotation;
}

