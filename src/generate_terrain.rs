use crate::craters::add_craters;
use crate::cubemap_data::{CubeMapDataLayer, CubeMapFace};
use crate::erosion::erosion_run;
use crate::generate_icosphere::generate_icosphere_raw;
use crate::json_input::{
    InputBiome, InputBiomeModifier, InputCelestialBodyDefinition, InputTerrain, InputVector3,
};
use crate::math_util::{map, mix, usat};
use crate::noise::fbm;
use glam::{DVec3, Vec3};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::time::Instant;

#[derive(Clone)]
pub struct InterpolatedBiomeData {
    //   pub dominating_id: u32,
    pub color: Vec3,
    pub roughness: f32,
    pub erosion_strength: f32,
    pub deposition_strength: f32,
    pub craters_probability: f32,
    pub min_crater_size: f32,
    pub max_crater_size: f32,
}

fn generate_biomes(
    input: &InputCelestialBodyDefinition,
    terrain: &InputTerrain,
    cube_map_height: &CubeMapDataLayer<f64>,
    cube_map_biome: &CubeMapDataLayer<InterpolatedBiomeData>,
) {
    let cube_map_res = input.generator_config.cube_map_resolution;

    let mutable_biome = [
        (
            CubeMapFace::PX,
            cube_map_biome.get_mutable_face(&CubeMapFace::PX),
        ),
        (
            CubeMapFace::PY,
            cube_map_biome.get_mutable_face(&CubeMapFace::PY),
        ),
        (
            CubeMapFace::PZ,
            cube_map_biome.get_mutable_face(&CubeMapFace::PZ),
        ),
        (
            CubeMapFace::NX,
            cube_map_biome.get_mutable_face(&CubeMapFace::NX),
        ),
        (
            CubeMapFace::NY,
            cube_map_biome.get_mutable_face(&CubeMapFace::NY),
        ),
        (
            CubeMapFace::NZ,
            cube_map_biome.get_mutable_face(&CubeMapFace::NZ),
        ),
    ];

    mutable_biome.into_par_iter().for_each(|face| {
        println!(
            "Generating terrain biome face {}, res: {}",
            face.0, cube_map_res
        );
        let mut face_data = face.1.lock().unwrap();
        for y in (0..cube_map_res) {
            for x in (0..cube_map_res) {
                let dir = cube_map_biome.pixel_coords_to_direction(&face.0, x as usize, y as usize);
                let index = (y as usize) * (cube_map_res as usize) + (x as usize);

                let height = cube_map_height.get_bilinear(dir) - terrain.radius;

                let modifier = match terrain.biome_modifier {
                    InputBiomeModifier::Latitude => dir.y.abs() * 90.0,
                    InputBiomeModifier::Tidal => -dir.z, // so by default -z faces the star
                    InputBiomeModifier::Random => fbm(dir, 4, 2.0, 0.5),
                };

                let mut result = InterpolatedBiomeData {
                    //dominating_id: 0,
                    color: Vec3::new(0.0, 0.0, 0.0),
                    roughness: 0.0,
                    erosion_strength: 0.0,
                    deposition_strength: 0.0,
                    craters_probability: 0.0,
                    min_crater_size: 0.0,
                    max_crater_size: 0.0,
                };
                let mut sum: f32 = 0.0;

                let mut randomseed = 0.0;

                terrain.biomes.iter().for_each(|biome| {
                    let fitness_altitude = usat(map(
                        height,
                        biome.min_altitude,
                        biome.max_altitude,
                        0.0,
                        1.0,
                    ));
                    let fitness_modifier = usat(map(
                        modifier,
                        biome.min_modifier,
                        biome.max_modifier,
                        0.0,
                        1.0,
                    ));

                    let randomizer = fbm(dir * 4.0 + randomseed + biome.seed, 5, 2.0, 0.5);
                    randomseed += 123.0;

                    let fitness =
                        fitness_altitude * fitness_modifier * (0.5 + 0.5 * randomizer) + 0.001;

                    // result.dominating_id += biome.id; //TODO :(
                    result.color += Vec3::new(
                        biome.color.x as f32,
                        biome.color.y as f32,
                        biome.color.z as f32,
                    ) * fitness as f32;
                    result.roughness += (biome.roughness * fitness) as f32;
                    result.erosion_strength += (biome.erosion_strength * fitness) as f32;
                    result.deposition_strength += (biome.deposition_strength * fitness) as f32;

                    result.craters_probability += (biome.craters_probability * fitness) as f32;
                    result.min_crater_size += (biome.min_crater_size * fitness) as f32;
                    result.max_crater_size += (biome.max_crater_size * fitness) as f32;

                    sum += fitness as f32;
                });

                if sum > 0.0 {
                    //  result.dominating_id = (result.dominating_id as f64 / sum) as u32;
                    result.color = result.color / sum;
                    result.roughness = result.roughness / sum;
                    result.erosion_strength = result.erosion_strength / sum;
                    result.deposition_strength = result.deposition_strength / sum;

                    result.craters_probability = result.craters_probability / sum;
                    result.min_crater_size = result.min_crater_size / sum;
                    result.max_crater_size = result.max_crater_size / sum;
                }

                face_data[index] = result;
            }
        }
    });
}

