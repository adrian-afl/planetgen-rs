/*
EROSION IDEA to make it parallel

Create an ErosionDropletRun struct
it will hold a vector list of struct of shape ErosionDropletModification {
    vec3 dir (maybe pixels im not sure) pixels will be easier to apply
    float delta
}

then in paraller per iteration i can spawn a droplet on random X pixels - maybe on all of them?
iterate all teh droplers in parallel until all end their journeys
resulting changes are then applied on the main data and the iteration restarts
 */
use crate::cubemap_data::CubeMapDataLayer;
use crate::random::random_2d_to_3d;
use glam::{DVec2, DVec3};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

struct ErosionDroplet {
    position: DVec3,
    velocity: DVec3,
    accumulation: f64,
    water_left: f64,
}

struct ErosionDropletModification {
    position: DVec3,
    delta: f64,
}

struct ErosionDropletRunResult {
    modifications: Vec<ErosionDropletModification>,
}

pub fn erosion_run<const RES: usize>(
    cubemap_data: &mut CubeMapDataLayer<RES>,
    iterations: i32,
    droplets_per_iteration: i32,
) {
    (0..iterations).for_each(|iteration| {
        println!("Erosion iteration: {iteration}");
        let run_results: Vec<ErosionDropletRunResult> = (0..droplets_per_iteration)
            .into_par_iter()
            .map(|droplet_num| {
                let mut run = ErosionDropletRunResult {
                    modifications: Vec::new(),
                };
                let mut droplet = ErosionDroplet {
                    position: (random_2d_to_3d(DVec2::new(iteration as f64, droplet_num as f64))
                        * 2.0
                        - 1.0)
                        .normalize(),
                    velocity: DVec3::new(0.0, 0.0, 0.0),
                    accumulation: 0.0,
                    water_left: 1.0,
                };
                while droplet.water_left > 0.0 {
                    let normal = cubemap_data.get_normal(droplet.position, 0.0001);

                    run.modifications.push(ErosionDropletModification {
                        position: droplet.position,
                        delta: -0.0001,
                    });

                    droplet.velocity +=
                        (droplet.position + normal * 0.00002).normalize() - droplet.position;
                    droplet.position = (droplet.position + droplet.velocity * 0.002).normalize();
                    droplet.accumulation += 0.00001;
                    droplet.water_left -= 0.00001;
                    // if (droplet_num == 0 && iteration == 0) {
                    //     println!(
                    //         "p {}, v {}, acc {}, wl: {}",
                    //         droplet.position,
                    //         droplet.velocity,
                    //         droplet.accumulation,
                    //         droplet.water_left
                    //     )
                    // }
                }
                run.modifications.push(ErosionDropletModification {
                    position: droplet.position,
                    delta: droplet.accumulation,
                });
                run
            })
            .collect();

        run_results.iter().for_each(|result| {
            result.modifications.iter().for_each(|modification| {
                cubemap_data.add(modification.position, modification.delta);
            })
        })
    })
}
