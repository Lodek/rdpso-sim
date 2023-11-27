import * as THREE from 'three';
import { ParametricGeometry } from 'three/addons/geometries/ParametricGeometry.js';
import { Lut } from 'three/addons/math/Lut.js';

import {Terrain} from "rdpso-sim"


const goal_material = new THREE.MeshBasicMaterial({ color: 0x0000ff, wireframe: true });
const terrain_material = new THREE.MeshBasicMaterial({ color: 0xaaffaa, wireframe: true });

export function draw_goal_surface(scene, sim) {
    const surface = sim.get_goal_surface();

    let f = (u, v, vec) => {
        let point = surface.parametric_eval(u, v);
        vec.set(point.x, point.y, point.z);
    }
    const map_geometry = new ParametricGeometry(f, 250, 250);

    let mesh = new THREE.Mesh(map_geometry, goal_material);

    scene.add(mesh);
    return mesh;
}

export function draw_terrain(scene, sim) {
    let f = (u, v, vec) => {
        let point = sim.parametric_terrain_eval(u, v);
        vec.set(point.x, point.y, point.z);
    }

    const map_geometry = new ParametricGeometry(f, 250, 250);
    let mesh = new THREE.Mesh( map_geometry, terrain_material );
    scene.add(mesh);
    return mesh;
}