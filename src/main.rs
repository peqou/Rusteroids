use bevy::{
    core_pipeline::fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin},
    prelude::*,
    render::{RenderApp, extract_component::ExtractComponent, render_resource::ShaderType},
    shader::ShaderRef,
    window::{Window, WindowPlugin, WindowResolution},
};

// ========================================
// ASCII RENDERER
// ========================================

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

// ========================================
// GAME COMPONENTS
// ========================================

#[derive(Component)]
struct Ship;

#[derive(Component)]
struct Asteroid {
    speed: f32,
}

#[derive(Component)]
struct Star {
    speed: f32,
}

const SHIP_SPEED: f32 = 9.0;
const ASTEROID_COUNT: usize = 28;
const STAR_COUNT: usize = 180;

// Простые псевдослучайные числа.
// Позже сделаем нормальный генератор.
fn random(seed: f32) -> f32 {
    (seed.sin() * 43758.5453).fract().abs()
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rusteriods | Space Flight".into(),
                    resolution: WindowResolution::new(1920, 1080),
                    ..default()
                }),
                ..default()
            }),
            FullscreenMaterialPlugin::<AsciiEffect>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_ship, move_asteroids, move_stars))
        .run();
}

// ========================================
// SCENE SETUP
// ========================================

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Камера позади корабля
    commands.spawn((
        Camera3d::default(),
        Camera {
            clear_color: Color::srgb(0.0, 0.0, 0.0).into(),
            ..default()
        },
        Transform::from_xyz(0.0, 3.5, 14.0).looking_at(Vec3::new(0.0, 0.0, -8.0), Vec3::Y),
        AsciiEffect { strength: 1.0 },
    ));

    // Освещение
    commands.spawn((
        PointLight {
            intensity: 2_500_000.0,
            range: 120.0,
            ..default()
        },
        Transform::from_xyz(4.0, 10.0, 8.0),
    ));

    // Материалы
    let cyan = materials.add(Color::srgb(0.0, 0.9, 1.0));

    let blue = materials.add(Color::srgb(0.05, 0.25, 1.0));

    let engine = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.9, 1.0),
        unlit: true,
        ..default()
    });

    let orange = materials.add(Color::srgb(1.0, 0.35, 0.05));

    let purple = materials.add(Color::srgb(0.65, 0.2, 1.0));

    let white = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.8, 1.0),
        unlit: true,
        ..default()
    });

    // ====================================
    // SPACESHIP
    // ====================================

    let hull_mesh = meshes.add(Cuboid::new(0.8, 0.35, 3.2));

    let wing_mesh = meshes.add(Cuboid::new(4.2, 0.12, 0.85));

    let cockpit_mesh = meshes.add(Cuboid::new(0.55, 0.3, 1.1));

    let engine_mesh = meshes.add(Cuboid::new(0.4, 0.4, 1.0));

    let glow_mesh = meshes.add(Cuboid::new(0.3, 0.3, 0.12));

    // Все детали привязаны к одному кораблю
    commands
        .spawn((Ship, Transform::default(), Visibility::default()))
        .with_children(|ship| {
            // Корпус
            ship.spawn((
                Mesh3d(hull_mesh),
                MeshMaterial3d(cyan.clone()),
                Transform::default(),
            ));

            // Крылья
            ship.spawn((
                Mesh3d(wing_mesh),
                MeshMaterial3d(blue.clone()),
                Transform::from_xyz(0.0, -0.05, 0.5),
            ));

            // Кабина
            ship.spawn((
                Mesh3d(cockpit_mesh),
                MeshMaterial3d(cyan.clone()),
                Transform::from_xyz(0.0, 0.28, -0.4),
            ));

            // Левый и правый двигатели
            for side in [-1.0_f32, 1.0] {
                ship.spawn((
                    Mesh3d(engine_mesh.clone()),
                    MeshMaterial3d(blue.clone()),
                    Transform::from_xyz(side * 1.25, 0.0, 1.1),
                ));

                // Голубое свечение сопла
                ship.spawn((
                    Mesh3d(glow_mesh.clone()),
                    MeshMaterial3d(engine.clone()),
                    Transform::from_xyz(side * 1.25, 0.0, 1.67),
                ));
            }
        });

    // ====================================
    // ASTEROIDS
    // ====================================

    let asteroid_mesh = meshes.add(Sphere::new(1.0));

    for i in 0..ASTEROID_COUNT {
        let n = i as f32;

        let x = (random(n * 12.3 + 1.0) - 0.5) * 28.0;

        let y = (random(n * 7.9 + 4.0) - 0.5) * 17.0;

        let z = -12.0 - n * 3.5;

        let scale = 0.5 + random(n * 9.2) * 1.3;

        let material = if i % 3 == 0 {
            purple.clone()
        } else {
            orange.clone()
        };

        commands.spawn((
            Asteroid {
                speed: 12.0 + random(n) * 8.0,
            },
            Mesh3d(asteroid_mesh.clone()),
            MeshMaterial3d(material),
            Transform::from_xyz(x, y, z).with_scale(Vec3::splat(scale)),
        ));
    }

    // ====================================
    // STAR FIELD
    // ====================================

    let star_mesh = meshes.add(Sphere::new(0.045));

    for i in 0..STAR_COUNT {
        let n = i as f32;

        let x = (random(n * 17.7 + 5.0) - 0.5) * 65.0;

        let y = (random(n * 23.1 + 3.0) - 0.5) * 40.0;

        let z = -5.0 - random(n * 31.3 + 8.0) * 110.0;

        commands.spawn((
            Star {
                speed: 14.0 + random(n * 6.1) * 12.0,
            },
            Mesh3d(star_mesh.clone()),
            MeshMaterial3d(white.clone()),
            Transform::from_xyz(x, y, z),
        ));
    }
}

