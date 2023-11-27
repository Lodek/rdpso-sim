import * as THREE from 'three';
import {Vector, Swarm} from "rdpso-sim";

const PARTICLE_COLOR = 0xff0000;
const particle_geometry = new THREE.OctahedronGeometry(3.0);
const particle_material = new THREE.MeshBasicMaterial( { color: PARTICLE_COLOR, wireframe: false } );

export function step(sim, particles) {
    sim.step();

    for (let j=0; j < sim.get_swarm_size(); j++) {
        //let ptr = sim.get_particle_position_ptr_by_idx(j);
        //let position = Vector.__wrap(ptr);
        let position = sim.get_particle_position_by_idx(j);
		particles[j].position.x = position.get_x();
		particles[j].position.y = position.get_y();
		particles[j].position.z = position.get_z();
    }
}

export function draw_particles(scene, sim) {
    const particles = []

    for (let i=0; i < sim.get_swarm_size(); i++) {
        let position = sim.get_particle_position_by_idx(i);
        //let ptr = sim.get_particle_position_ptr_by_idx(i);
        //let position = Vector.__wrap(ptr);
        let particle_mesh = new THREE.Mesh(particle_geometry, particle_material);
        particle_mesh.position.x = position.x;
        particle_mesh.position.y = position.y;
        particle_mesh.position.z = position.z;
        scene.add(particle_mesh);
        particles.push(particle_mesh);
    }

    return particles;
}