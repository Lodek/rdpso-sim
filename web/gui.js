import { GUI } from 'three/addons/libs/lil-gui.module.min.js';
import {draw_terrain, draw_goal_surface} from "./surface.js";
import {draw_particles} from "./swarm.js";


export function build_menu(sim, scene, meshes) {

    const menu = {
        Reset: () => {
            scene.clear();

            sim.reset();

            const goal = draw_goal_surface(scene, sim);
            goal.visible = meshes.goal.visible;
            meshes.goal = goal;

            const terrain = draw_terrain(scene, sim);
            terrain.visible = meshes.terrain.visible;
            meshes.terrain = terrain;

            meshes.particles = draw_particles(scene, sim);
        },
        Apply: () => {
            const config = document.querySelector("#config").value;

            try {
                sim.set_config(config);
                document.querySelector("#config-result").textContent = "";
            } catch (error) {
                console.log(error)
                document.querySelector("#config-result").textContent = error;
            }
        },
        Goal: true,
        Terrain: true,
    };
    const gui = new GUI();

    gui.add(menu, "Reset");
    gui.add(menu, "Apply");
    gui.add(menu, "Goal").onChange(() => meshes.goal.visible = !meshes.goal.visible);
    gui.add(menu, "Terrain").onChange(() => meshes.terrain.visible = !meshes.terrain.visible);

    return gui
}