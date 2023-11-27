use crate::wasm_bindgen;
use ka::SingleObjective;
use serde::{Serialize, Deserialize};

use super::space::Vector;
use super::space::Mapper;

#[derive(Debug, PartialEq,Copy, Clone, PartialOrd)]
#[wasm_bindgen]
pub struct Performance {
    pub score: f64,
    pub position: Vector,
}

#[wasm_bindgen]
impl Performance {
    pub fn new(position: Vector, score: f64) -> Self {
        Self {
            position,
            score,
        }
    }

    pub fn get_score(&self) -> f64{
        self.score
    }

    pub fn get_position(&self) -> Vector {
        self.position
    }
}

#[derive(Debug, PartialEq,Copy, Clone)]
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub enum Strategy {
    Maximize = 0,
    Minimize = 1,
}

impl Strategy {

    /// pick_best chooses between two values, based on which one is more desireable
    /// for the current strategy.
    /// ie if Minimize pick the smallest, if maximize picks the biggest
    pub fn pick_best(&self, left: f64, right: f64) -> f64 {
        match self {
            Self::Maximize => {
                if left > right {
                    left
                } else {
                    right
                }
            },
            Self::Minimize => {
                if left < right {
                    left
                } else {
                    right
                }
            },
        }
    }

    pub fn pick_best_performance(&self, left: &Performance, right: &Performance) -> Performance {
        match self {
            Self::Maximize => {
                if left.get_score() > right.get_score() {
                    *left
                } else {
                    *right
                }
            },
            Self::Minimize => {
                if left.get_score() < right.get_score() {
                    *left
                } else {
                    *right
                }
            },
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
#[derive(Serialize, Deserialize)]
/// Goal represents the Swarm's goal function, the task to be accomplished
pub enum Goal {
    Ackley,
    Griewank,
}

impl Goal {
    /// evaluate takes a vector and produces a score for the vector
    pub fn evaluate(&self, x: f64, z: f64) -> f64 {
        match self {
            Self::Ackley => self.ackley(x, z),
            Self::Griewank => self.griewank(x, z),
        }
    }
    
    fn griewank(&self, x: f64, z: f64) -> f64 {
        ka::single::Griewank::f(vec![x, z])
    }

    fn ackley(&self, x: f64, z: f64) -> f64 {
        ka::single::Ackley::f(vec![x, z])
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct GoalSurface {
    goal: Goal,
    parametric_mapper: Mapper,
}

#[wasm_bindgen]
impl GoalSurface{
    pub fn new(goal: Goal, domain_size: usize,) -> Self {
        let half_domain = (domain_size / 2) as f64;
        Self {
             goal: goal,
             parametric_mapper: Mapper::new(0.0, 1.0, -half_domain, half_domain),
        }
    }

    pub fn eval(&self, x: f64, z: f64) -> f64 {
        self.goal.evaluate(x, z)
    }

    /// parametric_eval evaluates the goal function for parametric variables in the range [0,1]
    pub fn parametric_eval(&self, x: f64, z: f64) -> Vector {
        let x = self.parametric_mapper.map(x);
        let z = self.parametric_mapper.map(z);
        let y = self.goal.evaluate(x, z);
        Vector::new(x, y, z)
    }
}