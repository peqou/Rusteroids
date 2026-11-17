use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct CockpitPlugin;

impl Plugin for CockpitPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CockpitState>()
            .init_resource::<CalibrationMode>()
            .add_systems(Startup, (setup_calibration, setup_oscilloscope))
            .add_systems(
                Update,
                (update_cockpit, update_calibration, update_oscilloscope),
            );
    }
}

#[derive(Resource)]
pub struct CockpitState {
    pub uptime: f32,
    pub power: bool,
    pub signal: f32,
}

impl Default for CockpitState {
    fn default() -> Self {
        Self {
            uptime: 0.0,
            power: true,
            signal: 1.0,
        }
    }
}

fn update_cockpit(time: Res<Time<Real>>, mut state: ResMut<CockpitState>) {
    state.uptime += time.delta_secs();
    state.signal = if state.power {
        0.75 + 0.25 * (state.uptime * 2.0).sin()
    } else {
        0.0
    };
}

// All measurements are in design-space coordinates, NOT physical display pixels.
const DESIGN_WIDTH: f32 = 1920.0;
const DESIGN_HEIGHT: f32 = 1080.0;

const OSC_X: f32 = 994.0;
const OSC_Y: f32 = 1003.0;

const OSC_WIDTH: f32 = 158.0;
const OSC_HEIGHT: f32 = 49.0;

// Шаг между точками осциллографа
const OSC_STEP: f32 = 2.0;

// Количество точек
const OSC_SAMPLES: usize = 79;

const BUTTON_01_X: f32 = 37.0;
const BUTTON_01_Y: f32 = 84.0;
const BUTTON_01_WIDTH: f32 = 34.0;
const BUTTON_01_HEIGHT: f32 = 33.0;

#[derive(Resource, Default)]
struct CalibrationMode {
    enabled: bool,
}

#[derive(Component)]
struct CoordinateLabel;

#[derive(Component)]
struct CoordinateCursor;

fn setup_calibration(mut commands: Commands) {
    commands.spawn((
        CoordinateLabel,
        Text::new("COCKPIT CALIBRATION"),
        TextFont {
            font_size: bevy::text::FontSize::Px(18.0),
            ..default()
        },
        TextColor(Color::srgb(0.0, 1.0, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            left: px(180),
            bottom: px(125),
            display: Display::None,
            ..default()
        },
        GlobalZIndex(2000),
    ));
    commands.spawn((
        CoordinateCursor,
        Text::new("+"),
        TextFont {
            font_size: bevy::text::FontSize::Px(25.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.2, 0.2)),
        Node {
            position_type: PositionType::Absolute,
            display: Display::None,
            ..default()
        },
        GlobalZIndex(2001),
    ));
}

fn update_calibration(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut mode: ResMut<CalibrationMode>,
    mut labels: Query<(&mut Text, &mut Node), (With<CoordinateLabel>, Without<CoordinateCursor>)>,
    mut cursors: Query<&mut Node, (With<CoordinateCursor>, Without<CoordinateLabel>)>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        mode.enabled = !mode.enabled;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((mut text, mut label)) = labels.single_mut() else {
        return;
    };
    let Ok(mut cursor) = cursors.single_mut() else {
        return;
    };
    let display = if mode.enabled {
        Display::Flex
    } else {
        Display::None
    };
    label.display = display;
    cursor.display = display;
    if !mode.enabled {
        return;
    }
    let Some(pos) = window.cursor_position() else {
        return;
    };
    let x = pos.x / window.width() * DESIGN_WIDTH;
    let y = pos.y / window.height() * DESIGN_HEIGHT;
    text.0 = format!("COCKPIT | X: {:.0} Y: {:.0} | F3: HIDE", x, y);
    cursor.left = px(pos.x - 7.0);
    cursor.top = px(pos.y - 15.0);
    if mouse.just_pressed(MouseButton::Left) {
        info!("COCKPIT POSITION: X={:.0}, Y={:.0}", x, y);
    }
}

#[derive(Component)]
struct OscPanel;

#[derive(Component)]
struct OscPixel {
    index: usize,
}

fn setup_oscilloscope(mut commands: Commands) {
    // A single dark display area and 70 independent 2-pixel-wide samples.
    // It draws over the passive frame artwork, not inside the game world.
    commands
        .spawn((
            OscPanel,
            Node {
                position_type: PositionType::Absolute,
                width: px(OSC_WIDTH),
                height: px(OSC_HEIGHT),
                left: px(OSC_X),
                top: px(OSC_Y),
                ..default()
            },
            BackgroundColor(Color::srgb(0.015, 0.035, 0.020)),
            GlobalZIndex(1500),
        ))
        .with_children(|parent| {
            for index in 0..OSC_SAMPLES {
                parent.spawn((
                    OscPixel { index },
                    Node {
                        position_type: PositionType::Absolute,
                        left: px(index as f32 * OSC_STEP),
                        top: px(OSC_HEIGHT * 0.5),
                        width: px(OSC_STEP),
                        height: px(OSC_STEP),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.20, 1.0, 0.20)),
                ));
            }
        });
}

fn update_oscilloscope(
    windows: Query<&Window, With<PrimaryWindow>>,
    cockpit: Res<CockpitState>,
    mut panel: Query<&mut Node, (With<OscPanel>, Without<OscPixel>)>,
    mut pixels: Query<(&OscPixel, &mut Node, &mut BackgroundColor), Without<OscPanel>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    if window.width() <= 0.0 || window.height() <= 0.0 {
        return;
    }
    let sx = window.width() / DESIGN_WIDTH;
    let sy = window.height() / DESIGN_HEIGHT;

    if let Ok(mut node) = panel.single_mut() {
        node.left = px(OSC_X * sx);
        node.top = px(OSC_Y * sy);
        node.width = px(OSC_WIDTH * sx);
        node.height = px(OSC_HEIGHT * sy);
    }

    for (sample, mut node, mut color) in &mut pixels {
        let x = sample.index as f32;
        let t = cockpit.uptime;
        // Pixel-art wave, clamped so it cannot spill into the bezel.
        let y = (15.0 + 8.0 * (x * 0.21 - t * 5.0).sin() + 3.0 * (x * 0.47 + t * 2.3).sin())
            .round()
            .clamp(1.0, 27.0);
        node.left = px(x * OSC_STEP * sx);
        node.top = px(y * sy);
        node.width = px(OSC_STEP * sx);
        node.height = px((OSC_STEP * sy).max(0.5));
        color.0 = if cockpit.power {
            Color::srgb(0.12, 0.68 + cockpit.signal * 0.30, 0.14)
        } else {
            Color::srgb(0.02, 0.07, 0.03)
        };
    }
}
struct CockpitButton {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

const TOP_LEFT_BUTTONS: [CockpitButton; 2] = [
    CockpitButton {
        x: 37.0,
        y: 84.0,
        width: 34.0,
        height: 33.0,
    },
    CockpitButton {
        x: 79.0,
        y: 84.0,
        width: 32.0,
        height: 33.0,
    },
];
