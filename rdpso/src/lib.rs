use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use goal::Goal;
use goal::GoalSurface;

mod utils;
pub mod terrain;

pub mod pso;
pub mod goal;
pub mod space;

pub mod physics;

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct SimConfig {
    pub params: pso::ParameterSet,
    pub terrain: terrain::Config,
    pub controller: pso::ControllerConfig,
    pub swarm: pso::SwarmConfig,
    pub ctx: pso::Ctx,
    pub particle: pso::ParticleConfig,
}

#[wasm_bindgen]
impl SimConfig {
    pub fn new(
        params: pso::ParameterSet,
        terrain: terrain::Config,
        controller: pso::ControllerConfig,
        swarm: pso::SwarmConfig,
        ctx: pso::Ctx,
        particle: pso::ParticleConfig,
    ) -> Self {
        Self {
            params,
            terrain,
            controller,
            swarm,
            ctx,
            particle,
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct Simulator {
    swarm: pso::Swarm,
    terrain: terrain::Terrain,
    config: SimConfig,
    goal: Goal,
}

#[wasm_bindgen]
impl Simulator {

    pub fn new(config: SimConfig) -> Self {
        let terrain = terrain::Terrain::new(config.terrain);
        let controller = pso::particle::ParticleController::new_from_config(config.ctx, config.controller);
        let swarm = pso::Swarm::new(config.ctx,config.params, config.swarm, controller, config.particle, &terrain);
        
        Self {
            swarm: swarm,
            terrain: terrain,
            config: config,
            goal: config.ctx.goal,
        }
    }

    pub fn update_pso_params(&mut self, params: pso::ParameterSet) {
        self.config.params = params;
    }

    pub fn reset(&mut self) {
        let config = self.config;

        let terrain = terrain::Terrain::new(config.terrain);
        let controller = pso::particle::ParticleController::new_from_config(config.ctx, config.controller);
        let swarm = pso::Swarm::new(config.ctx,config.params, config.swarm, controller, config.particle, &terrain);
        self.swarm = swarm;
        self.terrain = terrain;
        self.goal = config.ctx.goal;
    }

    pub fn step(&mut self) {
        self.swarm.update(&self.terrain)
    }

    pub fn get_goal(&self) -> Goal{
        self.goal
    }

    pub fn get_goal_surface(&self) -> GoalSurface{
        GoalSurface::new(self.goal, self.config.terrain.size)
    }

    pub fn get_config(&self) -> SimConfig {self.config}
    
    pub fn dump_config(&self) -> String {
        serde_json::to_string_pretty(&self.config).unwrap()
    }

    // for some reason rebuilding a terrain on JS from the raw pointer isn't working
    // thus duplicate this method here
    pub fn parametric_terrain_eval(&self, u: f64, v: f64) -> space::Vector {
        self.terrain.get_point_from_parametric(u, v)
    }

    pub fn get_swarm_size(&self)  -> usize {self.config.swarm.size}

    pub fn get_particle_position_by_idx(&self, id: usize) -> space::Vector {
        self.swarm.get_position_by_idx(id)
    }

    pub fn set_config(&mut self, config: &str) -> Result<(), String> {
            let conf: SimConfig = serde_json::from_str(&config).map_err(|err| format!("invalid config: {}",err.to_string()))?;
            self.config = conf;
            Ok(())
    }
}


impl Simulator {
    pub fn get_swarm(&self) -> &pso::Swarm{
        &self.swarm
    }
}