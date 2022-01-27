use bevy::{prelude::*, sprite::collide_aabb::{Collision, collide}};

// Component | NA        | Component | NA | Component
// Component | Component | Component | NA | Component

#[derive(Default, Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct BallMarker;

#[derive(Bundle)]
pub struct Ball {
    marker: BallMarker,

    #[bundle]
    sprite: SpriteBundle,

    velocity: Velocity,
}

#[derive(Component)]
pub struct PaddleMarker;

#[derive(Bundle)]
pub struct Paddle {
    marker: PaddleMarker,

    #[bundle]
    sprite: SpriteBundle,

    velocity: Velocity,
}

// Resources, Res<Time>
// Queries, Query<(&Component, &Component)>

pub fn kinematics(time: Res<Time>, mut queries: Query<(&mut Transform, &Velocity)>) {
    let delta = time.delta().as_secs_f32();
    for (mut transform, velocity) in queries.iter_mut() {
        transform.translation += velocity.0.extend(0.) * delta;
    }
}

pub fn wall_bounce(mut queries: Query<(&Transform, &mut Velocity)>) {
    for (transform, mut velocity) in queries.iter_mut() {
        if transform.translation.x > WIDTH / 2. || transform.translation.x < -(WIDTH / 2.) {
            velocity.0 *= Vec2::new(-1., 0.);
        }
        if transform.translation.y > HEIGHT / 2. || transform.translation.y < -(HEIGHT / 2.) {
            velocity.0 *= Vec2::new(0., -1.);
        }
    }
}

pub fn collision(
    paddle: Query<(&Transform, &Sprite), With<PaddleMarker>>,
    mut ball: Query<(&Transform, &Sprite, &mut Velocity), With<BallMarker>>,
) {
    for (paddle_transform, paddle_sprite) in paddle.iter() {
        for (ball_transform, sprite, ball_velocity) in paddle.iter_mut() {
            let sprite_size: Vec2 = ;
            let ball_size: Vec2;

            let collision: Collision = collide(
                paddle_transform.translation,
                sprite_size,
                ball_transform.translation,
                ball_size,
            );

            use Collision::*;
            let multiplier = match collision {
                Top => Vec2::new(1., -1.),
                Bottom => Vec::new(1., -1.),
                Left => Vec::new(-1., 1.),
                Right => Vec::new(-1, 1.)
            };
            ball_velocity.0 *= 
        }
    }
}

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 800.0;

fn keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<PaddleMarker>>) {
    const SPEED: f32 = 200.;
    if !keys.is_changed() {
        return;
    }

    let mut velocity = query.single_mut();

    if keys.just_pressed(KeyCode::W) {
        velocity.0 = Vec2::new(0., SPEED);
    }

    if keys.just_pressed(KeyCode::A) {
        velocity.0 = Vec2::new(-SPEED, 0.);
    }

    if keys.just_pressed(KeyCode::S) {
        velocity.0 = Vec2::new(0., -SPEED);
    }

    if keys.just_pressed(KeyCode::D) {
        velocity.0 = Vec2::new(SPEED, 0.);
    }
}

pub fn setup(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Ball
    let ball = Ball {
        marker: BallMarker,
        sprite: SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(40., 40.)),
                ..Default::default()
            },
            ..Default::default()
        },
        velocity: Velocity(Vec2::new(30., 0.)),
    };
    commands.spawn().insert_bundle(ball);

    let paddle = Paddle {
        marker: PaddleMarker,
        sprite: SpriteBundle {
            transform: Transform {
                translation: Vec3::new(100., 100., 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(80., 20.)),
                ..Default::default()
            },
            ..Default::default()
        },
        velocity: Velocity::default(),
    };
    commands.spawn().insert_bundle(paddle);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(kinematics)
        .add_system(keyboard_input)
        .add_system(wall_bounce)
        .run();
}
