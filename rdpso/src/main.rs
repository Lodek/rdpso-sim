use rdpso_sim::*;
use std::f64::consts;


fn main() {
    let config = SimConfig {
        params: pso::ParameterSet{
            w: 3.0,
            c1: 2.0,
            c2: 2.0,
            c3: 0.0,
            max_velocity: 10.0,
        },
        terrain: terrain::Config{
            size: 1000,
            octave_count: 4,
            octave_delta: 0.01,
            scaling_factor: 0.01,
        },
        controller: pso::ControllerConfig{
            collision: pso::SensorConfig{
                range: 10.0,
                linear_step_size: 0.5,
                fov_angle: consts::PI / 6.0,
                angular_step_size: consts::PI / 180.0,
            },
        },
        swarm: pso::SwarmConfig{
            size: 5,
            deploy_position: space::Vector::new(10.0, 10.0, 10.0),
            deploy_spread_radius: 10.0,
            initial_swarm_velocity: 0.5,
        },
        ctx: pso::Ctx::new(goal::Goal::Ackley, goal::Strategy::Minimize),
        particle: pso::ParticleConfig {
            position_log_size: 10,
        }
    };

    let mut simulator = Simulator::new(config);

    for i in 0..10 {
        simulator.step();
        for position in simulator.get_swarm().get_positions(){
            println!("{}: {:?}", i, position)
        }
    }

}
