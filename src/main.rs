use bevy::{
    color::palettes::css::{BROWN, GREEN, RED},
    math::{
        bounding::{Aabb2d, IntersectsVolume},
        vec3,
    },
    prelude::*,
};

const WINDOW_WIDTH: f32 = 1200.;
const WINDOW_HEIGHT: f32 = 800.;

const JUMP_VELOCITY: f32 = WINDOW_HEIGHT;
const GRAVITY: f32 = -WINDOW_HEIGHT;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    InGame,
    Pause,
}

#[derive(Component)]
struct Car;

impl Car {
    const CAR_DEFAULT_POSITION: Vec3 = vec3(
        -((3.0 * WINDOW_WIDTH) / (4.0 * 2.0) - (Car::WIDTH / 2.0)),
        Earth::GROUND_Y,
        0.,
    );
    const HEIGHT: f32 = WINDOW_HEIGHT / 20.;
    const WIDTH: f32 = WINDOW_WIDTH / 12.;
}

#[derive(Component)]
struct Rock;

impl Rock {
    // constants related rock
    const HEIGHT: f32 = Car::HEIGHT;
    const WIDTH: f32 = Car::WIDTH;
    const VELOCITY: f32 = 200.0;
    const SPAWN_POSITION: Vec3 = vec3(
        (WINDOW_WIDTH + Rock::WIDTH) / 2.,
        Earth::GROUND_Y + Rock::HEIGHT / 2.0,
        0.0,
    );
}

#[derive(Component)]
struct Earth;

impl Earth {
    const GROUND_Y: f32 = -((WINDOW_HEIGHT / 2.) / 2.);
    const WIDTH: f32 = WINDOW_WIDTH;
    const HEIGHT: f32 = WINDOW_HEIGHT / 2.0 + Earth::GROUND_Y;
    const POSITION: Vec3 = vec3(0., (-WINDOW_HEIGHT / 2.0) + (Earth::HEIGHT / 2.0), 0.);
}

#[derive(Component)]
struct JumpVelocity(f32);

#[derive(Component)]
struct PlayButton;

impl PlayButton {
    const WIDTH: f32 = 10.0;
    const HEIGHT: f32 = 10.0;
    const TEXT_SIZE: f32 = 33.0;
}

#[derive(Component)]
struct ExitButton;

impl ExitButton {
    const WIDTH: f32 = 10.0;
    const HEIGHT: f32 = 10.0;
}

fn setup(mut commands: Commands) {
    // setup camera
    commands.spawn(Camera2d);
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // spawn car
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(Car::WIDTH, Car::HEIGHT))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        Transform {
            translation: Car::CAR_DEFAULT_POSITION,
            ..default()
        },
        JumpVelocity(0.),
        Car,
    ));

    // spawn earth
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(Earth::WIDTH, Earth::HEIGHT))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(GREEN))),
        Transform {
            translation: Earth::POSITION,
            ..default()
        },
        Earth,
    ));
}

fn move_car(
    car_query: Single<(&mut Transform, &mut JumpVelocity), With<Car>>,
    keyboard_inputs: Res<ButtonInput<KeyCode>>,
    timer: Res<Time>,
) {
    let (mut car_pos, mut velocity) = car_query.into_inner();
    for key in keyboard_inputs.get_just_pressed() {
        match key {
            KeyCode::Space => {
                if car_pos.translation.y == Earth::GROUND_Y + Car::HEIGHT / 2. {
                    velocity.0 = JUMP_VELOCITY
                }
            }
            _ => debug!("ignoring keypress: {key:?}"),
        }
    }

    velocity.0 += GRAVITY * timer.delta_secs();

    car_pos.translation.y += timer.delta_secs() * velocity.0;
    if car_pos.translation.y < Earth::GROUND_Y + Car::HEIGHT / 2. {
        velocity.0 = 0.;
        car_pos.translation.y = Earth::GROUND_Y + Car::HEIGHT / 2.0;
    }
}