fn generate_height(
    input: &InputCelestialBodyDefinition,
    terrain: &InputTerrain,
    cube_map_height: &CubeMapDataLayer<f64>,
) {
    let cube_map_res = input.generator_config.cube_map_resolution;

    let mutable_height = [
        (
            CubeMapFace::PX,
            cube_map_height.get_mutable_face(&CubeMapFace::PX),
        ),
        (
            CubeMapFace::PY,
            cube_map_height.get_mutable_face(&CubeMapFace::PY),
        ),
        (
            CubeMapFace::PZ,
            cube_map_height.get_mutable_face(&CubeMapFace::PZ),
        ),
        (
            CubeMapFace::NX,
            cube_map_height.get_mutable_face(&CubeMapFace::NX),
        ),
        (
            CubeMapFace::NY,
            cube_map_height.get_mutable_face(&CubeMapFace::NY),
        ),
        (
            CubeMapFace::NZ,
            cube_map_height.get_mutable_face(&CubeMapFace::NZ),
        ),
    ];

    mutable_height.into_par_iter().for_each(|face| {
        println!(
            "Generating terrain height face {}, res: {}",
            face.0, cube_map_res
        );
        let mut face_data = face.1.lock().unwrap();
        for y in (0..cube_map_res) {
            for x in (0..cube_map_res) {
                let dir =
                    cube_map_height.pixel_coords_to_direction(&face.0, x as usize, y as usize);
                let value = if terrain.terrain_generation.fbm_iterations == 0 {
                    0.0
                } else {
                    let unorm = fbm(
                        dir * terrain.terrain_generation.fbm_scale
                            + terrain.terrain_generation.seed,
                        terrain.terrain_generation.fbm_iterations,
                        terrain.terrain_generation.fbm_iteration_scale_coefficient,
                        terrain.terrain_generation.fbm_iteration_weight_coefficient,
                    )
                    .powf(terrain.terrain_generation.fbm_final_power);
                    mix(terrain.min_height, terrain.max_height, unorm)
                };
                let index = (y as usize) * (cube_map_res as usize) + (x as usize);
                face_data[index] = terrain.radius + value;
            }
        }
    });
}

