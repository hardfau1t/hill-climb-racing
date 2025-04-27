use bevy::{
    color::palettes::css::{BLUE, GRAY, RED},
    prelude::*,
};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

const PLAYER_SPEED: f32 = 200.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(GRAY.into()))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, exit_game))
        .run();
}

fn exit_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Mesh2d(meshes.add(Mesh::from(Circle::new(50.))).into()),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::from(BLUE)))),
        Transform::from_translation(Vec3::new(0., 0., 0.)),
        Player,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Mesh::from(Circle::new(50.))).into()),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::from(RED)))),
        Transform::from_translation(Vec3::new(-100., 0., 0.)),
        Enemy,
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, (With<Player>, Without<Enemy>)>,
    mut enemy: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    time: Res<Time>,
) {
    let mut player_transform = player.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    player_transform.translation += direction * PLAYER_SPEED * time.delta_secs();
}
