mod perlin;

pub use perlin::PerlinNoise;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(not(target_family = "wasm"))]
use rand::Rng;

/// gen_random generates a random float in the interval [0, 1]
pub fn gen_random() -> f64 {
    #[cfg(target_family = "wasm")]
    return js_sys::Math::random();

    #[cfg(not(target_family = "wasm"))]
    return rand::thread_rng().gen();
}

pub fn gen_random_range(min: usize, max: usize) -> usize {
    let min = min as f64;
    let max = max as f64;
    let delta = max - min;
    let random = gen_random();
    (delta * random - min) as usize
}