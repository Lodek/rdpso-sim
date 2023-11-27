use crate::{wasm_bindgen, physics::collision::LinearDetector, space::Boundary};

use circular_queue::CircularQueue;

use crate::space::Vector;
use super::{ParameterSet, ControllerConfig};
use crate::goal::Performance;
use super::Ctx;
use crate::utils::gen_random;
use super::sensor::CollisionSensor;
use crate::terrain::Terrain;
use std::f64::consts;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub struct ParticleState {
    /// p represents the current position vector
    p: Vector,

    /// best_performance models the solution the particle has encountered
    best_performance: Performance,

    /// score stores the latest value for the evaluation function
    score: f64,

    /// v is the current velocity vector
    v: Vector,

    positions: CircularQueue<Vector>,

    collisions: usize,

}

impl ParticleState {

    /// new initializes a particle
    pub fn new(p0: Vector, v0: Vector, history_amount: usize, controller: &ParticleController) -> Self {
        let f_result = controller.ctx.goal.evaluate(p0.x, p0.z);
        Self {
            p: p0,
            best_performance: Performance::new(p0, f_result),
            v: v0,
            score: f_result,
            positions: CircularQueue::with_capacity(history_amount),
            collisions: 0,
        }
    }


    pub fn get_best_performance(&self) -> Performance {
        self.best_performance
    }

    /// get_performance returns the particles current performance
    pub fn get_performance(&self) -> Performance {
        Performance::new(self.p, self.score)
    }
    pub fn get_position(&self) -> Vector {
        self.p.into()
    }

    pub fn get_particle_history(&self) -> impl Iterator<Item=&Vector> {
        self.positions.iter()
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct ParticleController {
    /// ctx describes the problem space the particle is exploring
    ctx: Ctx,

    sensor: CollisionSensor,
}

impl ParticleController{
    /// new initializes a particle
    pub fn new(ctx: Ctx, sensor: CollisionSensor) -> Self {
        Self {
            ctx: ctx,
            sensor: sensor,
        }
    }

    pub fn new_from_config(ctx: Ctx, config: ControllerConfig) -> Self {
        let linear_detector = LinearDetector::new(config.collision.range, config.collision.linear_step_size);
        let sensor = CollisionSensor::new(config.collision.fov_angle, config.collision.angular_step_size, linear_detector);
        Self::new(ctx, sensor)
    }


    /// update moves the particle to the next position according to the PSO kinematics equations
    pub fn update(&mut self, state: &mut ParticleState, g_best: Vector, params: &ParameterSet, boundary: &Boundary, terrain: &Terrain) {
        let mut v_prime = self.calc_new_velocity(g_best, params, state, terrain);
        let p_prime = state.p + v_prime;
        let mut p_prime = boundary.clip(&p_prime);

        if self.check_collision(p_prime, terrain) {
            v_prime = 0.5 * v_prime.rotate_xz(consts::PI / 2.0);
            state.collisions += 1;
            p_prime = state.p;
        } 
        
        let score = self.ctx.goal.evaluate(p_prime.x, p_prime.z);
        let current_performance = Performance::new(p_prime, score);

        state.p = p_prime;
        state.v = v_prime;
        state.score = score;
        state.best_performance = self.ctx.get_strategy().pick_best_performance(&state.best_performance, &current_performance);
        state.positions.push(p_prime);
    }

    fn check_collision(&self, pos: Vector, terrain: &Terrain) -> bool {
        let y = terrain.get_height(pos.x, pos.z);
        y >= pos.y 
    }

    /// get optimal_collision_position returns an "optimal" direction to follow in order to avoid collision
    /// the optimal opsition is given by the direction given by the sensor as being optimal
    /// displaced by the magnitude of the velocity vector
    /// 
    /// note this model is completely arbitrary
    fn get_optimal_collision_position(&self, state: &mut ParticleState, terrain: &Terrain) -> Vector {
        let direction = self.sensor.find_clear_direction(state.p, state.v, terrain);

        state.p + (state.v.magnitude() * direction)
    }

    fn calc_new_velocity(&self, g_best: Vector, params: &ParameterSet, state: &mut ParticleState, terrain: &Terrain) -> Vector {
        let (w, c1, c2, c3) = (params.w, params.c1, params.c2, params.c3);
        let (r1, r2, r3) = (gen_random(), gen_random(), gen_random());
        let (v, p, p_best) = (state.v, state.p, state.best_performance.position);
        let optimal_collision_pos = self.get_optimal_collision_position(state, terrain);

        let v = w * v 
        + c1 * r1 * (p_best - p) 
        + c2 * r2 * (g_best - p)
        + c3 * (optimal_collision_pos - p);

        if v.magnitude() > params.max_velocity {
            return params.max_velocity * v.unit();
        }
        v
    }
}
