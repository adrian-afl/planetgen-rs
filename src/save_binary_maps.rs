use crate::cubemap_data::{CubeMapDataLayer, CubeMapFace};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::fs::File;
use std::io::Write;

pub fn save_terrain_maps(
    output_dir: &str,
    terrain_min_height: f64,
    cube_map_height: &CubeMapDataLayer<f64>,
) {
    let mutable_faces = [
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

    let mut metadata_file =
        File::create(output_dir.to_owned() + "/terrain_resolution.ini").expect("create failed");
    metadata_file
        .write(format!("{}", cube_map_height.res).as_bytes())
        .expect("Write failed");
    metadata_file.flush();

    mutable_faces.into_par_iter().for_each(|face| {
        println!(
            "Saving binary height map face {}, res: {}",
            face.0, cube_map_height.res
        );

        let face_data = face.1.lock().unwrap();

        let mut file = brotli::CompressorWriter::new(
            File::create(output_dir.to_owned() + format!("/terrain_{}.raw", face.0).as_str())
                .expect("create failed"),
            40960,
            11,
            21,
        );
        let res_usize = cube_map_height.res as usize;
        for i in (0..res_usize * res_usize) {
            file.write(&((face_data[i] - terrain_min_height) as f32).to_le_bytes())
                .expect("Write failed");
        }

        file.flush().unwrap();
    });
}
