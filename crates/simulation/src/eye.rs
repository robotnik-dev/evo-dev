use std::f32::consts::{FRAC_PI_4, PI};

use crate::Food;

/// How far the eye can see in percentage of the entire map
const FOV_RANGE: f32 = 0.25;

/// How wide the eye can see in radians
const FOV_ANGLE: f32 = FRAC_PI_4;

/// How many cells the eye has
const CELLS: usize = 8;

#[derive(Debug, Clone)]
pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}

impl Eye {
    pub fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self,
        position: nalgebra::Point2<f32>,
        rotation: nalgebra::Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];
        for food in foods {
            let vec = food.position - position;
            let dist = vec.norm();
            if dist > self.fov_range {
                continue;
            }

            let angle =
                nalgebra::Rotation2::rotation_between(&nalgebra::Vector2::y(), &vec).angle();
            let angle = angle - rotation.angle();
            let angle = nalgebra::wrap(angle, -PI, PI);
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            let angle = angle + self.fov_angle / 2.0;
            let cell = angle / self.fov_angle;
            let cell = cell * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);
            let energy = (self.fov_range - dist) / self.fov_range;
            cells[cell] += energy;
        }
        cells
    }
}
