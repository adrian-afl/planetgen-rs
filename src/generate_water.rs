use crate::cubemap_data::{CubeMapDataLayer, CubeMapFace};
use crate::generate_icosphere::generate_icosphere_raw;
use crate::json_input::InputCelestialBodyDefinition;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

pub fn generate_water(input: &InputCelestialBodyDefinition) {
    let water_out_dir = Path::new(&input.generator_config.out_dir).join("water");
    let water_icosphere_out_dir = Path::new(&input.generator_config.out_dir)
        .join("water")
        .join("icosphere");
    match fs::remove_dir_all(&water_out_dir) {
        Ok(_) => {}
        Err(e) => match e.kind() {
            ErrorKind::NotFound => (), // this is fine
            _ => panic!("Failed to delete the directory because {}", e),
        },
    }

    fs::create_dir_all(&water_out_dir);
    fs::create_dir(&water_icosphere_out_dir);

    let water = &input.water;
    if (water.is_none()) {
        return;
    }
    let water = water.as_ref().unwrap();

    let cube_map_res = input.generator_config.cube_map_resolution;
    // this is to allow to modify the water height it needed, later
    let mut water_cube_map: CubeMapDataLayer<f64> = CubeMapDataLayer::new(cube_map_res, 0.0);

    let mutable_faces = [
        (
            CubeMapFace::PX,
            water_cube_map.get_mutable_face(&CubeMapFace::PX),
        ),
        (
            CubeMapFace::PY,
            water_cube_map.get_mutable_face(&CubeMapFace::PY),
        ),
        (
            CubeMapFace::PZ,
            water_cube_map.get_mutable_face(&CubeMapFace::PZ),
        ),
        (
            CubeMapFace::NX,
            water_cube_map.get_mutable_face(&CubeMapFace::NX),
        ),
        (
            CubeMapFace::NY,
            water_cube_map.get_mutable_face(&CubeMapFace::NY),
        ),
        (
            CubeMapFace::NZ,
            water_cube_map.get_mutable_face(&CubeMapFace::NZ),
        ),
    ];

    mutable_faces.into_par_iter().for_each(|face| {
        println!("Generating water face {}, res: {}", face.0, cube_map_res);
        let mut face_data = face.1.lock().unwrap();
        for y in (0..cube_map_res) {
            for x in (0..cube_map_res) {
                let index = (y as usize) * (cube_map_res as usize) + (x as usize);
                face_data[index] = water.height
            }
        }
    });

    println!("Saving water icosphere");
    generate_icosphere_raw(
        water_icosphere_out_dir.to_str().unwrap(),
        &water_cube_map,
        None,
        water.height,
        input.generator_config.subdivide_initial,
        input.generator_config.subdivide_level1,
        input.generator_config.subdivide_level2,
        input.generator_config.subdivide_level3,
    );
}
