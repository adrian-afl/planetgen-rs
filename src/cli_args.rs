use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLIArgs {
    // https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_2/index.html#options
    #[arg(short = None, long = "out-dir", default_value = "icosphere")]
    pub out_dir: String,

    #[arg(short, long, default_value_t = 6360000.0)]
    pub radius: f64,

    #[arg(short = None, long = "terrain-height", default_value_t = 3000000.0)]
    pub terrain_height: f64,

    #[arg(short = None, long = "subdivide-initial", default_value_t = 3)]
    pub subdivide_initial: u16,

    #[arg(short = None, long = "subdivide-level-1", default_value_t = 2)]
    pub subdivide_level1: u16,

    #[arg(short = None, long = "subdivide-level-2", default_value_t = 3)]
    pub subdivide_level2: u16,

    #[arg(short = None, long = "subdivide-level-3", default_value_t = 4)]
    pub subdivide_level3: u16,

    #[arg(short = None, long = "cube-map-res", default_value_t = 4096)]
    pub cube_map_resolution: u16,

    #[arg(short = None, long = "fbm-scale", default_value_t = 4.0)]
    pub fbm_scale: f64,

    #[arg(short = None, long = "fbm-iters", default_value_t = 15)]
    pub fbm_iterations: u16,

    #[arg(short = None, long = "fbm-iter-scale", default_value_t = 2.5)]
    pub fbm_iteration_scale_coef: f64,

    #[arg(short = None, long = "fbm-iter-weight", default_value_t = 0.4)]
    pub fbm_iteration_weight_coef: f64,

    #[arg(short = None, long = "fbm-final-pow", default_value_t = 4.0)]
    pub fbm_final_pow: f64,

    #[arg(short = None, long = "erosion-iters", default_value_t = 1000)]
    pub erosion_iterations: u16,

    #[arg(short = None, long = "erosion-droplets", default_value_t = 20000)]
    pub erosion_droplets_count: u16,
}
