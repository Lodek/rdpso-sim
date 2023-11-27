import {scene, redraw} from "./space.js";
import {buildSimulator} from "./simulator.js";
import {draw_terrain, draw_goal_surface} from "./surface.js";
import {step, draw_particles} from "./swarm.js";
import {build_menu} from "./gui.js";

buildSimulator().then( (sim) => {
    document.querySelector("#config").value = sim.dump_config();

    let meshes = {
        goal: draw_goal_surface(scene, sim),
        terrain: draw_terrain(scene, sim),
        particles: draw_particles(scene, sim),
    };

    build_menu(sim, scene, meshes);

    function animate() {
        step(sim, meshes.particles);

        redraw();

        setTimeout(function () {
            requestAnimationFrame( animate );

        }, 1000 / 30);
    }
    animate();
});
