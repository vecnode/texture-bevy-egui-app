// setup.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use crate::components::{GridLine, GridState, TexturedPlane};
use crate::constants::*;

pub fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid_state: Res<GridState>,
) {
    spawn_grid_lines(
        &mut commands,
        &mut meshes,
        &mut materials,
        grid_state.size_x,
        grid_state.size_z,
    );
}

pub fn spawn_grid_lines(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    size_x: i32,
    size_z: i32,
) {
    let size_x = size_x as f32;
    let size_z = size_z as f32;
    let half_size_x = size_x / 2.0;
    let half_size_z = size_z / 2.0;
    let num_lines_x = size_x as i32 + 1;
    let num_lines_z = size_z as i32 + 1;
    
    for i in 0..num_lines_z {
        let z = -half_size_z + (i as f32 * GRID_SPACING);
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(GRID_LINE_RADIUS, size_x))),
            MeshMaterial3d(materials.add(GRID_COLOR)),
            Transform::from_translation(Vec3::new(0.0, 0.0, z))
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            GridLine,
        ));
    }
    
    for i in 0..num_lines_x {
        let x = -half_size_x + (i as f32 * GRID_SPACING);
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(GRID_LINE_RADIUS, size_z))),
            MeshMaterial3d(materials.add(GRID_COLOR)),
            Transform::from_translation(Vec3::new(x, 0.0, 0.0))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            GridLine,
        ));
    }
}

pub fn spawn_textured_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid_state: Res<GridState>,
) {
    let size_x = grid_state.size_x as f32;
    let size_z = grid_state.size_z as f32;
    
    let plane_mesh = meshes.add(Rectangle::new(size_x, size_z));
    let material = materials.add(StandardMaterial {
        unlit: true,
        ..default()
    });

    commands.spawn((
        Mesh3d(plane_mesh),
        MeshMaterial3d(material),
        Transform::from_translation(Vec3::new(0.0, 0.01, 0.0))
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2) * Quat::from_rotation_z(std::f32::consts::PI)),
        TexturedPlane,
    ));
}

pub fn setup_camera_and_lights(mut commands: Commands) {
    // Front light
    commands.spawn(DirectionalLight {
        illuminance: FRONT_LIGHT_ILLUMINANCE,
        ..default()
    });
    
    // Back light (from behind)
    commands.spawn((
        DirectionalLight {
            illuminance: BACK_LIGHT_ILLUMINANCE,
            ..default()
        },
        Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
    ));
}
