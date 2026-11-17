use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, WindowResolution};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rusteriods".into(),
                resolution: WindowResolution::new(1920, 1080),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Камера за кораблём
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Освещение
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            range: 100.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 10.0),
    ));

    // Временная модель корабля
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.5, 0.4, 3.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 0.8, 1.0))),
        Transform::default(),
    ));

    // Первый астероид
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.5))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.35, 0.1))),
        Transform::from_xyz(2.0, 1.0, -10.0),
    ));
}
