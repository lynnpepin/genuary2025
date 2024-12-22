use bevy::{
  input::gamepad::{
    GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadButtonStateChangedEvent,
    GamepadConnectionEvent, GamepadEvent,
  },
  input::keyboard::KeyboardInput,
  prelude::*,
};

// https://github.com/bevyengine/bevy/blob/latest/examples/input/gamepad_input.rs
pub fn gamepad_events(
  mut connection_events: EventReader<GamepadConnectionEvent>,
  // Handles the continuous measure of an axis, equivalent to GamepadAxes::get.
  mut axis_changed_events: EventReader<GamepadAxisChangedEvent>,
  // Handles the continuous measure of how far a button has been pressed down, equivalent to `GamepadButtons::get`.
  mut button_changed_events: EventReader<GamepadButtonChangedEvent>,
  // Handles the boolean measure of whether a button is considered pressed or unpressed, as
  // defined by the thresholds in `GamepadSettings::button_settings`.
  // When the threshold is crossed and the button state changes, this event is emitted.
  mut button_input_events: EventReader<GamepadButtonStateChangedEvent>,
) {
  for connection_event in connection_events.read() {
    info!("{:?}", connection_event);
  }
  for axis_changed_event in axis_changed_events.read() {
    info!(
      "{:?} of {:?} is changed to {}",
      axis_changed_event.axis, axis_changed_event.entity, axis_changed_event.value
    );
  }
  for button_changed_event in button_changed_events.read() {
    info!(
      "{:?} of {:?} is changed to {}",
      button_changed_event.button, button_changed_event.entity, button_changed_event.value
    );
  }
  for button_input_event in button_input_events.read() {
    info!("{:?}", button_input_event);
  }
}

// If you require in-frame relative event ordering, you can also read the `Gamepad` event
// stream directly. For standard use-cases, reading the events individually or using the
// `Input<T>` or `Axis<T>` resources is preferable.
pub fn gamepad_ordered_events(mut gamepad_events: EventReader<GamepadEvent>) {
  for gamepad_event in gamepad_events.read() {
    match gamepad_event {
      GamepadEvent::Connection(connection_event) => info!("{:?}", connection_event),
      GamepadEvent::Button(button_event) => info!("{:?}", button_event),
      GamepadEvent::Axis(axis_event) => info!("{:?}", axis_event),
    }
  }
}

// https://github.com/bevyengine/bevy/blob/latest/examples/input/keyboard_input_events.rs
/// This system prints out all keyboard events as they come in
pub fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
  for event in keyboard_input_events.read() {
    info!("{:?}", event);
  }
}

