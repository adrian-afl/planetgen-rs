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
        uniform samplerCube cubeNormalMap;
        out vec3 norm;
        mat3 axisAngleMat3(vec3 axis, float angle)
        {
          axis = normalize(axis);
          float s = sin(angle);
          float c = cos(angle);
          float oc = 1.0 - c;
        
          return mat3(oc * axis.x * axis.x + c, oc * axis.x * axis.y - axis.z * s, oc * axis.z * axis.x + axis.y * s,
          oc * axis.x * axis.y + axis.z * s, oc * axis.y * axis.y + c, oc * axis.y * axis.z - axis.x * s,
          oc * axis.z * axis.x - axis.y * s, oc * axis.y * axis.z + axis.x * s, oc * axis.z * axis.z + c);
        }
        vec3 getNormal(float dxrange, vec3 dir){
            vec3 tangdir = dir * axisAngleMat3(vec3(0.0, 1.0, 0.0), 3.1415); // wtf is going on here
            vec3 bitangdir = normalize(cross(tangdir, dir));
            tangdir = normalize(cross(dir, bitangdir));
            mat3 normrotmat1 = axisAngleMat3(tangdir, dxrange);
            mat3 normrotmat2 = axisAngleMat3(bitangdir, dxrange);
            vec3 dir2 = normrotmat1 * dir;
            vec3 dir3 = normrotmat2 * dir;
            vec3 p1 = dir * (1.0 + 0.1 * texture(cubeMap, dir).r);
            vec3 p2 = dir2 * (1.0 + 0.1 * texture(cubeMap, dir2).r);
            vec3 p3 = dir3 * (1.0 + 0.1 * texture(cubeMap, dir3).r);
            return normalize(cross(normalize(p3 - p1), normalize(p2 - p1)));
        }
        void main(){
            //norm = getNormal(0.001, normal);
            norm = texture(cubeNormalMap, normal).rgb;
            gl_Position = projectionMatrix * modelViewMatrix * vec4(position + normal * texture(cubeMap, normal).r * 0.1, 1.0 );
        }
        `,
    fragmentShader: `
        in vec3 norm;
        void main(){
            float dt = dot(norm, vec3(0.0, 1.0, 0.0)) * 0.5 + 0.5;
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