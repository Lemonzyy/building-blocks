use crate::{
    mesh_generator::{MeshCommand, MeshCommandQueue},
    voxel_map::VoxelMap,
};

use building_blocks::{core::prelude::*, storage::prelude::ChunkUnits};

use bevy_utilities::bevy::{prelude::*, render::camera::Camera};

pub struct LodState {
    old_lod0_center: ChunkUnits<Point3i>,
}

impl LodState {
    pub fn new(lod0_center: ChunkUnits<Point3i>) -> Self {
        Self {
            old_lod0_center: lod0_center,
        }
    }
}

/// Adjusts the sample rate of voxels depending on their distance from the camera.
pub fn level_of_detail_system<Map: VoxelMap>(
    cameras: Query<(&Camera, &Transform)>,
    voxel_map: Res<Map>,
    mut lod_state: ResMut<LodState>,
    mut mesh_commands: ResMut<MeshCommandQueue>,
) {
    let camera_position = if let Some((_camera, tfm)) = cameras.iter().next() {
        tfm.translation
    } else {
        return;
    };

    let map_config = voxel_map.config();

    let lod0_center =
        ChunkUnits(Point3f::from(camera_position).in_voxel() >> map_config.chunk_exponent);

    if lod0_center == lod_state.old_lod0_center {
        return;
    }

    voxel_map.chunk_index().find_clipmap_chunk_updates(
        &map_config.world_extent(),
        map_config.clip_box_radius,
        lod_state.old_lod0_center,
        lod0_center,
        |update| mesh_commands.enqueue(MeshCommand::Update(update)),
    );

    lod_state.old_lod0_center = lod0_center;
}
