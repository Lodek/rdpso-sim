use crate::wasm_bindgen;

use crate::space::Vector;
use crate::terrain::Terrain;

/// LinearDetector sweeps a line to identify whether a collision happens
/// A collision is said to happen is the projection of direction vector placed 
/// on a position yields a y value higher than the original position
#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct LinearDetector {
    /// range indicates how far ahead the detector will check
    range: f64,

    /// step_size models the precision of the detector
    step_size: f64,

    step_count: usize,
}

impl LinearDetector {

    pub fn new(range: f64, step_size: f64) -> Self {
        Self {
            range: range,
            step_size: step_size,
            step_count: (range / step_size) as usize,
        }
    }

    /// new_from_count creates a detector which will verify range
    /// for a collision with step_count checks
    pub fn new_from_count(range: f64, step_count: usize) -> Self {
        Self::new(range, range / step_count as f64)
    }


    /// gen_collision returns the position of a collision detected by the detector
    /// if no collision was foudn returns None.
    pub fn get_collision(&self, position: Vector, direction: Vector, land: &Terrain) -> Option<Vector> {
        let direction = direction.unit();
        let step = self.step_size * direction;
        let mut position = Vector::new(position.x, position.y, position.z);

        for _ in 0..self.step_count {
            let y = land.get_height(position.x, position.z);
            if y >= position.y {
                return Some(Vector::new(position.x, y, position.z));
            }
            position = position + step;
        }
        None
    }

    pub fn has_collision(&self, position: Vector, direction: Vector, land: &Terrain) -> bool {
        self.get_collision(position, direction, land).is_some()
    }

}