use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (file_drag_and_drop_system, end_flag),
        )
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity::<EndFlag>("EndFlag")
        .run();
}

#[derive(Component)]
struct CurrentWorld;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            704. / 2.,
            576. / 2.,
            1.,
        ),
        ..default()
    });

    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: asset_server
                .load("default_level.ldtk"),
            ..Default::default()
        },
        CurrentWorld,
    ));
}

#[derive(Component)]
struct AnimateFlag {
    current_frame: usize,
    num_frames: usize,
    timer: Timer,
    base_index: Option<usize>,
}

impl Default for AnimateFlag {
    fn default() -> Self {
        Self {
            base_index: None,
            current_frame: 0,
            num_frames: 2,
            timer: Timer::from_seconds(
                0.25,
                TimerMode::Repeating,
            ),
        }
    }
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct EndFlag {
    a: AnimateFlag,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}

fn end_flag(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlas, &mut AnimateFlag)>,
) {
    for (mut atlas, mut flag) in &mut query {
        if flag.timer.tick(time.delta()).finished() {
            let base_index = match flag.base_index {
                Some(i) => i,
                None => {
                    flag.base_index = Some(atlas.index);
                    atlas.index
                }
            };
            flag.current_frame =
                (flag.current_frame + 1) % flag.num_frames;
            atlas.index = base_index + flag.current_frame;
        }
    }
}
fn file_drag_and_drop_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<FileDragAndDrop>,
    query_current_world: Query<Entity, With<CurrentWorld>>,
) {
    for event in events.read() {
        match event {
            FileDragAndDrop::DroppedFile {
                window,
                path_buf,
            } => {
                for world in &query_current_world {
                    commands
                        .entity(world)
                        .despawn_recursive();
                }
                if !path_buf
                    .extension()
                    .is_some_and(|ext| ext == "ldtk")
                {
                    continue;
                };

                commands.spawn((
                    LdtkWorldBundle {
                        ldtk_handle: asset_server.load(
                            path_buf.display().to_string(),
                        ),
                        ..Default::default()
                    },
                    CurrentWorld,
                ));
            }
            FileDragAndDrop::HoveredFile {
                window,
                path_buf,
            } => {
                info!("hovering");
            }
            FileDragAndDrop::HoveredFileCanceled {
                window,
            } => {
                info!("cancelled");
            }
        }
    }
}
