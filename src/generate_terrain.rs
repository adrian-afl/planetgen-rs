use crate::cubemap_data::{CubeMapDataLayer, CubeMapFace};
use crate::erosion::erosion_run;
use crate::generate_icosphere::generate_icosphere_raw;
use crate::json_input::{
    InputBiome, InputBiomeModifier, InputCelestialBodyDefinition, InputVector3,
};
use crate::math_util::mix;
use crate::noise::fbm;
use glam::DVec3;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::time::Instant;

#[derive(Clone)]
pub struct InterpolatedBiomeData {
    pub dominating_id: u32,
    pub color: DVec3,
    pub roughness: f64,
    pub erosion_strength: f64,
    pub deposition_strength: f64,
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

    fs::create_dir(&terrain_out_dir);
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
            color: DVec3::new(0.0, 0.0, 0.0),
            deposition_strength: 0.0,
            erosion_strength: 0.0,
            roughness: 1.0,
            dominating_id: 0,
        },
    );

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

                let height = cube_map_height.get_bilinear(dir);

                let modifier = match terrain.biome_modifier {
                    InputBiomeModifier::Latitude => dir.y.abs(),
                    InputBiomeModifier::Tidal => -dir.z, // so by default -z faces the star
                    InputBiomeModifier::Random => fbm(dir, 4, 2.0, 0.5),
                };

                let suitable_biomes: Vec<&InputBiome> = terrain
                    .biomes
                    .iter()
                    .filter(|biome| {
                        if biome.min_altitude > height
                            && biome.max_altitude < height
                            && biome.min_modifier > height
                            && biome.max_modifier < height
                        {
                            return true;
                        }
                        false
                    })
                    .collect();
                if suitable_biomes.len() == 0 {
                    panic!("Could not find a suitable biome for parameters height {height}, modifier {modifier}");
                }
                let interpolated = if suitable_biomes.len() == 1 {
                    let biome = suitable_biomes[0];
                    InterpolatedBiomeData {
                        dominating_id: biome.id,
                        color: DVec3::new(biome.color.x, biome.color.y, biome.color.z),
                        roughness: biome.roughness,
                        erosion_strength: biome.erosion_strength,
                        deposition_strength: biome.deposition_strength,
                    }
                } else {
                    let randomizer = suitable_biomes.len() as f64
                        * fbm(dir * 3.0, 4, 2.2, 0.5);

                    let randomizer_floor = randomizer.floor();
                    let randomizer_ceil = randomizer.ceil();
                    let randomizer_fract = randomizer.fract();

                    let biome_a = suitable_biomes[randomizer_floor as usize];
                    let biome_b = suitable_biomes[randomizer_ceil as usize];
                    InterpolatedBiomeData {
                        dominating_id: if randomizer_fract < 0.5 { biome_a.id } else { biome_b.id },
                        color: DVec3::new(mix(biome_a.color.x, biome_b.color.x, randomizer_fract),
                                          mix(biome_a.color.y, biome_b.color.y, randomizer_fract),
                                          mix(biome_a.color.z, biome_b.color.z, randomizer_fract),
                        ),
                        roughness: mix(biome_a.roughness, biome_b.roughness, randomizer_fract),
                        erosion_strength: mix(biome_a.erosion_strength, biome_b.erosion_strength, randomizer_fract),
                        deposition_strength: mix(biome_a.deposition_strength, biome_b.deposition_strength, randomizer_fract),
                    }
                };

                face_data[index] = interpolated;
            }
        }
    });

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
                        dir * terrain.terrain_generation.fbm_scale,
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

    erosion_run(
        &mut cube_map_height,
        &mut cube_map_biome,
        input.generator_config.erosion_iterations,
        input.generator_config.erosion_droplets_count,
        terrain.radius,
    );

    faces.clone().into_par_iter().for_each(|face| {
        println!("Saving height face {}, res: {}", face, cube_map_res);
        let mut imgbuf = image::ImageBuffer::new(cube_map_res as u32, cube_map_res as u32);
        imgbuf.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let dir = cube_map_height.pixel_coords_to_direction(&face, x as usize, y as usize);
            let value = (cube_map_height.get_bilinear(dir) - terrain.radius - terrain.min_height)
                / (terrain.max_height / terrain.min_height);

            *pixel = image::Luma([(value * 255.0) as u8]);
        });
        imgbuf
            .save(format!("{:?}/height_face_{}.png", terrain_out_dir, face))
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
            .save(format!("{:?}/normal_face_{}.png", terrain_out_dir, face))
            .unwrap();
        imgbuf
            .save(format!(
                "cubemap_visualizer/public/normal_face_{}.png",
                face
            ))
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
