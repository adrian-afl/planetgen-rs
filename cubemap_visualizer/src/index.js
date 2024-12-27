import * as THREE from 'three';
import {OrbitControls} from 'three/addons/controls/OrbitControls.js';

const renderer = new THREE.WebGLRenderer();
document.body.append(renderer.domElement);

renderer.setSize(document.body.clientWidth, document.body.clientHeight);

const scene = new THREE.Scene();

const cubetex = new THREE.CubeTextureLoader().load([
    'face_PX.png',
    'face_NX.png',
    'face_PY.png',
    'face_NY.png',
    'face_PZ.png',
    'face_NZ.png'
]);

const cubenormaltex = new THREE.CubeTextureLoader().load([
    'normal_face_PX.png',
    'normal_face_NX.png',
    'normal_face_PY.png',
    'normal_face_NY.png',
    'normal_face_PZ.png',
    'normal_face_NZ.png'
]);

scene.background = cubetex;

//scene.background = new THREE.Color("white");

const camera = new THREE.PerspectiveCamera(90, 1, 0.001, 100.0);
camera.position.set(0, 15, 15);
const controls = new OrbitControls(camera, renderer.domElement);

const uniforms = {
    cubeMap: {type: "samplerCube", value: cubetex},
    cubeNormalMap: {type: "samplerCube", value: cubenormaltex}
};

const material = new THREE.ShaderMaterial({
    uniforms: uniforms,
    vertexShader: `
           uniform float time;
        uniform samplerCube cubeMap;
        out vec3 norm;
        void main(){
            norm = normal;
            gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0 );
        }
        `,
    fragmentShader: `
        in vec3 norm;
        uniform samplerCube cubeNormalMap;
        void main(){
            vec3 n = texture(cubeNormalMap, norm).rgb;
            float dt = dot(n, vec3(0.0, 1.0, 0.0)) * 0.5 + 0.5;
            gl_FragColor = vec4(vec3(dt), 1.0);
        }`
});

const mesh = new THREE.Mesh(
    new THREE.IcosahedronGeometry(10, 500),
    material
)

scene.add(mesh);

scene.add(new THREE.DirectionalLight("white"))

function loop() {
    controls.update();
    renderer.render(scene, camera);
    requestAnimationFrame(loop);
}

requestAnimationFrame(loop);