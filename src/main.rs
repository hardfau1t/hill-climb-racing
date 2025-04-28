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
const GRAVITY: f32 = -WINDOW_HEIGHT; // 10px/s2

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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // setup camera
    commands.spawn(Camera2d);

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
    rocks_query: Option<Single<&Transform, With<Rock>>>,
    car_query: Single<&Transform, With<Car>>,
    mut exit: EventWriter<AppExit>,
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
            exit.send(AppExit::Success);
        }
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
        .add_systems(Update, (move_car, move_rocks, check_collision).chain())
        .run();
}