fn move_rocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    rocks_query: Option<Single<(&mut Transform, Entity), With<Rock>>>,
    timer: Res<Time>,
) {
    if let Some(x) = rocks_query {
        let (mut rock, entity) = x.into_inner();
        // no need to iterate twice to calculate length
        rock.translation.x -= Rock::VELOCITY * timer.delta_secs();

        if rock.translation.x + Rock::WIDTH / 2. < -(WINDOW_WIDTH / 2.) {
            commands.entity(entity).despawn();
        }
    } else {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(Rock::WIDTH, Rock::HEIGHT))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(BROWN))),
            Transform {
                translation: Rock::SPAWN_POSITION,
                ..default()
            },
            Rock,
        ));
    }
}

fn check_collision(
    mut next_state: ResMut<NextState<GameState>>,
    rocks_query: Option<Single<&Transform, With<Rock>>>,
    car_query: Single<&Transform, With<Car>>,
) {
    if let Some(rock) = rocks_query {
        let t = rock.translation.truncate();
        let rock_box = Aabb2d::new(
            t,
            Vec2 {
                x: Rock::WIDTH / 2.0,
                y: Rock::HEIGHT / 2.0,
            },
        );
        let car_box = Aabb2d::new(
            car_query.translation.truncate(),
            Vec2 {
                x: Car::WIDTH / 2.0,
                y: Car::HEIGHT / 2.0,
            },
        );
        if rock_box.intersects(&car_box) {
            info!("You are dead, rock: {rock_box:?}, car: {car_box:?}");
            next_state.set(GameState::Menu)
        }
    }
}

#[derive(Resource)]
struct MenuButton {
    buttons: Entity,
}

fn setup_menu(mut commands: Commands) {
    let buttons = commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    PlayButton,
                    Node {
                        width: Val::Percent(PlayButton::WIDTH),
                        height: Val::Percent(PlayButton::HEIGHT),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Play"),
                        TextFont {
                            font_size: PlayButton::TEXT_SIZE,
                            ..default()
                        },
                    ));
                });
            // spawn exit button
            parent
                .spawn((
                    Button,
                    ExitButton,
                    Node {
                        width: Val::Percent(PlayButton::WIDTH),
                        height: Val::Percent(PlayButton::HEIGHT),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Exit"),
                        TextFont {
                            font_size: PlayButton::TEXT_SIZE,
                            ..default()
                        },
                    ));
                });
})
        .id();
    commands.insert_resource(MenuButton { buttons })
}

fn exit_button_interactions(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<ExitButton>, With<Button>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    for interaction in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                exit.send(AppExit::Success);
            }
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}
fn play_button_interactions(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<PlayButton>, With<Button>),
    >,
) {
    for interaction in &mut interaction_query {
        match interaction {
            Interaction::Pressed => next_state.set(GameState::InGame),
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuButton>) {
    commands.entity(menu_data.buttons).despawn_recursive();
}

fn pause_game() {}

fn handle_pause() {}

fn resume_game() {}

fn end_game(
    mut commands: Commands,
    player: Query<Entity, With<Car>>,
    rock: Query<Entity, With<Rock>>,
    earth: Query<Entity, With<Earth>>,
) {
    for player_entity in player.iter() {
        commands.entity(player_entity).despawn()
    }
    for rock_entity in rock.iter() {
        commands.entity(rock_entity).despawn()
    }
    for earth_entity in earth.iter() {
        commands.entity(earth_entity).despawn()
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "hill climb racing".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(
            Update,
            (exit_button_interactions, play_button_interactions).run_if(in_state(GameState::Menu)),
        )
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        .add_systems(OnEnter(GameState::InGame), setup_game)
        .add_systems(
            Update,
            (move_car, move_rocks, check_collision)
                .chain()
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(OnExit(GameState::InGame), end_game)
        .add_systems(OnEnter(GameState::Pause), pause_game)
        .add_systems(Update, handle_pause.run_if(in_state(GameState::Pause)))
        .add_systems(OnExit(GameState::Pause), resume_game)
        .run();
}
