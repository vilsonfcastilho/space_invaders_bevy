mod alien;
mod config;
mod game;
mod player;
mod resolution;

mod prelude {
    pub use bevy::prelude::*;

    pub use crate::alien::*;
    pub use crate::config::*;
    pub use crate::game::*;
    pub use crate::player::*;
    pub use crate::resolution::*;
}

use prelude::*;

fn main() {
    App::new()
        .add_plugins((
            // List of Plugins
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Space Invaders"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            GamePlugin,
        ))
        .run();
}
