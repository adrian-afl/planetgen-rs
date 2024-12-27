use crate::generate_icosphere::Triangle;
use glam::DVec3;

pub fn get_base_icosphere() -> [Triangle; 20] {
    [
        [
            DVec3::new(-1.0, 1.618033988749895, 0.0),
            DVec3::new(-1.618033988749895, 0.0, 1.0),
            DVec3::new(0.0, 1.0, 1.618033988749895),
        ],
        [
            DVec3::new(-1.0, 1.618033988749895, 0.0),
            DVec3::new(0.0, 1.0, 1.618033988749895),
            DVec3::new(1.0, 1.618033988749895, 0.0),
        ],
        [
            DVec3::new(-1.0, 1.618033988749895, 0.0),
            DVec3::new(1.0, 1.618033988749895, 0.0),
            DVec3::new(0.0, 1.0, -1.618033988749895),
        ],
        [
            DVec3::new(-1.0, 1.618033988749895, 0.0),
            DVec3::new(0.0, 1.0, -1.618033988749895),
            DVec3::new(-1.618033988749895, 0.0, -1.0),
        ],
        [
            DVec3::new(-1.0, 1.618033988749895, 0.0),
            DVec3::new(-1.618033988749895, 0.0, -1.0),
            DVec3::new(-1.618033988749895, 0.0, 1.0),
        ],
        [
            DVec3::new(1.0, 1.618033988749895, 0.0),
            DVec3::new(0.0, 1.0, 1.618033988749895),
            DVec3::new(1.618033988749895, 0.0, 1.0),
        ],
        [
            DVec3::new(0.0, 1.0, 1.618033988749895),
            DVec3::new(-1.618033988749895, 0.0, 1.0),
            DVec3::new(0.0, -1.0, 1.618033988749895),
        ],
        [
            DVec3::new(-1.618033988749895, 0.0, 1.0),
            DVec3::new(-1.618033988749895, 0.0, -1.0),
            DVec3::new(-1.0, -1.618033988749895, 0.0),
        ],
        [
            DVec3::new(-1.618033988749895, 0.0, -1.0),
            DVec3::new(0.0, 1.0, -1.618033988749895),
            DVec3::new(0.0, -1.0, -1.618033988749895),
        ],
        [
            DVec3::new(0.0, 1.0, -1.618033988749895),
            DVec3::new(1.0, 1.618033988749895, 0.0),
            DVec3::new(1.618033988749895, 0.0, -1.0),
        ],
        [
            DVec3::new(1.0, -1.618033988749895, 0.0),
            DVec3::new(1.618033988749895, 0.0, 1.0),
            DVec3::new(0.0, -1.0, 1.618033988749895),
        ],
        [
            DVec3::new(1.0, -1.618033988749895, 0.0),
            DVec3::new(0.0, -1.0, 1.618033988749895),
            DVec3::new(-1.0, -1.618033988749895, 0.0),
        ],
        [
            DVec3::new(1.0, -1.618033988749895, 0.0),
            DVec3::new(-1.0, -1.618033988749895, 0.0),
            DVec3::new(0.0, -1.0, -1.618033988749895),
        ],
        [
            DVec3::new(1.0, -1.618033988749895, 0.0),
            DVec3::new(0.0, -1.0, -1.618033988749895),
            DVec3::new(1.618033988749895, 0.0, -1.0),
        ],
        [
            DVec3::new(1.0, -1.618033988749895, 0.0),
            DVec3::new(1.618033988749895, 0.0, -1.0),
            DVec3::new(1.618033988749895, 0.0, 1.0),
        ],
        [
            DVec3::new(0.0, -1.0, 1.618033988749895),
            DVec3::new(1.618033988749895, 0.0, 1.0),
            DVec3::new(0.0, 1.0, 1.618033988749895),
        ],
        [
            DVec3::new(-1.0, -1.618033988749895, 0.0),
            DVec3::new(0.0, -1.0, 1.618033988749895),
            DVec3::new(-1.618033988749895, 0.0, 1.0),
        ],
        [
            DVec3::new(0.0, -1.0, -1.618033988749895),
            DVec3::new(-1.0, -1.618033988749895, 0.0),
            DVec3::new(-1.618033988749895, 0.0, -1.0),
        ],
        [
            DVec3::new(1.618033988749895, 0.0, -1.0),
            DVec3::new(0.0, -1.0, -1.618033988749895),
            DVec3::new(0.0, 1.0, -1.618033988749895),
        ],
        [
            DVec3::new(1.618033988749895, 0.0, 1.0),
            DVec3::new(1.618033988749895, 0.0, -1.0),
            DVec3::new(1.0, 1.618033988749895, 0.0),
        ],
    ]
}