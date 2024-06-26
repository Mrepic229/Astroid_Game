use macroquad::prelude::*;
use macroquad::rand::rand;
use core::f32::consts::PI;
use crate::bullet::Bullet;
use crate::util::{self, get_distance};

#[derive(Clone)]
pub struct Astroid {
    pub position: Vec2,
    pub points: Vec<Vec2>,
    pub radius: f32,
    pub speed: f32,
    pub color: Color,
}

const BUMPYNESS: f32 = 25.0;

impl Astroid {
    pub fn new(pos: Vec2, radius: f32, speed: f32 ) -> Astroid {
        let mut list_of_points:Vec<Vec2> = vec![];
        for i in 0..12{
            let point:Vec2 = Vec2 {x: radius - (fastrand::f32()*BUMPYNESS) + (BUMPYNESS/2.0), y: (PI * i as f32)/6.0};
            list_of_points.push(point);
        }

        let mut color = BLACK;
        if fastrand::i32(0..7) == 0 {
            color = GOLD;
        }

        let astroid: Astroid = Astroid {
            position: pos,
            points: list_of_points,
            radius: radius,
            speed: speed,
            color: color,
        };
        astroid
    }

    pub fn draw(&self) {
        let mut astroid = self.clone();
        astroid.points.push(self.points[0]);
        for i in 0..(astroid.points.len()-1) {
            let pos = astroid.position;
            let current_point = Vec2{x: astroid.points[i].x * astroid.points[i].y.cos(), y: astroid.points[i].x * astroid.points[i].y.sin()};
            let next_point = Vec2{x: astroid.points[i+1].x * astroid.points[i+1].y.cos(), y: astroid.points[i+1].x * astroid.points[i+1].y.sin()};
            draw_line(
                current_point.x + pos.x, current_point.y + pos.y,
                next_point.x + pos.x, next_point.y + pos.y,
                2.0, self.color);
        }
    }

    pub fn move_pos(&mut self, target: Vec2) {
        let rotation = util::get_angle(self.position, target);
        self.position += Vec2{x: rotation.cos(), y: rotation.sin()} * self.speed;
    }

    pub fn is_intersected(&mut self, bullets: Vec<Bullet>) -> bool {
        for i in bullets {
            if get_distance(self.position, i.position) < self.radius {
                return true;
            }
        }
        return false;
    }
}