use crate::prelude::*;

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_aliens);
        app.add_systems(Update, (update_aliens, manage_alien_logic));
    }
}

#[derive(Component)]
pub struct Alien;

// Control the behavior of our aliens
#[derive(Resource)]
pub struct AlienManager {
    pub direction: f32,
    // we increment the aliens vertically when this is true
    pub shift_alien_down: bool,
    // the distance the closest alien to the edge is from the boundary so thet we can correct it to be confined within the boundary
    pub dist_from_boundary: f32,
}

fn setup_aliens(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<Resolution>,
) {
    commands.insert_resource(AlienManager {
        dist_from_boundary: 0.0,
        shift_alien_down: false,
        direction: 1.0,
    });

    let alien_texture: Handle<Image> = asset_server.load("alien.png");

    for x in 0..ALIEN_COLUMNS {
        for y in 0..ALIEN_ROWS {
            let position: Vec3 = Vec3::new(x as f32 * ALIEN_SPACING, y as f32 * ALIEN_SPACING, 0.0)
                - (Vec3::X * ALIEN_COLUMNS as f32 * ALIEN_SPACING * 0.5) // center the aliens on the x axis 
                - (Vec3::Y * ALIEN_ROWS as f32 * ALIEN_SPACING * 1.0) // Displace the aliens below the x axis sop that we can displace them to the top of the nex line
                + (Vec3::Y * resolution.screen_dimensions.y * 0.5); // Displace the aliens to the top of the screen

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(position)
                        // The splat just create a vector with 3 of the same value
                        .with_scale(Vec3::splat(resolution.pixel_ratio)),
                    texture: alien_texture.clone(),
                    ..Default::default()
                },
                Alien {},
            ));
        }
    }
}

fn update_aliens(
    mut alien_query: Query<(&Alien, &mut Transform)>,
    mut alien_manager: ResMut<AlienManager>,
    resolution: Res<Resolution>,
    time: Res<Time>,
) {
    for (alien, mut tranform) in alien_query.iter_mut() {
        // delta_seconds makes it so our aliens move at the speed regardless of framerate; delta_seconds() gives the time between each frame
        tranform.translation.x += time.delta_seconds() * alien_manager.direction * ALIEN_SPEED;

        if tranform.translation.x.abs() > resolution.screen_dimensions.x * 0.5 {
            alien_manager.shift_alien_down = true;
            // calculate the delta x we need to move the alien to get it back into our bounds
            alien_manager.dist_from_boundary =
                resolution.screen_dimensions.x * alien_manager.direction * 0.5
                    - tranform.translation.x;
        }
    }
}

fn manage_alien_logic(
    mut alien_query: Query<(&Alien, &mut Transform)>,
    mut alien_manager: ResMut<AlienManager>,
) {
    if alien_manager.shift_alien_down {
        // reverse direction and move aliens downward
        alien_manager.shift_alien_down = false;
        alien_manager.direction *= -1.0;

        for (alien, mut tranform) in alien_query.iter_mut() {
            tranform.translation.x += alien_manager.dist_from_boundary;
            tranform.translation.y -= ALIEN_SHIFT_AMOUNT;
        }
    }
}
