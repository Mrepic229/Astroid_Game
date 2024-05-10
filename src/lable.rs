use macroquad::prelude::*;

#[derive(Clone)]
pub struct TextLable {
    pub position: Vec2,
    pub offset: Vec2,
    pub rotation: f32,
    pub text: String,
    pub size: f32,
    pub color: Color,
}

impl TextLable {
    pub fn draw(&self) {
        let new_position:Vec2 = Vec2 { x: self.offset.x+self.position.x, y: self.offset.y+self.position.y };

        let size: TextDimensions = measure_text(&self.text, None, self.size as u16, 1.0);
        
        draw_text(&self.text, new_position.x-(size.width/2.0), new_position.y-(size.height/2.0), self.size, self.color)
    }

}