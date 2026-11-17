mod cockpit;

use bevy::{
    core_pipeline::fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin},
    mesh::VertexAttributeValues,
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum RockKind {
    Destructible,
    Indestructible,
}

#[derive(Component)]
struct Asteroid {
    speed: f32,
    kind: RockKind,
    hp: u8,
    max_hp: u8,
}

#[derive(Component)]
struct Star {
    speed: f32,
}

const SHIP_SPEED: f32 = 9.0;
const ASTEROID_COUNT: usize = 20;
const STAR_COUNT: usize = 180;
const ROCK_HP_SEGMENTS: usize = 3;

// Простые псевдослучайные числа.
// Позже сделаем нормальный генератор.
fn random(seed: f32) -> f32 {
    (seed.sin() * 43758.5453).fract().abs()
}

// Low-poly rocks with deterministic uneven surfaces.
// Deforming the same vertex coordinates consistently keeps each mesh intact.
fn make_rock_mesh(seed: f32) -> Mesh {
    let mut mesh = Sphere::new(1.0).mesh().ico(2).expect("valid icosphere");
    if let Some(VertexAttributeValues::Float32x3(vertices)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for point in vertices.iter_mut() {
            let p = Vec3::from_array(*point);
            let coarse = (p.x * 5.1 + seed).sin() * (p.y * 4.3 - seed * 0.71).cos();
            let fine = (p.z * 9.0 + p.x * 6.5 + seed * 2.0).sin();
            let displacement = 0.90 + 0.20 * coarse + 0.10 * fine;
            *point = (p * displacement).to_array();
        }
    }
    mesh
}

