use macroquad::prelude::*;

#[derive(Clone)]
pub struct TextLable {
    pub position: Vec2,
    pub offset: Vec2,
    pub rotation: f32,
    pub text: String,
}

impl TextLable {
    pub fn draw(&self) {
        let offset:Vec2 = Vec2 { //x: self.rotation.cos()*self.offset.x, y: self.rotation.sin()*self.offset.y };
        x: 0.0,
        y: 0.0};
        let new_position:Vec2 = Vec2 { x: offset.x+self.position.x, y: offset.y+self.position.y };

        let size: TextDimensions = measure_text(&self.text, None, 100, 1.0);
        draw_text(&self.text, (size.width/2.0)+new_position.x, (size.height/2.0)+new_position.y, 100.0, BLACK)
    }

}