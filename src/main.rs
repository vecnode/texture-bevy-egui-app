// main.rs
// Copyright (C) 2026 vecnode

mod components;
mod constants;
mod setup;
mod systems;

use bevy::prelude::*;
use bevy::camera::Viewport;
use bevy::log::LogPlugin;
use bevy::window::WindowResolution;
use bevy_egui::{EguiPlugin, EguiGlobalSettings, PrimaryEguiContext, EguiPrimaryContextPass};

use setup::*;
use systems::*;
use components::EguiLayoutState;
use constants::WORLD_BACKGROUND_COLOR;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "info,bevy_render::view::window=error".into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "texture-bevy-egui-app".into(),
                        resolution: WindowResolution::new(960, 640),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
        )
        .add_plugins(EguiPlugin::default())
        .insert_resource(EguiGlobalSettings {
            auto_create_primary_context: false,
            ..default()
        })
        .insert_resource(ClearColor(WORLD_BACKGROUND_COLOR))
        .init_resource::<components::EguiLayoutState>()
        .init_resource::<components::GridState>()
        .init_resource::<components::AspectRatioState>()
        .init_resource::<components::TextureModeState>()
        .add_systems(
            Startup,
            (
                spawn_grid,
                spawn_textured_plane,
                setup_camera_and_lights,
                setup_split_screen_cameras,
            ),
        )
        .add_systems(
            Update,
            (
                update_grid_dimensions,
                update_texture_aspect_ratio,
            ),
        )
        .add_systems(
            EguiPrimaryContextPass,
            (egui_controls_ui, update_camera_viewports).chain(),
        )
        .run();
}

fn setup_split_screen_cameras(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
) {
    // Disable auto-create primary context
    egui_global_settings.auto_create_primary_context = false;
    
    // Single camera for 3D world (will take remaining space on right)
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: 60.0_f32.to_radians(), // 60 degrees FOV
            ..default()
        }),
        Transform::from_translation(crate::constants::CAMERA_TOP_POSITION).looking_at(Vec3::ZERO, Vec3::Z),
        crate::components::RightCamera,
    ));
    
    // Primary Egui context camera (renders UI on top)
    commands.spawn((
        PrimaryEguiContext,
        Camera2d,
        Camera {
            order: 10,
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
    ));
}

fn update_camera_viewports(
    window: Query<&Window>,
    mut right_camera: Query<&mut Camera, With<crate::components::RightCamera>>,
    layout_state: Res<EguiLayoutState>,
) {
    let Ok(window) = window.single() else { return };
    let physical_size = window.physical_size();
    let scale_factor = window.scale_factor();

    // Ceil left/top and floor right/bottom so the 3D viewport never intrudes into UI panels.
    let viewport_left = (layout_state.viewport_left * scale_factor).ceil() as u32;
    let viewport_top = (layout_state.viewport_top * scale_factor).ceil() as u32;
    let viewport_right = (layout_state.viewport_right * scale_factor).floor() as u32;
    let viewport_bottom = (layout_state.viewport_bottom * scale_factor).floor() as u32;

    if let Ok(mut camera) = right_camera.single_mut() {
        camera.viewport = Some(Viewport {
            physical_position: UVec2::new(viewport_left, viewport_top),
            physical_size: UVec2::new(
                viewport_right.min(physical_size.x).saturating_sub(viewport_left),
                viewport_bottom.min(physical_size.y).saturating_sub(viewport_top),
            ),
            ..default()
        });
    }
}