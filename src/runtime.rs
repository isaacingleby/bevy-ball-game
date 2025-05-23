use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    render::{
        RenderDebugFlags, RenderPlugin,
        settings::{Backends, RenderCreation, WgpuSettings},
    },
};

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
