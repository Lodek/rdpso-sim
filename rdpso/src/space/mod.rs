use crate::wasm_bindgen;

mod vector;
pub use vector::Vector;


pub type Pair = [f64; 2];

pub type Triplet = [f64; 3];

#[derive(Debug, PartialEq,Copy, Clone)]
#[wasm_bindgen]
pub struct Point2D {
    pub a: f64,
    pub b: f64,
}

impl Point2D {
    pub fn new(a: f64, b: f64) -> Self {
        Self {
            a, b
        }
    }
}


#[derive(Debug, PartialEq,Copy, Clone)]
#[wasm_bindgen]
pub struct Boundary {
    min_x: f64,
    max_x: f64,
    min_z: f64,
    max_z: f64,
}

#[wasm_bindgen]
impl Boundary {

    pub fn new(mut min_x: f64, mut max_x: f64, mut min_z: f64, mut max_z: f64) -> Self {
        if min_x > max_x{
            (min_x, max_x) = (max_x, min_x);
        }

        if min_z > max_z{
            (min_z, max_z) = (max_z, min_z);
        }

        Self{
            min_x: min_x,
            max_x: max_x,
            min_z: min_z,
            max_z: max_z,
        }
    }

    /// clip receives a vector and clips it to within the threshold
    /// that is, if one of vec's components is outside the threshold
    /// that coordinate is clipped to the boundaries defined by the region vectors
    pub fn clip(&self, vec: &Vector) -> Vector {
        let mut vec = vec.clone();

        if vec.x < self.min_x {
            vec.x = self.min_x;
        } else if vec.x > self.max_x {
            vec.x = self.max_x;
        }

        if vec.z < self.min_z {
            vec.z = self.min_z;
        } else if vec.z > self.max_z {
            vec.z = self.max_z;
        }

        vec
    }
}


/// Mapper maps one interval onto another
#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Mapper {
    start1: f64,
    stop1: f64,
    start2: f64,
    stop2: f64,
    delta1: f64,
    delta2: f64,
}

#[wasm_bindgen]
impl Mapper {

    pub fn new(start1: f64, stop1: f64, start2: f64, stop2: f64) -> Self {
        Self {
            start1: start1,
            stop1: stop1,
            start2: start2,
            stop2: stop2,
            delta1: stop1 - start1,
            delta2: stop2 - start2,
        }
    }

    pub fn map(&self, x:f64) -> f64 {
        ( self.delta2 * (x - self.start1) / self.delta1 ) + self.start2
    }
}

impl Mapper {
    pub fn new_from_pair(from: Pair, to: Pair) -> Self {
        Self {
            start1: from[0],
            stop1: from[1],
            start2: to[0],
            stop2: to[1],
            delta1: from[1] - from[0],
            delta2: to[1] - to[0],
        }
    }
}

/// Domain represents an interval bound domain, from a to b
#[wasm_bindgen]
#[derive(Copy, Debug, Clone)]
pub struct Domain {
    a: f64,
    b: f64,
}

impl Domain {

    pub fn new(a: f64, b: f64) -> Self {
        Self { a: a, b: b }
    }

    pub fn contains(&self, x: f64) -> bool {
        if x >= self.a && x <= self.b {
            true
        } else {
            false
        }
    }
}


#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct PiecewieseInterpolator {
    domains: Vec<Domain>,
    mappers: Vec<Mapper>,
}

#[wasm_bindgen]
impl PiecewieseInterpolator {
    
    pub fn new(from: Vec<f64>, to: Vec<f64>) -> Option<PiecewieseInterpolator> {
        if from.len() != to.len() {
            return None;
        }

        let mut domains = Vec::with_capacity(from.len() - 1);
        let mut mappers = Vec::with_capacity(from.len() - 1);

        for i in 0..(from.len() -1) {
            let domain = Domain::new(from[i], from[i+1]);
            let mapper = Mapper::new(from[i], from[i+1], to[i], to[i+1]);
            domains.push(domain);
            mappers.push(mapper);
        }

        Some(
            Self{
                mappers: mappers,
                domains: domains,
            }
        )
    }

    /// map interpolates value v according to the piecewise interpolation
    /// if v is not part of the domain, returns None
    pub fn map(&self, v: f64) -> Option<f64> {
        for (domain, mapper) in Iterator::zip(self.domains.iter(), self.mappers.iter()) {
            if domain.contains(v) {
                return Some(mapper.map(v))
            }
        }
        None
    }

}