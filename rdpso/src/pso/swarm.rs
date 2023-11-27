use std::f64::consts;

use crate::wasm_bindgen;

use crate::space::Vector;
use crate::goal::Performance;
use super::ParameterSet;
use super::Ctx;
use super::particle::ParticleController;
use super::particle::ParticleState;
use crate::utils::gen_random;
use super::SwarmConfig;
use super::ParticleConfig;
use crate::terrain::Terrain;
use crate::space::Boundary;

#[derive(Debug)]
#[wasm_bindgen]
pub struct Swarm {
    ctx: Ctx,
    params: ParameterSet,
    controller: ParticleController,
    population: Vec<ParticleState>,
    best: Performance,
    historic_best: Performance,
    positions: Vec<Vector>,
    iteration: u64,
    space_boundary: Boundary,
}

#[wasm_bindgen]
impl Swarm {

    pub fn new(ctx: Ctx,
        params: ParameterSet, 
        config: SwarmConfig,
        controller: ParticleController,
        particle_config: ParticleConfig,
        terrain: &Terrain,
    ) -> Self {
        let size = config.size;

        if size == 0 {
            panic!("population size must be > 0")
        }

        let unit = Vector::unit_x();

        let mut population = Vec::with_capacity(size);
        let mut positions = Vec::with_capacity(size);

        // initialize particles
        for i in 0..size {
            let rotation_angle = consts::TAU * gen_random();
            let magnitude = config.deploy_spread_radius * gen_random();

            let rotation_vec = unit.rotate_xz(rotation_angle);
            let spread = magnitude * rotation_vec;
            let start_position = config.deploy_position + spread;

            let v0 = config.initial_swarm_velocity * rotation_vec;

            population.push(ParticleState::new(start_position, v0, particle_config.position_log_size, &controller));
        }

        // initialize position vectors
        for i in 0..size {
            positions.push(Vector::new(0.0, 0.0, 0.0));
        }
        let initial_perf = population[0].get_performance();

        let mut swarm = Self {
            params: params,
            population: population,
            controller: controller,
            positions: positions,
            historic_best: initial_perf.clone(),
            best: initial_perf.clone(),
            ctx: ctx,
            iteration: 0,
            space_boundary: terrain.get_boundary(),
        };

        swarm.update_positions();
        swarm.update_bests();
        swarm
    }

    pub fn update(&mut self, terrain: &Terrain) {
        for particle in self.population.iter_mut() {
            self.controller.update(particle, self.best.position, &self.params, &self.space_boundary, terrain);
        }
        self.update_positions();
        self.update_bests();
        self.iteration += 1;
    }

    fn update_positions(&mut self) {
        for (i, particle) in self.population.iter().enumerate() {
            self.positions[i] = particle.get_position();
        }
    }

    /// update_bests finds the new current best performance,
    /// sets it as bests and updates the historic best if necessary
    fn update_bests(&mut self) {
        self.best = self.find_best();

        let historic_best = self.ctx.get_strategy().pick_best_performance(&self.best, &self.historic_best);
        self.historic_best = historic_best;
    }

    /// find_best returns the score of the currently best performing particle
    fn find_best(&self) -> Performance {
        let mut best = self.population[0].get_performance();

        for particle in self.population[1..].iter() {
            let performance = particle.get_performance();
            best = self.ctx.get_strategy().pick_best_performance(&best, &performance);
        }
        best
    }

    pub fn set_params(&mut self, params: ParameterSet) {
        self.params = params
    }

    pub fn get_best(&self) -> Performance {
        self.best
    }

    pub fn get_historic_best(&self) -> Performance {
        self.historic_best
    }

    pub fn get_population_size(&self) -> usize {
        self.positions.len()
    }

    /// get_position_by_idx returns a pointer to the idxth particle from
    /// the contiguous memory of the underlying Vec.
    /// 
    /// Note this method is primarily used for the WASM wrapper in order
    /// to directly access the Vector with a copy.
    pub fn get_position_ptr_by_idx(&self, idx: usize) -> *const Vector {
        if idx >= self.positions.len() {
            panic!("idx must be less than {}", self.positions.len());
        }
        self.positions.get(idx).unwrap() as *const Vector
    }

    pub fn get_position_by_idx(&self, idx: usize) -> Vector {
        if idx >= self.positions.len() {
            panic!("idx must be less than {}", self.positions.len());
        }
        self.positions[idx]
    }
}

impl Swarm {
    pub fn get_positions(&self) -> &Vec<Vector> {
        &self.positions
    }
}