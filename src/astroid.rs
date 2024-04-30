use macroquad::prelude::*;
use core::f32::consts::PI;

#[derive(Clone)]
pub struct Astroid {
    pub position: Vec2,
    pub points: Vec<Vec2>,
    pub radius: f32,
}

const BUMPYNESS: f32 = 20.0;
const ASTROID_SPEED: f32 = 1.0;

impl Astroid {
    pub fn new(pos: Vec2, radius: f32) -> Astroid {
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

    pub fn draw(&mut self) {
        self.points.push(self.points[0]);
        for i in 0..(self.points.len()-1) {
            let pos = self.position;
            let current_point = Vec2{x: self.points[i].x * self.points[i].y.cos(), y: self.points[i].x * self.points[i].y.sin()};
            let next_point = Vec2{x: self.points[i+1].x * self.points[i+1].y.cos(), y: self.points[i+1].x * self.points[i+1].y.sin()};
            draw_line(
                current_point.x + pos.x, current_point.y + pos.y,
                next_point.x + pos.x, next_point.y + pos.y,
                2.0, BLACK);
        }
        
    }



}