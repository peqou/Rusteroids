use bevy::{
    core_pipeline::fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin},
    prelude::*,
    render::{RenderApp, extract_component::ExtractComponent, render_resource::ShaderType},
    shader::ShaderRef,
    window::{Window, WindowPlugin, WindowResolution},
};

// Настройки ASCII-эффекта
#[derive(Component, ExtractComponent, Clone, Copy, ShaderType, Default)]
#[extract_app(RenderApp)]
struct AsciiEffect {
    strength: f32,
}

impl FullscreenMaterial for AsciiEffect {
    fn fragment_shader() -> ShaderRef {
        "shaders/ascii.wesl".into()
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rusteriods | ASCII Space".into(),
                    resolution: WindowResolution::new(1920, 1080),
                    ..default()
                }),
                ..default()
            }),
            FullscreenMaterialPlugin::<AsciiEffect>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Камера и ASCII-фильтр
    commands.spawn((
        Camera3d::default(),
        Camera {
            clear_color: Color::srgb(0.003, 0.005, 0.012).into(),
            ..default()
        },
        Transform::from_xyz(0.0, 3.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        AsciiEffect { strength: 1.0 },
    ));

    // Свет
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            range: 100.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 10.0),
    ));

    // Голубой корабль
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.5, 0.4, 3.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 0.8, 1.0))),
        Transform::default(),
    ));

    // Оранжевый астероид
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.5))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.35, 0.1))),
        Transform::from_xyz(2.0, 1.0, -10.0),
    ));
}
