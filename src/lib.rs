use bevy::{
    app::{App, Plugin},
    pbr::{Material, MaterialPlugin},
    reflect::TypeUuid,
    render::{
        color::Color,
        render_resource::{AsBindGroup, ShaderRef},
    },
};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "81d93b11-4a94-4560-a75d-7b827ecd887f"]
pub struct ToonMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub glossiness: f32,
    #[uniform(0)]
    pub receive_shadows: u32,
}

impl Material for ToonMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/toon_material.wgsl".into()
    }
}

impl Default for ToonMaterial {
    fn default() -> Self {
        Self {
            color: Color::PINK,
            glossiness: 0.0,
            receive_shadows: 1,
        }
    }
}

pub struct ToonMaterialPlugin;

impl Plugin for ToonMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<ToonMaterial>::default());
    }
}
