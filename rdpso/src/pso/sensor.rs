use crate::physics::collision::LinearDetector;
use crate::space::Vector;
use crate::terrain::Terrain;

#[derive(Debug, Copy, Clone)]
/// Collision Sensor models a sensor which detects inbound collisions
pub struct CollisionSensor {
    /// detection_angle models the field of vision of the detector
    detection_angle: f64,

    /// angular_step_size models the angular increments the detector takes while sweeping across the arc
    angular_step_size: f64,

    detector: LinearDetector,
}

impl CollisionSensor {

    pub fn new(detection_angle: f64,  angular_step_size: f64, detector: LinearDetector) -> Self {
        Self {
            detection_angle: detection_angle,
            angular_step_size: angular_step_size,
            detector: detector,
        }
    }


    /// find_clear_direction returns a unit vector pointing to a direction with no obstacles in the Sensor FOV
    /// if no clear path is found, returns the leftmost direction seen by the sensor
    pub fn find_clear_direction(&self, pos: Vector, direction: Vector, land: &Terrain) -> Vector {
        let direction = direction.unit();
        let clear = self.bissect(pos, direction, self.detection_angle, land);
        if let Some(clear) = clear {
            return clear;
        }

        direction.rotate_xz(-self.detection_angle/2.0)
    }

    /// bissect bissects a view cone
    /// it tests whether there's an inbound collision at the given direction,
    /// if there is rotate 1/4 of the FOV to the left and right and try again.
    /// recurses until a clear direction is found or the bissection angle becomes less than
    /// the step_size
    fn bissect(&self, pos: Vector, direction: Vector, angle: f64, land: &Terrain) -> Option<Vector> {
        let center_clear = self.detector.has_collision(pos, direction, land);
        if center_clear {
            return Some(direction);
        }

        let bissection = angle / 2.0;
        let rotation_angle = bissection / 2.0;
        if bissection < self.angular_step_size {
            return None;
        }

        let left = direction.rotate_xz(-rotation_angle);
        let left_clear = self.bissect(pos, left, bissection, land);
        if left_clear.is_some() {
            return left_clear;
        }

        let right = direction.rotate_xz(rotation_angle);
        self.bissect(pos, right, bissection, land)
    }
}