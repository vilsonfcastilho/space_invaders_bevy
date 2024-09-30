use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(Update, update_player);
    }
}

#[derive(Component)]
pub struct Player {
    pub shoot_timer: f32,
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<Resolution>,
) {
    let player_image: Handle<Image> = asset_server.load("player.png");

    commands.spawn((
        SpriteBundle {
            texture: player_image,
            transform: Transform::from_xyz(
                0.0,
                -(resolution.screen_dimensions.y * 0.5) + (resolution.pixel_ratio * 5.0),
                0.0,
            )
            .with_scale(Vec3::splat(resolution.pixel_ratio)),
            ..Default::default()
        },
        Player { shoot_timer: 0.0 },
    ));
}

fn update_player(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    resolution: Res<Resolution>,
) {
    let (mut player, mut transform) = player_query.single_mut();

    let mut horizontal: f32 = 0.0;

    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        horizontal += -1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        horizontal += 1.0;
    }

    // Move Player
    transform.translation.x += horizontal * time.delta_seconds() * PLAYER_SPEED;

    // Confine player
    let left_bound: f32 = -resolution.screen_dimensions.x * 0.5;
    let right_bould: f32 = resolution.screen_dimensions.x * 0.5;

    if transform.translation.x > right_bould {
        transform.translation.x = right_bould;
    }
    if transform.translation.x < left_bound {
        transform.translation.x = left_bound;
    }

    player.shoot_timer -= time.delta_seconds();

    if keys.pressed(KeyCode::Space) && player.shoot_timer < 0.0 {}
}
