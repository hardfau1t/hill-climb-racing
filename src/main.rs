use bevy::{
    color::palettes::css::{GREEN, RED},
    math::vec3,
    prelude::*,
};

const WINDOW_WIDTH: f32 = 1200.;
const WINDOW_HEIGHT: f32 = 800.;

const GROUND_Y: f32 = -((WINDOW_HEIGHT / 2.) / 2.);
const CAR_DEFAULT_POSITION: Vec3 = vec3(
    -((3.0 * WINDOW_WIDTH) / (4.0 * 2.0) - (CAR_WIDTH / 2.0)),
    GROUND_Y,
    0.,
);
const EARTH_WIDTH: f32 = WINDOW_WIDTH;
const EARTH_HEIGHT: f32 = WINDOW_HEIGHT / 2.0 + GROUND_Y;
const EARTH_POSITION: Vec3 = vec3(0., (-WINDOW_HEIGHT / 2.0) + (EARTH_HEIGHT / 2.0), 0.);
const CAR_HEIGHT: f32 = WINDOW_HEIGHT / 20.;
const CAR_WIDTH: f32 = WINDOW_WIDTH / 12.;
const DEFAULT_VELOCITY_INCREASE: f32 = WINDOW_HEIGHT;
const GRAVITY_PULL_FACTOR: f32 = DEFAULT_VELOCITY_INCREASE / 40.;

#[derive(Component)]
struct Car;

#[derive(Component)]
struct Earth;

#[derive(Component)]
struct Velocity(f32);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // setup camera
    commands.spawn(Camera2d);

    // spawn car
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(CAR_WIDTH, CAR_HEIGHT))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        Transform {
            translation: CAR_DEFAULT_POSITION,
            ..default()
        },
        Velocity(0.),
        Car,
    ));

    // spawn earth
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            EARTH_WIDTH,
            EARTH_HEIGHT,
        ))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(GREEN))),
        Transform {
            translation: EARTH_POSITION,
            ..default()
        },
        Earth,
    ));
}

fn move_car(
    car_query: Single<(&mut Transform, &mut Velocity), With<Car>>,
    keyboard_inputs: Res<ButtonInput<KeyCode>>,
    timer: Res<Time>,
) {
    let (mut car_pos, mut velocity) = car_query.into_inner();
    for key in keyboard_inputs.get_just_pressed() {
        match key {
            KeyCode::Space => {
                if car_pos.translation.y == GROUND_Y + CAR_HEIGHT / 2. {
                    velocity.0 = DEFAULT_VELOCITY_INCREASE
                }
            }
            _ => debug!("ignoring keypress: {key:?}"),
        }
    }

    velocity.0 -= GRAVITY_PULL_FACTOR;

    car_pos.translation.y += timer.delta_secs() * velocity.0;
    if car_pos.translation.y < GROUND_Y + CAR_HEIGHT / 2.  {
        velocity.0 = 0.;
        car_pos.translation.y = GROUND_Y + CAR_HEIGHT / 2.0;
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
        .add_systems(Startup, setup)
        .add_systems(Update, move_car)
        .run();
}
