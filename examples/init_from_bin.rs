use rocketsim_rs::{init_from_mem, sim::Arena};


macro_rules! include_slice {
    ($path:expr) => {
        include_bytes!($path).as_slice()
    };
}

/// Do NOT redistribute mesh files!
fn init_from_bin() {
    let soccar = [
        include_slice!("../collision_meshes/soccar/mesh_0.cmf"),
        include_slice!("../collision_meshes/soccar/mesh_1.cmf"),
        include_slice!("../collision_meshes/soccar/mesh_2.cmf"),
        include_slice!("../collision_meshes/soccar/mesh_3.cmf"),
        include_slice!("../collision_meshes/soccar/mesh_4.cmf"),
        include_slice!("../collision_meshes/soccar/mesh_5.cmf"),
        include_slice!("../collision_meshes/soccar/mesh_6.cmf"),
        include_slice!("../collision_meshes/soccar/mesh_7.cmf"),
        include_slice!("../collision_meshes/soccar/mesh_8.cmf"),
        include_slice!("../collision_meshes/soccar/mesh_9.cmf"),
        include_slice!("../collision_meshes/soccar/mesh_10.cmf"),
        include_slice!("../collision_meshes/soccar/mesh_11.cmf"),
        include_slice!("../collision_meshes/soccar/mesh_12.cmf"),
        include_slice!("../collision_meshes/soccar/mesh_13.cmf"),
        include_slice!("../collision_meshes/soccar/mesh_14.cmf"),
        include_slice!("../collision_meshes/soccar/mesh_15.cmf"),
    ];

    init_from_mem(&soccar, &[]);
}

fn main() {
    init_from_bin();

    // Prove the data was loaded
    let mut arena = Arena::default_standard();
    arena.pin_mut().step(120);
}