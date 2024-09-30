use crate::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ResolutionPlugin, AlienPlugin, PlayerPlugin));
        app.add_systems(Startup, setup_scene);
    }
}

// Organize and include other plugins to the GameLoop
fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
}