fn main() {
    App::new()
        .add_plugins(cockpit::CockpitPlugin)
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Rusteriods | Asteroid Fields v0.9".into(),
                        resolution: WindowResolution::new(1920, 1080),
                        ..default()
                    }),
                    ..default()
                }),
            FullscreenMaterialPlugin::<AsciiEffect>::default(),
        ))
        .init_resource::<Score>()
        .init_resource::<ArcadeState>()
        .init_resource::<MissionState>()
        .init_resource::<WeaponState>()
        .add_systems(Startup, (setup, setup_combat, setup_cockpit).chain())
        .add_systems(
            Update,
            (
                toggle_pause,
                restart_game,
                advance_arcade,
                move_ship,
                update_mission,
                move_asteroids,
                move_stars,
                update_weapon,
                spawn_powerups,
                move_powerups,
                shoot,
                move_lasers,
                check_hits,
                collide_ship,
                check_lock_on,
                update_explosions,
                update_hud,
                update_mission_hud,
                animate_cockpit_lamps,
            )
                .chain(),
        )
        .add_systems(Update, update_rock_hp_bars.after(move_asteroids))
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

    // Several differently shaped asteroid meshes are reused by all rocks.
    let rock_meshes: Vec<Handle<Mesh>> = (0..8)
        .map(|i| meshes.add(make_rock_mesh(i as f32 * 2.71 + 0.3)))
        .collect();

    let ordinary_rock = materials.add(StandardMaterial {
        base_color: Color::srgb(0.65, 0.65, 0.68),
        unlit: true,
        ..default()
    });

    let dark_rock = materials.add(StandardMaterial {
        base_color: Color::srgb(0.45, 0.50, 0.58),
        unlit: true,
        ..default()
    });

    let armored_rock = materials.add(StandardMaterial {
        base_color: Color::srgb(0.28, 0.50, 0.65),
        unlit: true,
        ..default()
    });

    for i in 0..ASTEROID_COUNT {
        let n = i as f32;
        let (x, y) = formation_position(i, 0);
        let z = -12.0 - n * 3.5;
        let size = 0.45 + random(n * 9.2 + 2.0) * 0.65;
        let kind = if i % 6 == 0 {
            RockKind::Indestructible
        } else {
            RockKind::Destructible
        };
        let hp = if kind == RockKind::Indestructible {
            255
        } else {
            1 + (random(n * 31.73 + 3.6) * 3.0).floor() as u8
        };
        let material = if kind == RockKind::Indestructible {
            armored_rock.clone()
        } else if i % 2 == 0 {
            ordinary_rock.clone()
        } else {
            dark_rock.clone()
        };
        let rock_entity = commands
            .spawn((
                Asteroid {
                    speed: 7.0 + random(n + 1.0) * 4.0,
                    kind,
                    hp,
                    max_hp: hp,
                },
                Mesh3d(rock_meshes[i % rock_meshes.len()].clone()),
                MeshMaterial3d(material),
                Transform::from_xyz(x, y, z)
                    .with_scale(Vec3::new(
                        size * (0.85 + random(n * 2.1) * 0.35),
                        size * (0.75 + random(n * 3.7) * 0.40),
                        size * (0.85 + random(n * 5.9) * 0.40),
                    ))
                    .with_rotation(Quat::from_euler(
                        EulerRot::XYZ,
                        n * 0.51,
                        n * 0.27,
                        n * 0.19,
                    )),
            ))
            .id();

        if kind == RockKind::Destructible {
            for segment in 0..ROCK_HP_SEGMENTS {
                commands.spawn((
                    RockHpSegment {
                        rock: rock_entity,
                        segment,
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        width: px(8.0),
                        height: px(4.0),
                        display: Display::None,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.05, 0.9, 0.25)),
                    GlobalZIndex(20),
                ));
            }
        }
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
    arcade: Res<ArcadeState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut ships: Query<&mut Transform, With<Ship>>,
) {
    if arcade.game_over || arcade.paused {
        return;
    }
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

fn move_asteroids(
    time: Res<Time>,
    arcade: Res<ArcadeState>,
    mut asteroids: Query<(&mut Transform, &mut Asteroid)>,
) {
    if arcade.game_over || arcade.paused {
        return;
    }
    for (mut transform, mut asteroid) in &mut asteroids {
        // Летят к камере
        transform.translation.z += asteroid.speed * arcade.difficulty() * time.delta_secs();

        transform.rotate_y(0.5 * time.delta_secs());

        transform.rotate_x(0.3 * time.delta_secs());

        // Возвращаем астероид вдаль
        if transform.translation.z > 12.0 {
            transform.translation.z = -145.0;
            if asteroid.kind == RockKind::Destructible {
                let seed = arcade.elapsed * 1.73 + asteroid.speed * 13.1;
                asteroid.max_hp = 1 + (random(seed) * 3.0).floor() as u8;
                asteroid.hp = asteroid.max_hp;
            }

            let index = (asteroid.speed * 10.0) as usize;
            let wave = (arcade.elapsed / 25.0) as usize;
            let (x, y) = formation_position(index, wave);
            transform.translation.x = x;
            transform.translation.y = y;
        }
    }
}

// ========================================
// STAR MOVEMENT
// ========================================

fn move_stars(
    time: Res<Time>,
    arcade: Res<ArcadeState>,
    mut stars: Query<(&mut Transform, &Star)>,
) {
    if arcade.game_over || arcade.paused {
        return;
    }
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

// ========================================
// RUSTERIODS v0.5 - MISSION / DANGER
// ========================================

// Different formations instead of placing everything uniformly at random.
// Gap between lanes: player can dodge around dense groups.
fn formation_position(index: usize, wave: usize) -> (f32, f32) {
    let i = index as f32;
    match wave % 4 {
        0 => {
            // Two dense lanes, safe center (but shooting requires moving)
            let side = if index % 2 == 0 { -1.0 } else { 1.0 };
            (
                side * (3.2 + (index % 3) as f32 * 1.7),
                (random(i * 5.37 + wave as f32) - 0.5) * 8.0,
            )
        }
        1 => {
            // Diagonal stagger
            let col = (index % 6) as f32;
            (
                -7.5 + col * 3.0,
                -3.5 + ((index / 6 + index % 6) % 4) as f32 * 2.3,
            )
        }
        2 => {
            // Central pressure: dodge to the edges
            (
                (random(i * 12.3 + wave as f32 * 7.0) - 0.5) * 8.0,
                (random(i * 7.9 + wave as f32 * 5.0) - 0.5) * 7.0,
            )
        }
        _ => {
            // Scattered field
            (
                (random(i * 12.3 + wave as f32 * 7.0) - 0.5) * 16.0,
                (random(i * 7.9 + wave as f32 * 5.0) - 0.5) * 9.0,
            )
        }
    }
}

#[derive(Resource)]
struct MissionState {
    motion_last: Vec3,
    motion_timer: f32,
    lock_on: f32,
}

impl Default for MissionState {
    fn default() -> Self {
        Self {
            motion_last: Vec3::ZERO,
            motion_timer: 0.0,
            lock_on: 0.0,
        }
    }
}

#[derive(Component)]
struct MissionDisplay;

fn update_mission(time: Res<Time>, arcade: Res<ArcadeState>, mut mission: ResMut<MissionState>) {
    if arcade.game_over || arcade.paused {
        return;
    }
    mission.motion_timer += time.delta_secs();
}

// Anti-camping: after three seconds near the same point the enemy locks on.
// A short dodge resets the warning, so continuous movement is rewarded.
fn check_lock_on(
    time: Res<Time>,
    mut commands: Commands,
    mut arcade: ResMut<ArcadeState>,
    mut mission: ResMut<MissionState>,
    score: Res<Score>,
    ships: Query<&Transform, With<Ship>>,
    assets: Res<CombatAssets>,
) {
    if arcade.game_over || arcade.paused {
        return;
    }
    let Ok(ship) = ships.single() else {
        return;
    };
    if mission.motion_timer >= 0.35 {
        if ship.translation.distance(mission.motion_last) > 0.55 {
            mission.lock_on = 0.0;
        }
        mission.motion_last = ship.translation;
        mission.motion_timer = 0.0;
    }
    // Ignore the opening invulnerability window.
    if arcade.elapsed < 3.0 {
        return;
    }
    mission.lock_on += time.delta_secs();
    // Moving prevents the charge from reaching its threshold.
    if mission.lock_on >= 3.2 {
        mission.lock_on = 0.0;
        if arcade.invulnerability <= 0.0 {
            spawn_explosion(&mut commands, &assets, ship.translation);
            arcade.lives = arcade.lives.saturating_sub(1);
            arcade.invulnerability = 2.0;
            if arcade.lives == 0 {
                arcade.game_over = true;
                arcade.best = arcade.best.max(score.0);
            }
        }
    }
}

fn update_mission_hud(
    arcade: Res<ArcadeState>,
    mission: Res<MissionState>,
    mut displays: Query<&mut Text, With<MissionDisplay>>,
) {
    let warning = if mission.lock_on > 2.1 {
        "!!! TARGET LOCK — DODGE NOW !!!"
    } else if mission.lock_on > 1.1 {
        "WARNING: INCOMING TARGET LOCK"
    } else {
        "WASD MOVE | SPACE SINGLE SHOT | R RELOAD | P PAUSE"
    };
    for mut text in &mut displays {
        text.0 = if arcade.game_over {
            String::new()
        } else {
            warning.into()
        };
    }
}

use bevy::text::FontSize;

// Игровой счёт
#[derive(Resource, Default)]
struct Score(u32);

// Three small UI segments that follow an individual destructible asteroid.
#[derive(Component)]
struct RockHpSegment {
    rock: Entity,
    segment: usize,
}

// Метка для текста со счётом
#[derive(Component)]
struct ScoreDisplay;

#[derive(Component)]
struct GameOverDisplay;

#[derive(Resource)]
struct ArcadeState {
    lives: u32,
    invulnerability: f32,
    elapsed: f32,
    game_over: bool,
    paused: bool,
    best: u32,
}

impl Default for ArcadeState {
    fn default() -> Self {
        Self {
            lives: 3,
            invulnerability: 2.0,
            elapsed: 0.0,
            game_over: false,
            paused: false,
            best: 0,
        }
    }
}

impl ArcadeState {
    fn difficulty(&self) -> f32 {
        (1.0 + self.elapsed / 120.0).min(1.65)
    }
}

// ========================================
// WEAPON: ammo, reload and temporary rapid fire
// ========================================
const MAG_SIZE: u32 = 12;
const RELOAD_SECS: f32 = 1.35;
const AUTO_SECS: f32 = 8.0;

#[derive(Resource)]
struct WeaponState {
    ammo: u32,
    reloading: f32,
    auto_time: f32,
    shot_cooldown: f32,
    pickup_timer: f32,
    pickup_number: u32,
}

impl Default for WeaponState {
    fn default() -> Self {
        Self {
            ammo: MAG_SIZE,
            reloading: 0.0,
            auto_time: 0.0,
            shot_cooldown: 0.0,
            pickup_timer: 12.0,
            pickup_number: 0,
        }
    }
}

#[derive(Component)]
struct RapidFirePickup;

#[derive(Resource)]
struct PickupAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

fn update_weapon(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    arcade: Res<ArcadeState>,
    mut weapon: ResMut<WeaponState>,
) {
    if arcade.paused || arcade.game_over {
        return;
    }
    let dt = time.delta_secs();
    weapon.shot_cooldown = (weapon.shot_cooldown - dt).max(0.0);
    if weapon.auto_time > 0.0 {
        weapon.auto_time = (weapon.auto_time - dt).max(0.0);
        if weapon.auto_time > 0.0 {
            return;
        }
    }
    if weapon.reloading > 0.0 {
        weapon.reloading = (weapon.reloading - dt).max(0.0);
        if weapon.reloading == 0.0 {
            weapon.ammo = MAG_SIZE;
        }
        return;
    }
    if keyboard.just_pressed(KeyCode::KeyR) || weapon.ammo == 0 {
        weapon.reloading = RELOAD_SECS;
    }
}

fn spawn_powerups(
    mut commands: Commands,
    time: Res<Time>,
    arcade: Res<ArcadeState>,
    mut weapon: ResMut<WeaponState>,
    assets: Res<PickupAssets>,
    existing: Query<Entity, With<RapidFirePickup>>,
) {
    if arcade.paused || arcade.game_over {
        return;
    }
    if !existing.is_empty() || weapon.auto_time > 0.0 {
        return;
    }
    weapon.pickup_timer -= time.delta_secs();
    if weapon.pickup_timer > 0.0 {
        return;
    }
    let n = weapon.pickup_number as f32;
    weapon.pickup_number += 1;
    weapon.pickup_timer = 17.0;
    let x = (random(n * 7.23 + 4.1) - 0.5) * 10.0;
    let y = (random(n * 9.42 + 2.7) - 0.5) * 5.0;
    commands.spawn((
        RapidFirePickup,
        Mesh3d(assets.mesh.clone()),
        MeshMaterial3d(assets.material.clone()),
        Transform::from_xyz(x, y, -55.0),
    ));
}

fn move_powerups(
    mut commands: Commands,
    time: Res<Time>,
    arcade: Res<ArcadeState>,
    mut weapon: ResMut<WeaponState>,
    ships: Query<&Transform, (With<Ship>, Without<RapidFirePickup>)>,
    mut pickups: Query<(Entity, &mut Transform), (With<RapidFirePickup>, Without<Ship>)>,
) {
    if arcade.paused || arcade.game_over {
        return;
    }
    let Ok(ship) = ships.single() else {
        return;
    };
    for (entity, mut transform) in &mut pickups {
        transform.translation.z += 15.0 * time.delta_secs();
        transform.rotate_y(time.delta_secs() * 2.5);
        transform.rotate_z(time.delta_secs() * 1.5);
        if ship.translation.distance(transform.translation) < 2.1 {
            weapon.auto_time = AUTO_SECS;
            weapon.reloading = 0.0;
            weapon.shot_cooldown = 0.0;
            commands.entity(entity).despawn();
        } else if transform.translation.z > 12.0 {
            commands.entity(entity).despawn();
            weapon.pickup_timer = 8.0;
        }
    }
}

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

    let pickup_mesh = meshes.add(Sphere::new(0.85));
    let pickup_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.8, 0.1),
        emissive: Color::srgb(6.0, 4.0, 0.2).into(),
        unlit: true,
        ..default()
    });
    commands.insert_resource(PickupAssets {
        mesh: pickup_mesh,
        material: pickup_material,
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
        Text::new("SCORE: 00000   |   HP: 3   |   WAVE: 1   |   SPACE: FIRE"),
        TextFont {
            font_size: FontSize::Px(26.0),
            ..default()
        },
        TextColor(Color::srgb(0.0, 1.0, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            top: percent(10.0),
            left: percent(12.0),
            ..default()
        },
    ));

    // Gameplay warnings and control hints.
    commands.spawn((
        MissionDisplay,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(23.0),
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.45, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            bottom: percent(14.0),
            left: percent(12.0),
            ..default()
        },
    ));

    // Отдельный экран Game Over поверх 3D-сцены
    commands.spawn((
        GameOverDisplay,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(48.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.35, 0.55)),
        Node {
            position_type: PositionType::Absolute,
            top: percent(42.0),
            left: percent(30.0),
            ..default()
        },
    ));
}

