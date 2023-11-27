use crate::wasm_bindgen;

use serde::{Serialize, Deserialize};

use crate::space::Vector;
use crate::goal::Goal;
use crate::goal::Strategy;

pub mod particle;

mod swarm;
pub use swarm::Swarm;

mod sensor;

/// Ctx models the problem context, with the evaluation function and the strategy
/// The Ctx is immutable for a run.
#[derive(Debug, Copy, Clone)]
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Ctx {
    pub goal: Goal,
    pub strategy: Strategy,
}

#[wasm_bindgen]
impl Ctx {
    pub fn new(goal: Goal, strategy: Strategy) -> Self {
        Self {
            goal,
            strategy,
        }
    }

    fn get_goal(&self) -> Goal { self.goal }

    fn get_strategy(&self) -> Strategy { self.strategy }
}

#[derive(Debug, Copy, Clone)]
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
/// ParameterSet sets the PSO algorithms parameters
pub struct ParameterSet {
    pub w: f64,
    pub c1: f64,
    pub c2: f64,
    pub c3: f64,
    pub max_velocity: f64,
}

#[wasm_bindgen]
impl ParameterSet{
    pub fn new(w: f64, c1: f64, c2: f64, c3: f64, max_velocity: f64) -> Self {
        Self {
            w,
            c1,
            c2,
            c3,
            max_velocity,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
/// SensorConfig sets the parameters for the collision detection sensor
pub struct SensorConfig {
    /// range speecifies the upper limit of how far the sensor will check
    pub range: f64,

    /// linear_step_size specifies the size of the steps the sensor will check until range
    pub linear_step_size: f64,

    /// fov_angle is the sensor's field of view, which sweeps a sector
    pub fov_angle: f64,

    /// angular_step_size models the size of the steps the sweep will take
    pub angular_step_size: f64,
}

#[wasm_bindgen]
impl SensorConfig{
    pub fn new(range: f64, linear_step_size: f64, fov_angle: f64, angular_step_size: f64) -> Self {
        Self {
            range,
            linear_step_size,
            fov_angle,
            angular_step_size,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct ControllerConfig {
    pub collision: SensorConfig,
}

#[wasm_bindgen]
impl ControllerConfig {

    pub fn new(sensor: SensorConfig) -> Self {
        Self {
            collision: sensor,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
#[derive(Serialize, Deserialize)]
pub struct SwarmConfig {
    /// number of particles
    pub size: usize,

    /// starting position
    pub deploy_position: Vector,

    /// how far appart should the particles be initially spread
    pub deploy_spread_radius: f64,

    /// magnitude of the swarm's particle's initial velocity
    pub initial_swarm_velocity: f64,
}

#[wasm_bindgen]
impl SwarmConfig {
    pub fn new(size: usize, deploy_position: Vector, deploy_spread_radius: f64, initial_swarm_velocity: f64) -> Self {
    Self {
        size,
        deploy_position,
        deploy_spread_radius,
        initial_swarm_velocity,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ParticleConfig {
    /// how many positions to keep in the position history buffer
    pub position_log_size: usize,
}

#[wasm_bindgen]
impl ParticleConfig {
    pub fn new(position_log_size: usize) -> Self {
        Self {
            position_log_size,
        }
    }

}