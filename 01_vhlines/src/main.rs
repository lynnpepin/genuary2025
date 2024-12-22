#[cfg(feature = "custom_cursor")]

use bevy::winit::cursor::CustomCursor;
use bevy::{
  core::FrameCount,diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, input::{gamepad::{
    GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadButtonStateChangedEvent,
    GamepadConnectionEvent, GamepadEvent,
  }, keyboard::KeyboardInput},
  prelude::*, ui::update, window::{CompositeAlphaMode, CursorGrabMode, PresentMode, SystemCursorIcon, WindowLevel, WindowTheme, PrimaryWindow},
  winit::cursor::CursorIcon
};


mod print_input;
use print_input::*;
mod window_utils;
use window_utils::*;
use rand::{distributions::Standard, Rng};
use std::f32::consts::PI;

fn main() {
  App::new()
    .add_plugins(
      (
        DefaultPlugins
        .set(ImagePlugin::default_nearest())
        // see: https://github.com/bevyengine/bevy/blob/main/examples/window/window_settings.rs
        .set(
          WindowPlugin {
            primary_window: Some(
              Window {
                title: "Genuary 2025".into(),
                name: Some("genuary.app".into()),
                resolution: (512., 512.).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Light),
                enabled_buttons: bevy::window::EnabledButtons {
                  maximize: false,
                  minimize: true,
                  close: true
                },
                // see: https://github.com/bevyengine/bevy/blob/main/examples/window/transparent_window.rs
                // Setting `transparent` allows the `ClearColor`'s alpha value to take effect
                transparent: true,
                // Disabling window decorations to make it feel more like a widget than a window
                decorations: true,
                #[cfg(target_os = "macos")]
                composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                #[cfg(target_os = "linux")]
                composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
                // ClearColor must have 0 alpha, otherwise some color will bleed through
                visible: true,
                ..default()
              }
            ),
            ..default()
          }
        ),
      )
    ).insert_resource(
      ClearColor(Color::NONE) // for transparent window
    ).add_systems(Startup, setup)
    .add_systems(Update, update)
    .run();
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {

  let LENGTH = 256.0;
  let WIDTH  = 1./32.0;
  let N      = 10;

  for z in -(2*(N-1))..1 {
    for x in -N..N {
      commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(LENGTH, WIDTH, WIDTH))),
        MeshMaterial3d(
          materials.add(
            StandardMaterial {
              base_color: Color::srgb_u8(255, 192, 64),
              unlit: true,
              ..Default::default()
            }
          )
        ),
        // create a single new transform with translation and rotation
        Transform::from_xyz(0., x as f32, z as f32)
        .with_rotation(Quat::from_rotation_x(PI/4.0)),
        //Transform::from_rotation(Quat::from_rotation_x(PI/4.0)),
      ));
    }
    for y in -N..N {
      commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(WIDTH, LENGTH, WIDTH))),
        MeshMaterial3d(
          materials.add(
            StandardMaterial {
              base_color: Color::srgb_u8(250, 196, 64),
              unlit: true,
              ..Default::default()
            }
          )
        ),
        Transform::from_xyz(y as f32, 0., z as f32)
        .with_rotation(Quat::from_rotation_y(PI/4.0)),
      ));
    }
  }

  // camera
  commands.spawn((
      Camera3d {
          ..Default::default()
      },
      Projection::from(PerspectiveProjection {
        fov: 70.0_f32.to_radians(),
        ..default()
      }),
      Transform::from_xyz(0.3, 0.3, 0.)
      .looking_at(
        Vec3::new(0.3, 0.3, 0.0),
        Vec3::Z
      ),
      DistanceFog {
        color: Color::srgb_u8(30, 29, 57), 
        falloff: FogFalloff::Linear {
            start: 4.0,
            end: 12.0,
        },
        ..default()
      },
  ));
}

// Every entity is modified using a function like this, with a Query,
// where `Query<(&components, &go, &here)>` selects what entities are modified.
fn update(
  time: Res<Time>,
  q_windows: Query<&Window, With<PrimaryWindow>>,
  mut query: Query<(&Camera3d, &mut Transform)> // e.g. Query<(&ParticleDynamic, &mut Transform)>
) {
  // Practically, you will start every Update system with a Query like this

  let (mouse_x, mouse_y) = if let Some(position) = q_windows.single().cursor_position() {
    ((position.x - 256.0) / 64.0, (position.y - 256.0) / 64.0)
  } else {
    (0.0, 0.0)
  };

  for (cam, mut transform) in &mut query {
    transform.translation.x = (transform.translation.x).rem_euclid(1.0);
    transform.translation.y = (transform.translation.y).rem_euclid(1.0);
    transform.translation.z = (transform.translation.z).rem_euclid(1.0);


    transform.translation.z += - 1.0 * time.delta_secs();
    transform.translation.x += mouse_y * time.delta_secs();
    transform.translation.y += mouse_x * time.delta_secs();
  }
  
}