// ----------------------------------------
// СТРЕЛЬБА
// ----------------------------------------

fn shoot(
    arcade: Res<ArcadeState>,
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    ships: Query<&Transform, With<Ship>>,
    assets: Res<CombatAssets>,
    mut weapon: ResMut<WeaponState>,
) {
    if arcade.game_over || arcade.paused || weapon.reloading > 0.0 {
        return;
    }
    let auto = weapon.auto_time > 0.0;
    let wants_shot = if auto {
        keyboard.pressed(KeyCode::Space)
    } else {
        keyboard.just_pressed(KeyCode::Space)
    };
    if !wants_shot || weapon.shot_cooldown > 0.0 {
        return;
    }
    if !auto && weapon.ammo == 0 {
        return;
    }
    let Ok(ship) = ships.single() else {
        return;
    };
    weapon.shot_cooldown = if auto { 0.09 } else { 0.23 };
    if !auto {
        weapon.ammo -= 1;
    }
    // Normal mode: one centered laser per key press.
    // Power-up: twin automatic wing lasers while Space is held.
    let offsets: &[f32] = if auto { &[-1.65, 1.65] } else { &[0.0] };
    for &x in offsets {
        let offset = Vec3::new(x, 0.0, -1.2);
        commands.spawn((
            Laser,
            Mesh3d(assets.laser_mesh.clone()),
            MeshMaterial3d(assets.laser_material.clone()),
            Transform::from_translation(ship.translation + ship.rotation * offset),
        ));
    }
}

