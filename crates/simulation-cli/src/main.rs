mod animal;
mod food;
mod world;

use std::time::Duration;

pub use animal::*;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
pub use food::*;
use nalgebra::clamp;
use rand::{rng, rngs::ThreadRng};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Widget, canvas::Canvas},
};
use simulation::{self as sim, Stats};
pub use world::*;

type Result = std::io::Result<()>;

const MIN_TICK_RATE: f32 = 1.0;
const MAX_TICK_RATE: f32 = 240.0;

#[derive(Debug)]
pub struct App {
    sim: sim::Simulation,
    rng: ThreadRng,
    tick_rate: f32,
    age: usize,
    stats: Stats,
    exit: bool,
}

impl App {
    pub fn new(tick_rate: f32) -> Self {
        let mut rng = rng();
        let sim = sim::Simulation::random(&mut rng);
        Self {
            sim,
            rng,
            tick_rate,
            age: 0,
            stats: Stats {
                avg_fitness: 0.0,
                min_fitness: 0.0,
                max_fitness: 0.0,
            },
            exit: false,
        }
    }

    pub fn world(&self) -> World {
        World::from(self.sim.world())
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            if let Some(stats) = self.sim.step(&mut self.rng) {
                self.age += 1;
                self.stats = stats;
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Percentage(10),
            Constraint::Percentage(90),
            Constraint::Percentage(10),
        ]);
        let [top, mid, bot] = vertical.areas(frame.area());
        frame.render_widget(self.render_stats(), top);
        frame.render_widget(self.render_world(), mid);
        frame.render_widget(self.render_instructions(), bot);
    }

    fn handle_events(&mut self) -> Result {
        let timeout = Duration::from_secs_f32(1.0 / self.tick_rate);
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Up => self.faster(),
                        KeyCode::Down => self.slower(),
                        KeyCode::Right => self.train_generation(),
                        KeyCode::Char('r') => self.restart(),
                        KeyCode::Char('q') | KeyCode::Esc => self.exit(),
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn render_world(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::bordered().title("World"))
            .paint(|ctx| {
                for food in self.world().foods {
                    ctx.draw(&food);
                }
                ctx.layer();
                for animal in self.world().animals {
                    ctx.draw(&animal);
                }
            })
            .x_bounds([0.0, 1.0])
            .y_bounds([0.0, 1.0])
            .marker(ratatui::symbols::Marker::Braille)
    }
    fn render_stats(&self) -> impl Widget + '_ {
        let block = Block::bordered().title("Stats");
        Paragraph::new(format!(
            "Generation: {}, Day: {}, avg: {}, min: {}, max: {}",
            self.age,
            self.sim.age,
            self.stats.avg_fitness,
            self.stats.min_fitness,
            self.stats.max_fitness
        ))
        .block(block)
    }

    fn render_instructions(&self) -> impl Widget + '_ {
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<q> ".blue().bold(),
            " Restart ".into(),
            "<r> ".blue().bold(),
            " Faster ".into(),
            "<↑> ".blue().bold(),
            " Slower ".into(),
            "<↓> ".blue().bold(),
        ]);
        let block = Block::bordered().title("Controls");
        Paragraph::new(instructions.centered()).block(block)
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn restart(&mut self) {
        self.sim = sim::Simulation::random(&mut self.rng);
    }

    fn faster(&mut self) {
        self.tick_rate = clamp(self.tick_rate + 10.0, MIN_TICK_RATE, MAX_TICK_RATE);
    }

    fn slower(&mut self) {
        self.tick_rate = clamp(self.tick_rate - 10.0, MIN_TICK_RATE, MAX_TICK_RATE);
    }

    fn train_generation(&mut self) {
        let stats = self.sim.train(&mut self.rng);
        self.stats = stats;
        self.age += 1;
    }
}

fn main() -> Result {
    let mut terminal = ratatui::init();
    let app_result = App::new(MAX_TICK_RATE / 2.0).run(&mut terminal);
    ratatui::restore();
    app_result
}
