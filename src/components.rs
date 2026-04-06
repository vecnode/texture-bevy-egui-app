// components.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;

#[derive(Component)]
pub struct RightCamera;

#[derive(Resource, Default)]
pub struct EguiLayoutState {
    pub viewport_left: f32,
    pub viewport_top: f32,
    pub viewport_right: f32,
    pub viewport_bottom: f32,
}

#[derive(Component)]
pub struct GridLine;

#[derive(Component)]
pub struct TexturedPlane;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AspectRatio {
    Ratio16_9,
    Square,
}

#[derive(Resource)]
pub struct AspectRatioState {
    pub current: AspectRatio,
    pub previous: AspectRatio,
}

impl Default for AspectRatioState {
    fn default() -> Self {
        Self {
            current: AspectRatio::Square,
            previous: AspectRatio::Square,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TextureMode {
    Normal,
    Stretch,
}

#[derive(Resource)]
pub struct TextureModeState {
    pub current: TextureMode,
    pub previous: TextureMode,
}

impl Default for TextureModeState {
    fn default() -> Self {
        Self {
            current: TextureMode::Stretch,
            previous: TextureMode::Stretch,
        }
    }
}

#[derive(Resource)]
pub struct GridState {
    pub size_x: i32,
    pub size_z: i32,
    pub previous_size_x: i32,
    pub previous_size_z: i32,
}

impl Default for GridState {
    fn default() -> Self {
        Self {
            size_x: 10,
            size_z: 10,
            previous_size_x: 10,
            previous_size_z: 10,
        }
    }
}