// ========================================
// PLAYER MOVEMENT
// ========================================

fn move_ship(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut ships: Query<&mut Transform, With<Ship>>,
) {
    for mut transform in &mut ships {
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }

        if keyboard.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if keyboard.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }

        if keyboard.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }

        transform.translation += direction * SHIP_SPEED * time.delta_secs();

        // Ограничиваем пространство полёта
        transform.translation.x = transform.translation.x.clamp(-9.0, 9.0);

        transform.translation.y = transform.translation.y.clamp(-5.0, 5.0);

        // Наклон при движении в стороны
        let target_roll = -direction.x * 0.25;

        let current_roll = transform.rotation.to_euler(EulerRot::XYZ).2;

        let new_roll =
            current_roll + (target_roll - current_roll) * (time.delta_secs() * 8.0).min(1.0);

        transform.rotation = Quat::from_rotation_z(new_roll);
    }
}

// ========================================
// ASTEROID MOVEMENT
// ========================================

fn move_asteroids(time: Res<Time>, mut asteroids: Query<(&mut Transform, &Asteroid)>) {
    for (mut transform, asteroid) in &mut asteroids {
        // Летят к камере
        transform.translation.z += asteroid.speed * time.delta_secs();

        transform.rotate_y(0.5 * time.delta_secs());

        transform.rotate_x(0.3 * time.delta_secs());

        // Возвращаем астероид вдаль
        if transform.translation.z > 12.0 {
            transform.translation.z = -100.0;

            transform.translation.x =
                (random(time.elapsed_secs() * 7.3 + asteroid.speed) - 0.5) * 28.0;

            transform.translation.y =
                (random(time.elapsed_secs() * 3.7 + asteroid.speed) - 0.5) * 17.0;
        }
    }
}

// ========================================
// STAR MOVEMENT
// ========================================

fn move_stars(time: Res<Time>, mut stars: Query<(&mut Transform, &Star)>) {
    for (mut transform, star) in &mut stars {
        transform.translation.z += star.speed * time.delta_secs();

        if transform.translation.z > 10.0 {
            transform.translation.z = -110.0;
        }
    }
}