// ----------------------------------------
// ДВИЖЕНИЕ ЛАЗЕРОВ
// ----------------------------------------

fn move_lasers(
    arcade: Res<ArcadeState>,
    mut commands: Commands,
    time: Res<Time>,
    mut lasers: Query<(Entity, &mut Transform), With<Laser>>,
) {
    if arcade.game_over || arcade.paused {
        return;
    }
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
    arcade: Res<ArcadeState>,
    mut commands: Commands,
    lasers: Query<(Entity, &Transform), (With<Laser>, Without<Asteroid>)>,
    mut asteroids: Query<(&mut Transform, &mut Asteroid)>,
    mut score: ResMut<Score>,
    assets: Res<CombatAssets>,
) {
    if arcade.game_over || arcade.paused {
        return;
    }
    for (laser_entity, laser_transform) in &lasers {
        let laser_position = laser_transform.translation;
        for (mut rock_transform, mut rock) in &mut asteroids {
            let position = rock_transform.translation;
            let radius = rock_transform.scale.max_element() * 0.95;
            if laser_position.distance(position) < radius + 0.45 {
                commands.entity(laser_entity).despawn();
                if rock.kind == RockKind::Indestructible {
                    // Armor absorbs the shot: no score and no respawn.
                    // A small impact burst makes the hit noticeable.
                    spawn_explosion(&mut commands, &assets, laser_position);
                } else {
                    rock.hp = rock.hp.saturating_sub(1);
                    if rock.hp == 0 {
                        score.0 += if rock.max_hp > 1 { 200 } else { 100 };
                        spawn_explosion(&mut commands, &assets, position);
                        let (x, y) = formation_position(
                            (rock.speed * 10.0) as usize,
                            (arcade.elapsed / 25.0) as usize,
                        );
                        rock_transform.translation = Vec3::new(x, y, -150.0);
                        let seed = arcade.elapsed * 2.47 + rock.speed * 19.7;
                        rock.max_hp = 1 + (random(seed) * 3.0).floor() as u8;
                        rock.hp = rock.max_hp;
                    } else {
                        spawn_explosion(&mut commands, &assets, laser_position);
                    }
                }
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
    arcade: Res<ArcadeState>,
    mut commands: Commands,
    time: Res<Time>,
    mut explosions: Query<(Entity, &mut Transform, &mut Explosion)>,
) {
    if arcade.paused {
        return;
    }
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

// ========================================
// ARCADE GAME LOOP
// ========================================

fn toggle_pause(keyboard: Res<ButtonInput<KeyCode>>, mut arcade: ResMut<ArcadeState>) {
    if !arcade.game_over && keyboard.just_pressed(KeyCode::KeyP) {
        arcade.paused = !arcade.paused;
    }
}

fn advance_arcade(time: Res<Time>, mut arcade: ResMut<ArcadeState>) {
    if arcade.game_over || arcade.paused {
        return;
    }
    arcade.elapsed += time.delta_secs();
    arcade.invulnerability = (arcade.invulnerability - time.delta_secs()).max(0.0);
}

fn collide_ship(
    mut commands: Commands,
    mut arcade: ResMut<ArcadeState>,
    score: Res<Score>,
    ships: Query<&Transform, (With<Ship>, Without<Asteroid>)>,
    mut asteroids: Query<(&mut Transform, &mut Asteroid)>,
    assets: Res<CombatAssets>,
) {
    if arcade.game_over || arcade.paused || arcade.invulnerability > 0.0 {
        return;
    }
    let Ok(ship) = ships.single() else {
        return;
    };
    let ship_pos = ship.translation;
    for (mut asteroid_transform, mut asteroid) in &mut asteroids {
        // Проверяем столкновение около корпуса, не возле кончиков крыльев
        let radius = asteroid_transform.scale.max_element() * 0.95 + 0.85;
        if ship_pos.distance(asteroid_transform.translation) < radius {
            spawn_explosion(&mut commands, &assets, ship_pos);
            arcade.lives = arcade.lives.saturating_sub(1);
            arcade.invulnerability = 2.0;
            let (x, y) = formation_position(
                (asteroid.speed * 10.0) as usize,
                (arcade.elapsed / 25.0) as usize,
            );
            asteroid_transform.translation = Vec3::new(x, y, -145.0);
            if asteroid.kind == RockKind::Destructible {
                asteroid.hp = asteroid.max_hp;
            }
            if arcade.lives == 0 {
                arcade.game_over = true;
                arcade.best = arcade.best.max(score.0);
            }
            break;
        }
    }
}

fn restart_game(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut arcade: ResMut<ArcadeState>,
    mut score: ResMut<Score>,
    mut ship: Query<(&mut Transform, &mut Visibility), With<Ship>>,
    mut asteroids: Query<(&mut Transform, &mut Asteroid), (With<Asteroid>, Without<Ship>)>,
    mut commands: Commands,
    lasers: Query<Entity, With<Laser>>,
    explosions: Query<Entity, With<Explosion>>,
    mut mission: ResMut<MissionState>,
    mut weapon: ResMut<WeaponState>,
    pickups: Query<Entity, With<RapidFirePickup>>,
) {
    if !arcade.game_over || !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }
    arcade.best = arcade.best.max(score.0);
    *mission = MissionState::default();
    *weapon = WeaponState::default();
    for entity in &pickups {
        commands.entity(entity).despawn();
    }
    arcade.lives = 3;
    arcade.invulnerability = 2.0;
    arcade.elapsed = 0.0;
    arcade.game_over = false;
    arcade.paused = false;
    score.0 = 0;
    if let Ok((mut transform, mut visibility)) = ship.single_mut() {
        *transform = Transform::default();
        *visibility = Visibility::Visible;
    }
    for (index, (mut transform, mut asteroid)) in asteroids.iter_mut().enumerate() {
        let n = index as f32;
        let (x, y) = formation_position(index, 0);
        transform.translation = Vec3::new(x, y, -38.0 - n * 4.5 - asteroid.speed * 0.05);
        if asteroid.kind == RockKind::Destructible {
            asteroid.max_hp = 1 + (random(n * 31.73 + 3.6) * 3.0).floor() as u8;
        }
        asteroid.hp = asteroid.max_hp;
    }
    for entity in &lasers {
        commands.entity(entity).despawn();
    }
    for entity in &explosions {
        commands.entity(entity).despawn();
    }
}

fn update_hud(
    arcade: Res<ArcadeState>,
    score: Res<Score>,
    weapon: Res<WeaponState>,
    mut displays: Query<&mut Text, (With<ScoreDisplay>, Without<GameOverDisplay>)>,
    mut overlays: Query<&mut Text, (With<GameOverDisplay>, Without<ScoreDisplay>)>,
    mut ship: Query<&mut Visibility, With<Ship>>,
) {
    let wave = 1 + (arcade.elapsed / 30.0) as u32;
    let ammo_label = if weapon.auto_time > 0.0 {
        format!("AUTO {:.1}s", weapon.auto_time)
    } else if weapon.reloading > 0.0 {
        format!("RELOADING {:.1}s", weapon.reloading)
    } else {
        format!("AMMO {}/{}", weapon.ammo, MAG_SIZE)
    };
    for mut text in &mut displays {
        text.0 = format!(
            "SCORE: {:05} | HP: {} | WAVE: {} | {} | R: RELOAD",
            score.0, arcade.lives, wave, ammo_label
        );
    }
    for mut text in &mut overlays {
        text.0 = if arcade.paused {
            "PAUSED\nPRESS P TO RESUME".into()
        } else if arcade.game_over {
            format!(
                "GAME OVER\nSCORE: {}\nBEST: {}\nPRESS R TO RESTART",
                score.0, arcade.best
            )
        } else {
            String::new()
        };
    }
    if let Ok(mut visibility) = ship.single_mut() {
        *visibility = if arcade.game_over {
            Visibility::Hidden
        } else if arcade.invulnerability > 0.0 && (arcade.invulnerability * 8.0) as i32 % 2 == 0 {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
}

// ========================================
// RUSTERIODS v0.7 — PIXEL-ART COCKPIT
// ========================================
// The PNG has transparent pixels in its central display area.
// Our existing Bevy 3D game stays visible and interactive underneath it.

#[derive(Component, Clone, Copy)]
enum CockpitLamp {
    Fire,
    Warning,
    Health,
    Pause,
}

fn setup_cockpit(mut commands: Commands, asset_server: Res<AssetServer>) {
    // The 16:9 image fills the entire window. Transparent center reveals 3D.
    // This UI node is spawned after the HUD so the frame covers its edges.
    commands.spawn((
        ImageNode::new(asset_server.load("ui/console_frame.png")),
        Node {
            position_type: PositionType::Absolute,
            left: percent(0.0),
            top: percent(0.0),
            width: percent(100.0),
            height: percent(100.0),
            ..default()
        },
    ));

    // Four small status lights are drawn over corresponding bezel areas.
    // Their colors are updated from live gameplay state below.
    for (kind, x, y) in [
        (CockpitLamp::Fire, 4.15_f32, 8.1_f32),
        (CockpitLamp::Warning, 95.75, 27.15),
        (CockpitLamp::Health, 95.75, 20.75),
        (CockpitLamp::Pause, 25.6, 93.7),
    ] {
        commands.spawn((
            kind,
            Node {
                position_type: PositionType::Absolute,
                left: percent(x),
                top: percent(y),
                width: percent(0.85),
                height: percent(1.5),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.9, 0.25, 0.2)),
        ));
    }
}

fn animate_cockpit_lamps(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    arcade: Res<ArcadeState>,
    mission: Res<MissionState>,
    mut lamps: Query<(&CockpitLamp, &mut BackgroundColor)>,
) {
    let flash = (time.elapsed_secs() * 14.0).sin() > 0.0;

    for (kind, mut background) in &mut lamps {
        let color = match kind {
            CockpitLamp::Fire if keyboard.pressed(KeyCode::Space) && !arcade.paused => {
                Color::srgba(1.0, 0.7, 0.06, 0.95)
            }
            CockpitLamp::Fire => Color::srgba(0.3, 0.8, 0.25, 0.12),
            CockpitLamp::Warning if (mission.lock_on > 1.1 || arcade.game_over) && flash => {
                Color::srgba(1.0, 0.08, 0.02, 0.95)
            }
            CockpitLamp::Warning => Color::srgba(1.0, 0.65, 0.0, 0.1),
            CockpitLamp::Health if arcade.lives <= 1 && flash => Color::srgba(1.0, 0.06, 0.02, 0.9),
            CockpitLamp::Health => Color::srgba(0.0, 1.0, 0.15, 0.3),
            CockpitLamp::Pause if arcade.paused => Color::srgba(1.0, 0.7, 0.08, 0.95),
            CockpitLamp::Pause => Color::srgba(0.1, 0.9, 0.3, 0.15),
        };
        background.0 = color;
    }
}

// Rock HP bars live in screen coordinates but follow 3D asteroids.
// The central cockpit screen stays clear of other interface elements.
fn update_rock_hp_bars(
    arcade: Res<ArcadeState>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    asteroids: Query<(&Transform, &Asteroid)>,
    mut segments: Query<(&RockHpSegment, &mut Node, &mut BackgroundColor)>,
) {
    let Ok((camera, camera_transform)) = camera.single() else {
        return;
    };

    for (bar, mut node, mut background) in &mut segments {
        node.display = Display::None;
        if arcade.game_over {
            continue;
        }
        let Ok((rock_transform, rock)) = asteroids.get(bar.rock) else {
            continue;
        };
        if rock.kind == RockKind::Indestructible || rock.hp == 0 || rock.hp >= rock.max_hp {
            continue;
        }

        // The second projection puts the bar just above the visible rock.
        let world_above = rock_transform.translation + Vec3::Y * (rock_transform.scale.y * 1.35);
        let Ok(center) = camera.world_to_viewport(camera_transform, rock_transform.translation)
        else {
            continue;
        };
        let Ok(top) = camera.world_to_viewport(camera_transform, world_above) else {
            continue;
        };

        // Bezel-free playfield for a 1920x1080 window; adapts to resizing.
        let Some(size) = camera.logical_viewport_size() else {
            continue;
        };
        if center.x < size.x * 0.09
            || center.x > size.x * 0.91
            || center.y < size.y * 0.14
            || center.y > size.y * 0.86
            || rock_transform.translation.z > 10.0
        {
            continue;
        }

        let segment_width = 8.0;
        let gap = 2.0;
        let full_width =
            ROCK_HP_SEGMENTS as f32 * segment_width + (ROCK_HP_SEGMENTS as f32 - 1.0) * gap;
        let x = center.x - full_width * 0.5 + bar.segment as f32 * (segment_width + gap);
        let y = top.y - 12.0;
        if y < size.y * 0.12 {
            continue;
        }
        node.left = px(x);
        node.top = px(y);
        node.display = Display::Flex;

        let ratio = rock.hp as f32 / rock.max_hp.max(1) as f32;
        let color = if ratio > 0.66 {
            Color::srgb(0.1, 0.95, 0.2)
        } else if ratio > 0.33 {
            Color::srgb(1.0, 0.8, 0.1)
        } else {
            Color::srgb(1.0, 0.15, 0.12)
        };
        background.0 = if bar.segment < rock.hp as usize {
            color
        } else {
            Color::srgb(0.12, 0.14, 0.17)
        };
    }
}
