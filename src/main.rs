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
        .init_resource::<Score>()
        .add_systems(Startup, (setup, setup_combat))
        .add_systems(
            Update,
            (
                move_ship,
                move_asteroids,
                move_stars,
                shoot,
                move_lasers,
                check_hits,
                update_explosions,
                update_score,
            )
                .chain(),
        )
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

//
// RUSTERIODS v0.3 — COMBAT SYSTEM
//

use bevy::text::FontSize;

// Игровой счёт
#[derive(Resource, Default)]
struct Score(u32);

// Метка для текста со счётом
#[derive(Component)]
struct ScoreDisplay;

// Лазерный снаряд
#[derive(Component)]
struct Laser;

// Частица взрыва
#[derive(Component)]
struct Explosion {
    velocity: Vec3,
    lifetime: f32,
}

// Общие ресурсы для эффектов
#[derive(Resource)]
struct CombatAssets {
    laser_mesh: Handle<Mesh>,
    laser_material: Handle<StandardMaterial>,
    explosion_mesh: Handle<Mesh>,
    explosion_material: Handle<StandardMaterial>,
}

// ----------------------------------------
// ИНИЦИАЛИЗАЦИЯ
// ----------------------------------------

fn setup_combat(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let laser_mesh = meshes.add(Cuboid::new(0.13, 0.13, 1.8));

    let laser_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 1.0, 0.9),
        emissive: Color::srgb(0.0, 8.0, 6.0).into(),
        unlit: true,
        ..default()
    });

    let explosion_mesh = meshes.add(Sphere::new(0.16));

    let explosion_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.6, 0.1),
        emissive: Color::srgb(8.0, 2.0, 0.1).into(),
        unlit: true,
        ..default()
    });

    commands.insert_resource(CombatAssets {
        laser_mesh,
        laser_material,
        explosion_mesh,
        explosion_material,
    });

    // Счётчик очков в левом верхнем углу
    commands.spawn((
        ScoreDisplay,
        Text::new("SCORE: 00000   |   SPACE: FIRE"),
        TextFont {
            font_size: FontSize::Px(26.0),
            ..default()
        },
        TextColor(Color::srgb(0.0, 1.0, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            top: px(22),
            left: px(28),
            ..default()
        },
    ));
}

// ----------------------------------------
// СТРЕЛЬБА
// ----------------------------------------

fn shoot(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    ships: Query<&Transform, With<Ship>>,
    assets: Res<CombatAssets>,
    mut cooldown: Local<f32>,
) {
    *cooldown = (*cooldown - time.delta_secs()).max(0.0);

    if !keyboard.pressed(KeyCode::Space) {
        return;
    }

    if *cooldown > 0.0 {
        return;
    }

    let Ok(ship) = ships.single() else {
        return;
    };

    // Один выстрел примерно каждые 0.15 секунды
    *cooldown = 0.15;

    // Два лазера — по одному с каждого крыла
    for side in [-1.0_f32, 1.0] {
        let local_offset = Vec3::new(side * 1.65, 0.0, -0.9);

        let position = ship.translation + ship.rotation * local_offset;

        commands.spawn((
            Laser,
            Mesh3d(assets.laser_mesh.clone()),
            MeshMaterial3d(assets.laser_material.clone()),
            Transform::from_translation(position),
        ));
    }
}

// ----------------------------------------
// ДВИЖЕНИЕ ЛАЗЕРОВ
// ----------------------------------------

fn move_lasers(
    mut commands: Commands,
    time: Res<Time>,
    mut lasers: Query<(Entity, &mut Transform), With<Laser>>,
) {
    for (entity, mut transform) in &mut lasers {
        // Лазеры летят вперёд, вдоль оси -Z
        transform.translation.z -= 65.0 * time.delta_secs();

        // Удаляем далёкие лазеры
        if transform.translation.z < -125.0 {
            commands.entity(entity).despawn();
        }
    }
}

// ----------------------------------------
// ПОПАДАНИЯ ПО АСТЕРОИДАМ
// ----------------------------------------

fn check_hits(
    mut commands: Commands,
    time: Res<Time>,
    lasers: Query<(Entity, &Transform), (With<Laser>, Without<Asteroid>)>,
    mut asteroids: Query<(&mut Transform, &Asteroid)>,
    mut score: ResMut<Score>,
    assets: Res<CombatAssets>,
) {
    for (laser_entity, laser_transform) in &lasers {
        let laser_position = laser_transform.translation;

        for (mut asteroid_transform, asteroid) in &mut asteroids {
            let asteroid_position = asteroid_transform.translation;

            let radius = asteroid_transform.scale.x * 1.0;

            let distance = laser_position.distance(asteroid_position);

            if distance < radius + 0.45 {
                // Удаляем лазер
                commands.entity(laser_entity).despawn();

                // Добавляем очки
                score.0 += 100;

                // Создаём взрыв
                spawn_explosion(&mut commands, &assets, asteroid_position);

                // Возвращаем астероид вдаль
                let seed = time.elapsed_secs() + asteroid.speed;

                asteroid_transform.translation = Vec3::new(
                    (random(seed * 11.3) - 0.5) * 28.0,
                    (random(seed * 5.7) - 0.5) * 17.0,
                    -110.0,
                );

                // Один лазер — одно попадание
                break;
            }
        }
    }
}

// ----------------------------------------
// ВЗРЫВ ИЗ ЦВЕТНЫХ ЧАСТИЦ
// ----------------------------------------

fn spawn_explosion(commands: &mut Commands, assets: &CombatAssets, position: Vec3) {
    for i in 0..16 {
        let angle = i as f32 * std::f32::consts::TAU / 16.0;

        let velocity = Vec3::new(
            angle.cos() * 7.0,
            angle.sin() * 7.0,
            ((i % 5) as f32 - 2.0) * 2.5,
        );

        commands.spawn((
            Explosion {
                velocity,
                lifetime: 0.55,
            },
            Mesh3d(assets.explosion_mesh.clone()),
            MeshMaterial3d(assets.explosion_material.clone()),
            Transform::from_translation(position),
        ));
    }
}

// ----------------------------------------
// АНИМАЦИЯ ВЗРЫВОВ
// ----------------------------------------

fn update_explosions(
    mut commands: Commands,
    time: Res<Time>,
    mut explosions: Query<(Entity, &mut Transform, &mut Explosion)>,
) {
    for (entity, mut transform, mut explosion) in &mut explosions {
        explosion.lifetime -= time.delta_secs();

        transform.translation += explosion.velocity * time.delta_secs();

        // Частицы уменьшаются перед исчезновением
        let scale = (explosion.lifetime / 0.55).max(0.01);

        transform.scale = Vec3::splat(scale);

        if explosion.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

// ----------------------------------------
// ОБНОВЛЕНИЕ СЧЁТА
// ----------------------------------------

fn update_score(score: Res<Score>, mut displays: Query<&mut Text, With<ScoreDisplay>>) {
    if !score.is_changed() {
        return;
    }

    for mut text in &mut displays {
        text.0 = format!("SCORE: {:05}   |   SPACE: FIRE", score.0);
    }
}
