#[cfg(feature = "custom_cursor")]
use bevy::winit::cursor::CustomCursor;
use bevy::{
    core::FrameCount,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{CursorGrabMode, PresentMode, SystemCursorIcon, WindowLevel, WindowTheme},
    winit::cursor::CursorIcon,
};


pub fn make_visible(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
  // The delay may be different for your app or system.
  if frames.0 == 3 {
      // At this point the gpu is ready to show the app so we can make the window visible.
      // Alternatively, you could toggle the visibility in Startup.
      // It will work, but it will have one white frame before it starts rendering
      window.visible = true;
  }
}

/// This system toggles the vsync mode when pressing the button V.
/// You'll see fps increase displayed in the console.
pub fn toggle_vsync(input: Res<ButtonInput<KeyCode>>, mut window: Single<&mut Window>) {
  if input.just_pressed(KeyCode::KeyV) {
      window.present_mode = if matches!(window.present_mode, PresentMode::AutoVsync) {
          PresentMode::AutoNoVsync
      } else {
          PresentMode::AutoVsync
      };
      info!("PRESENT_MODE: {:?}", window.present_mode);
  }
}

/// This system switches the window level when pressing the T button
/// You'll notice it won't be covered by other windows, or will be covered by all the other
/// windows depending on the level.
///
/// This feature only works on some platforms. Please check the
/// [documentation](https://docs.rs/bevy/latest/bevy/prelude/struct.Window.html#structfield.window_level)
/// for more details.
pub fn switch_level(input: Res<ButtonInput<KeyCode>>, mut window: Single<&mut Window>) {
  if input.just_pressed(KeyCode::KeyT) {
      window.window_level = match window.window_level {
          WindowLevel::AlwaysOnBottom => WindowLevel::Normal,
          WindowLevel::Normal => WindowLevel::AlwaysOnTop,
          WindowLevel::AlwaysOnTop => WindowLevel::AlwaysOnBottom,
      };
      info!("WINDOW_LEVEL: {:?}", window.window_level);
  }
}

/// This system toggles the window controls when pressing buttons 1, 2 and 3
///
/// This feature only works on some platforms. Please check the
/// [documentation](https://docs.rs/bevy/latest/bevy/prelude/struct.Window.html#structfield.enabled_buttons)
/// for more details.
pub fn toggle_window_controls(input: Res<ButtonInput<KeyCode>>, mut window: Single<&mut Window>) {
  let toggle_minimize = input.just_pressed(KeyCode::Digit1);
  let toggle_maximize = input.just_pressed(KeyCode::Digit2);
  let toggle_close = input.just_pressed(KeyCode::Digit3);

  if toggle_minimize || toggle_maximize || toggle_close {
      if toggle_minimize {
          window.enabled_buttons.minimize = !window.enabled_buttons.minimize;
      }
      if toggle_maximize {
          window.enabled_buttons.maximize = !window.enabled_buttons.maximize;
      }
      if toggle_close {
          window.enabled_buttons.close = !window.enabled_buttons.close;
      }
  }
}

/// This system will then change the title during execution
pub fn change_title(mut window: Single<&mut Window>, time: Res<Time>) {
  window.title = format!(
      "Seconds since startup: {}",
      time.elapsed().as_secs_f32().round()
  );
}

pub fn toggle_cursor(mut window: Single<&mut Window>, input: Res<ButtonInput<KeyCode>>) {
  if input.just_pressed(KeyCode::Space) {
      window.cursor_options.visible = !window.cursor_options.visible;
      window.cursor_options.grab_mode = match window.cursor_options.grab_mode {
          CursorGrabMode::None => CursorGrabMode::Locked,
          CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
      };
  }
}

// This system will toggle the color theme used by the window
pub fn toggle_theme(mut window: Single<&mut Window>, input: Res<ButtonInput<KeyCode>>) {
  if input.just_pressed(KeyCode::KeyF) {
      if let Some(current_theme) = window.window_theme {
          window.window_theme = match current_theme {
              WindowTheme::Light => Some(WindowTheme::Dark),
              WindowTheme::Dark => Some(WindowTheme::Light),
          };
      }
  }
}

#[derive(Resource)]
pub struct CursorIcons(Vec<CursorIcon>);

pub fn init_cursor_icons(
  mut commands: Commands,
  #[cfg(feature = "custom_cursor")] asset_server: Res<AssetServer>,
) {
  commands.insert_resource(CursorIcons(vec![
      SystemCursorIcon::Default.into(),
      SystemCursorIcon::Pointer.into(),
      SystemCursorIcon::Wait.into(),
      SystemCursorIcon::Text.into(),
      #[cfg(feature = "custom_cursor")]
      CustomCursor::Image {
          handle: asset_server.load("branding/icon.png"),
          hotspot: (128, 128),
      }
      .into(),
  ]));
}

/// This system cycles the cursor's icon through a small set of icons when clicking
pub fn cycle_cursor_icon(
  mut commands: Commands,
  window: Single<Entity, With<Window>>,
  input: Res<ButtonInput<MouseButton>>,
  mut index: Local<usize>,
  cursor_icons: Res<CursorIcons>,
) {
  if input.just_pressed(MouseButton::Left) {
      *index = (*index + 1) % cursor_icons.0.len();
      commands
          .entity(*window)
          .insert(cursor_icons.0[*index].clone());
  } else if input.just_pressed(MouseButton::Right) {
      *index = if *index == 0 {
          cursor_icons.0.len() - 1
      } else {
          *index - 1
      };
      commands
          .entity(*window)
          .insert(cursor_icons.0[*index].clone());
  }
}