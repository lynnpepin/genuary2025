
use bevy::{
  prelude::*, ui::update, 
};


// https://bevyengine.org/examples/2d-rendering/2d-shapes/
pub fn setup(
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
pub fn update(
  time: Res<Time>,
  mut query: Query<()> // e.g. Query<(&ParticleDynamic, &mut Transform)>
) {
  // Practically, you will start every Update system with a Query like this

  //for (foo, mut bar) in &mut query {
  //}
}
