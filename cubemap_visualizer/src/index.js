import * as THREE from 'three';
import {OrbitControls} from 'three/addons/controls/OrbitControls.js';

const renderer = new THREE.WebGLRenderer();
document.body.append(renderer.domElement);

renderer.setSize(document.body.clientWidth, document.body.clientHeight);

const scene = new THREE.Scene();

scene.background = new THREE.CubeTextureLoader().load([
    'face_PX.png',
    'face_NX.png',
    'face_PY.png',
    'face_NY.png',
    'face_PZ.png',
    'face_NZ.png'
]);


const camera = new THREE.PerspectiveCamera(90, 1, 0.001, 100.0);
camera.position.set(0, 20, 100);
const controls = new OrbitControls(camera, renderer.domElement);

function loop() {
    controls.update();
    renderer.render(scene, camera);
    requestAnimationFrame(loop);
}

requestAnimationFrame(loop);