import initWasm from "rdpso-sim";
import * as sim from "rdpso-sim";

function getConfig() {
    const w = 0.5;
    const c1 = 2.0;
    const c2 = 1.0;
    const c3 = 1.0;
    const max_vel = 5.0;
    const psoParams = sim.ParameterSet.new(w, c1, c2, c3, max_vel);

    const size = 1000;
    const octaves = 5;
    const octave_delta = 0.01;
    const scaling_factor = 0.012;
    const terrainConfig = sim.Config.new(size, octaves, octave_delta, scaling_factor);

    const range = 17.0;
    const linear_step_size = 0.5;
    const fov_angle = Math.PI / 6;
    const angular_step_size = Math.PI / 180;
    const sensor = sim.SensorConfig.new(range, linear_step_size, fov_angle, angular_step_size);
    const controller = sim.ControllerConfig.new(sensor);

    const swarmSize = 5;
    const deployPos = sim.Vector.new(450, 75, 450);
    const deploySpread = 10.0;
    const v0 = 0.1;
    const swarm = sim.SwarmConfig.new(swarmSize, deployPos, deploySpread, v0);

    const ctx = sim.Ctx.new(sim.Goal.Griewank, sim.Strategy.Minimize);

    const particleLog = 20;
    const particle = sim.ParticleConfig.new(particleLog);

    const config = sim.SimConfig.new(psoParams, terrainConfig, controller, swarm, ctx, particle);

    return config;
}

export async function buildSimulator() {
    return initWasm().then(() => {
        const conf = getConfig();
        const simulator =  sim.Simulator.new(conf);
        return simulator;
    })
}