#[cfg(feature = "custom_cursor")]
use bevy::winit::cursor::CustomCursor;
use bevy::{
  core::FrameCount, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, input::{gamepad::{
    GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadButtonStateChangedEvent,
    GamepadConnectionEvent, GamepadEvent,
  }, keyboard::KeyboardInput}, prelude::*, ui::update, window::{CompositeAlphaMode, CursorGrabMode, PresentMode, SystemCursorIcon, WindowLevel, WindowTheme}, winit::cursor::CursorIcon
};

mod print_input;
use print_input::*;
mod window_utils;
use window_utils::*;
use rand::{Rng};

fn main() {
  App::new()
    .add_plugins(
      (DefaultPlugins
        .set(ImagePlugin::default_nearest())
        // see: https://github.com/bevyengine/bevy/blob/main/examples/window/window_settings.rs
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: "Genuary 2025 - Vertical and Horizontal Lines".into(),
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
            visible: false,
            ..default()
          }),
          ..default()
        }),
      )
    )
    .insert_resource(
      ClearColor(Color::NONE) // for transparent window
    ) 
    .add_systems(
      Update,
      (
        gamepad_events,
        gamepad_ordered_events,
        print_keyboard_event_system,
      ),
    )
    .add_systems(
      Update,
      (
        change_title,
        toggle_theme,
        toggle_cursor,
        toggle_vsync,
        toggle_window_controls,
        cycle_cursor_icon,
        switch_level,
        make_visible,
      )
    )
    .add_systems(Startup, setup)
    .add_systems(Update, update)
    .run();
}

// https://bevyengine.org/examples/2d-rendering/2d-shapes/
fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  commands.spawn(
    Camera3d::default()
  );
  let mut rng = rand::thread_rng();
}

// Every entity is modified using a function like this, with a Query,
// where `Query<(&components, &go, &here)>` selects what entities are modified.
fn update(
  time: Res<Time>,
  mut query: Query<()> // e.g. Query<(&ParticleDynamic, &mut Transform)>
) {
  // Practically, you will start every Update system with a Query like this

  //for (foo, mut bar) in &mut query {
  //}
}
