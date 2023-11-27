import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
import Stats from 'three/addons/libs/stats.module.js';


export const scene = new THREE.Scene();
scene.background = new THREE.Color(0xaaaaaa);

let app = document.querySelector("#app");
let stats = new Stats();
app.appendChild(stats.dom);

const camera = new THREE.PerspectiveCamera( 75, window.innerWidth / window.innerHeight, 0.1, 1000 );
camera.position.set( 0, 800, 0 );

const renderer = new THREE.WebGLRenderer({
  canvas: document.querySelector("#canvas"),
});
renderer.setSize( window.innerWidth, window.innerHeight );


const controls = new OrbitControls( camera, renderer.domElement );

controls.update();


export function redraw() {
	// required if controls.enableDamping or controls.autoRotate are set to true
	controls.update();
	stats.update();
	renderer.render( scene, camera );
}