// systems/grid.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use crate::components::{GridState, GridLine};
use crate::setup::spawn_grid_lines;

pub fn update_grid_dimensions(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut grid_state: ResMut<GridState>,
    grid_line_query: Query<Entity, With<GridLine>>,
) {
    if grid_state.size_x != grid_state.previous_size_x || grid_state.size_z != grid_state.previous_size_z {
        for entity in grid_line_query.iter() {
            commands.entity(entity).despawn();
        }

        grid_state.previous_size_x = grid_state.size_x;
        grid_state.previous_size_z = grid_state.size_z;

        spawn_grid_lines(
            &mut commands,
            &mut meshes,
            &mut materials,
            grid_state.size_x,
            grid_state.size_z,
        );
    }
}
