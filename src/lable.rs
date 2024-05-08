use macroquad::prelude::*;

#[derive(Clone)]
pub struct lable {
    position: Vec2,
    offset: Vec2,
    rotation: f32,
    text: String,
}

impl lable {
    pub fn draw(&self) {
        let offset:Vec2 = Vec2 { x: self.rotation.cos()*self.offset.x, y: self.rotation.sin()*self.offset.y };
        let new_position:Vec2 = Vec2 { x: offset.x+self.position.x, y: offset.y+self.position.y };

        let size = measure_text(&self.text, None, 100, 1.0);
        draw_text(&self.text, x, y, font_size, color)
    }

}