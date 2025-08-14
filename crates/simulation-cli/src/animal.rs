use crate::*;
use ratatui::{
    style::Color,
    widgets::canvas::{Circle, Shape},
};

#[derive(Debug, Clone)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub size: f32,
}

impl From<&sim::Animal> for Animal {
    fn from(value: &sim::Animal) -> Self {
        Self {
            x: value.position().x,
            y: value.position().y,
            rotation: value.rotation().angle(),
            size: value.size(),
        }
    }
}

impl Shape for Animal {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        Circle {
            x: self.x as f64,
            y: self.y as f64,
            radius: self.size as f64,
            color: Color::Red,
        }
        .draw(painter);
    }
}
