use ratatui::widgets::canvas::Shape;

#[derive(Debug, Clone)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

impl From<&simulation::Food> for Food {
    fn from(value: &simulation::Food) -> Self {
        Self {
            x: value.position().x,
            y: value.position().y,
        }
    }
}

impl Shape for Food {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        if let Some((x, y)) = painter.get_point(self.x as f64, self.y as f64) {
            painter.paint(x, y, ratatui::style::Color::Blue);
        }
    }
}