pub fn generate_terrain(input: &InputCelestialBodyDefinition) {
    let terrain_out_dir = Path::new(&input.generator_config.out_dir).join("terrain");
    let terrain_icosphere_out_dir = Path::new(&input.generator_config.out_dir)
        .join("terrain")
        .join("icosphere");
    match fs::remove_dir_all(&terrain_out_dir) {
        Ok(_) => {}
        Err(e) => match e.kind() {
            ErrorKind::NotFound => (), // this is fine
            _ => panic!("Failed to delete the directory because {}", e),
        },
    }

    fs::create_dir_all(&terrain_out_dir);
    fs::create_dir(&terrain_icosphere_out_dir);

    let terrain = &input.terrain;
    if (terrain.is_none()) {
        return;
    }
    let terrain = terrain.as_ref().unwrap();

    let faces = [
        CubeMapFace::PX,
        CubeMapFace::PY,
        CubeMapFace::PZ,
        CubeMapFace::NX,
        CubeMapFace::NY,
        CubeMapFace::NZ,
    ];

    let cube_map_res = input.generator_config.cube_map_resolution;
    let mut cube_map_height: CubeMapDataLayer<f64> = CubeMapDataLayer::new(cube_map_res, 0.0);
    let mut cube_map_biome: CubeMapDataLayer<InterpolatedBiomeData> = CubeMapDataLayer::new(
        cube_map_res,
        InterpolatedBiomeData {
            color: Vec3::new(0.0, 0.0, 0.0),
            deposition_strength: 0.0,
            erosion_strength: 0.0,
            roughness: 1.0,
            // dominating_id: 0,
            craters_probability: 0.0,
            min_crater_size: 0.0,
            max_crater_size: 0.0,
        },
    );

    generate_height(&input, &terrain, &cube_map_height);
    generate_biomes(&input, &terrain, &cube_map_height, &cube_map_biome);

    add_craters(
        &mut cube_map_height,
        &cube_map_biome,
        terrain.radius,
        terrain.terrain_generation.seed,
        terrain.terrain_generation.craters_count,
    );

    erosion_run(
        &mut cube_map_height,
        &mut cube_map_biome,
        input.generator_config.erosion_iterations,
        input.generator_config.erosion_droplets_count,
        terrain.radius,
        input.generator_config.erosion_droplet_velocity_coefficient,
        input
            .generator_config
            .erosion_droplet_evaporation_coefficient,
    );

    // remap biomes after erosion for some more realistic effect
    generate_biomes(&input, &terrain, &cube_map_height, &cube_map_biome);

    faces.clone().into_par_iter().for_each(|face| {
        println!("Saving height face {}, res: {}", face, cube_map_res);
        let mut imgbuf = image::ImageBuffer::new(cube_map_res as u32, cube_map_res as u32);
        imgbuf.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let dir = cube_map_height.pixel_coords_to_direction(&face, x as usize, y as usize);
            let value = map(
                cube_map_height.get_bilinear(dir),
                terrain.radius + terrain.min_height,
                terrain.radius + terrain.max_height,
                0.0,
                1.0,
            );
            // println!("{value}");

            *pixel = image::Luma([(value * 255.0) as u8]);
        });
        imgbuf
            .save(
                terrain_out_dir
                    .clone()
                    .join(format!("height_face_{}.png", face)),
            )
            .unwrap();
        imgbuf
            .save(format!("cubemap_visualizer/public/face_{}.png", face))
            .unwrap();
    });

    faces.clone().into_par_iter().for_each(|face| {
        println!("Saving normal face {}, res: {}", face, cube_map_res);
        let mut imgbuf = image::ImageBuffer::new(cube_map_res as u32, cube_map_res as u32);
        imgbuf.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let dir = cube_map_height.pixel_coords_to_direction(&face, x as usize, y as usize);
            let value =
                cube_map_height.get_normal(dir, cube_map_height.get_pixel_distance_for_dir(dir));

            *pixel = image::Rgb([
                (value.x * 255.0) as u8,
                (value.y * 255.0) as u8,
                (value.z * 255.0) as u8,
            ]);
        });
        imgbuf
            .save(
                terrain_out_dir
                    .clone()
                    .join(format!("normal_face_{}.png", face)),
            )
            .unwrap();
        imgbuf
            .save(format!(
                "cubemap_visualizer/public/normal_face_{}.png",
                face
            ))
            .unwrap();
    });

    faces.clone().into_par_iter().for_each(|face| {
        println!("Saving biome face {}, res: {}", face, cube_map_res);
        let mut imgbuf = image::ImageBuffer::new(cube_map_res as u32, cube_map_res as u32);
        imgbuf.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let dir = cube_map_biome.pixel_coords_to_direction(&face, x as usize, y as usize);
            let value = cube_map_biome.get(dir);

            *pixel = image::Rgb([
                (value.color.x * 255.0) as u8,
                (value.color.y * 255.0) as u8,
                (value.color.z * 255.0) as u8,
            ]);
        });
        imgbuf
            .save(
                terrain_out_dir
                    .clone()
                    .join(format!("biome_face_{}.png", face)),
            )
            .unwrap();
        imgbuf
            .save(format!("cubemap_visualizer/public/biome_face_{}.png", face))
            .unwrap();
    });

    println!("Saving terrain icosphere");
    generate_icosphere_raw(
        terrain_icosphere_out_dir.to_str().unwrap(),
        &cube_map_height,
        Some(&cube_map_biome),
        terrain.radius,
        input.generator_config.subdivide_initial,
        input.generator_config.subdivide_level1,
        input.generator_config.subdivide_level2,
        input.generator_config.subdivide_level3,
    );
}
