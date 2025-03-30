use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};

fn main() -> AppExit {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<MandelbrotMaterial>::default(),
        ))
        .run()
}

#[derive(AsBindGroup, Asset, TypePath, Clone)]
struct MandelbrotMaterial {}

impl Material2d for MandelbrotMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/mandelbrot.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MandelbrotMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(750.0, 750.0))),
        MeshMaterial2d(materials.add(MandelbrotMaterial {})),
    ));
}
