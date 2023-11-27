use crate::wasm_bindgen;
use serde::{Serialize, Deserialize};

use crate::space::Vector;
use super::space::Mapper;
use super::space::PiecewieseInterpolator;
use crate::utils::PerlinNoise;
use crate::space::Boundary;

/// Config specifies Terrain configuarion parameters
#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Config {
    /// size represents the terrain boundary
    pub size: usize,

    /// octave_count represent the amount of times the noise function will be sampled per x, z coord
    pub octave_count: usize,

    /// octave_delta configures the sampling distance used between each different perlin sampling (octave)
    pub octave_delta: f64,

    /// scaling_factory determines how much the x and y should be scaled down before being sampled
    pub scaling_factor: f64,

}

#[wasm_bindgen]
impl Config {
    pub fn new(size: usize, octave_count: usize, octave_delta: f64, scaling_factor: f64) -> Self{
        Self {
            size,
            scaling_factor,
            octave_count,
            octave_delta
        }

    }

    /// new_from_size returns with some predefined defaults
    pub fn new_from_size(size: usize) -> Self {
        let octaves = 4;
        let octave_delta = 0.1;
        Self::new(size, octaves, octave_delta, 0.01)
    }

}

/// Terrain is a parametric land generator using Perlin noise
#[wasm_bindgen]
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Terrain {
    config: Config,
    noise: PerlinNoise,
    /// parametric_mapper maps parametric coordinates into the world size range
    parametric_mapper: Mapper,
    sampling_mapper: Mapper,
    interpolator: PiecewieseInterpolator,
    boundary: Boundary,
}

#[wasm_bindgen]
impl Terrain {
    pub fn new(config: Config) -> Self {
        let interpolator = PiecewieseInterpolator::new(
            vec![0.0, 0.25, 0.50, 1.0],
            vec![0.0, 0.0, 30.0,  500.0],
        ).unwrap();

        let offset = (config.size/2) as f64;
        let parametric_mapper = Mapper::new_from_pair([0.0, 1.0], [-offset, offset]);
        let sampling_mapper = Mapper::new_from_pair([-offset, offset], [0.0, config.size as f64]);
        let boundary = Boundary::new(-offset, offset, -offset, offset);
        Self {
            config: config,
            noise: PerlinNoise::new(),
            parametric_mapper,
            interpolator,
            boundary: boundary,
            sampling_mapper: sampling_mapper,
        }
    }

    pub fn get_height(&self, x: f64, z: f64) -> f64 {
        let scaling = self.config.scaling_factor;
        let x = self.sampling_mapper.map(x) * scaling;
        let z = self.sampling_mapper.map(z) * scaling;

        let mut sample_y = self.config.octave_delta;
        let mut y = 0.0;
        for _ in 0..self.config.octave_count {
            y += self.noise.get3d([x, sample_y, z]);
            sample_y += self.config.octave_delta;
        }

        y = y / self.config.octave_count as f64;

        self.interpolator.map(y).unwrap()
    }

    pub fn get_point_from_parametric(&self, x: f64, z: f64) -> Vector {
        let x = self.parametric_mapper.map(x);
        let z = self.parametric_mapper.map(z);
        let y = self.get_height(x, z);

        Vector{
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn get_boundary(&self) -> Boundary {
        self.boundary
    }

    pub fn get_size(&self) -> usize {self.config.size}

    pub fn get_config(&self) -> Config {self.config}
}