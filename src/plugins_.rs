use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    render::{
        RenderDebugFlags, RenderPlugin,
        settings::{Backends, RenderCreation, WgpuSettings},
    },
};

use crate::{events::GameOverEvent, systems::*};

/// This DX12 plugin avoids errors with AMD GPUs
pub fn dx12_plugin() -> PluginGroupBuilder {
    DefaultPlugins.set(RenderPlugin {
        render_creation: RenderCreation::Automatic(WgpuSettings {
            backends: Some(Backends::DX12),
            ..default()
        }),
        synchronous_pipeline_compilation: true,
        debug_flags: RenderDebugFlags::empty(),
    })
}

pub struct CustomPlugin;

impl Plugin for CustomPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (handle_game_over, exit_game));
    }
}
