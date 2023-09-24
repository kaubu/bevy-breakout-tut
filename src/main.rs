// Disable Windows console on release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    prelude::*,
    math::{vec3, vec2},
    sprite::collide_aabb::{Collision, collide},
    audio::{VolumeLevel, Volume}
};
use bevy_embedded_assets::EmbeddedAssetPlugin;

// Paddle
const PADDLE_START_Y: f32 = BOTTOM_WALL + 60.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_COLOUR: Color = Color::rgb(0.3, 0.3, 0.7);
const PADDLE_SPEED: f32 = 500.0;

// Balls
const BALL_COLOUR: Color = Color::rgb(1.0, 0.5, 0.5);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const BALL_SPEED: f32 = 400.0;
const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

// Walls
const LEFT_WALL: f32 = -450.0;
const RIGHT_WALL: f32 = 450.0;
const BOTTOM_WALL: f32 = -300.0;
const TOP_WALL: f32 = 300.0;

const WALL_THICKNESS: f32 = 10.0;
const WALL_BLOCK_WIDTH: f32 = RIGHT_WALL - LEFT_WALL;
const WALL_BLOCK_HEIGHT: f32 = TOP_WALL - BOTTOM_WALL;
const WALL_COLOUR: Color = Color::rgb(0.8, 0.8, 0.8);

// Bricks
const BRICK_SIZE: Vec2 = Vec2::new(100.0, 30.0);
const BRICK_COLOUR: Color = Color::rgb(0.5, 0.5, 1.0);
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
const GAP_BETWEEN_BRICKS: f32 = 5.0;
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

// Scoreboard
const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOUR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOUR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball {
    size: Vec2,
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider {
    size: Vec2,
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

#[derive(Component)]
struct Brick;

#[derive(Resource, Clone, Copy)]
struct Scoreboard {
    score: usize,
}

#[derive(Resource, Default, Deref, DerefMut)]
struct CollisionSound(Handle<AudioSource>);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(
                    EmbeddedAssetPlugin
                )
        )
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(Scoreboard { score: 0 })
        .add_systems(Update, (
            bevy::window::close_on_esc,
            update_scoreboard,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                move_paddle,
                apply_velocity,
                check_ball_collisions.after(apply_velocity),
            )
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Sound
    let ball_collision_sound: Handle<AudioSource> = asset_server
        .load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound(ball_collision_sound));

    // Paddle
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec3(0.0, PADDLE_START_Y, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOUR,
                custom_size: Some(PADDLE_SIZE),
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider { size: PADDLE_SIZE },
    ));

    // Ball
    let ball_tex: Handle<Image> = asset_server
        .load("textures/circle.png");
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: BALL_STARTING_POSITION,
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOUR,
                custom_size: Some(BALL_SIZE),
                ..default()
            },
            texture: ball_tex,
            ..default()
        },
        Ball { size: BALL_SIZE },
        Velocity(BALL_SPEED * BALL_INITIAL_DIRECTION),
        // Velocity(Vec2::ZERO),
    ));

    // Walls
    {
        let vertical_wall_size = vec2(
            WALL_THICKNESS,
            WALL_BLOCK_HEIGHT + WALL_THICKNESS
        );
        let horizontal_wall_size = vec2(
            WALL_BLOCK_WIDTH + WALL_THICKNESS,
            WALL_THICKNESS
        );

        // Left wall
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(LEFT_WALL, 0.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOUR,
                    custom_size: Some(vertical_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: vertical_wall_size,
            }
        });

        // Right wall
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(RIGHT_WALL, 0.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOUR,
                    custom_size: Some(vertical_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: vertical_wall_size,
            }
        });

        // Bottom wall
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(0.0, BOTTOM_WALL, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOUR,
                    custom_size: Some(horizontal_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: horizontal_wall_size,
            }
        });

        // Top wall
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(0.0, TOP_WALL, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOUR,
                    custom_size: Some(horizontal_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: horizontal_wall_size,
            }
        });
    }

    // Bricks
    {
        let offset_x = LEFT_WALL
            + GAP_BETWEEN_BRICKS_AND_SIDES
            + BRICK_SIZE.x * 0.5;
        let offset_y = BOTTOM_WALL
            + GAP_BETWEEN_PADDLE_AND_BRICKS
            + BRICK_SIZE.y * 0.5;

        let bricks_total_width = (RIGHT_WALL - LEFT_WALL)
            - 2.0
            * GAP_BETWEEN_BRICKS_AND_SIDES;
        let bricks_total_height = (TOP_WALL - BOTTOM_WALL)
            - GAP_BETWEEN_BRICKS_AND_CEILING
            - GAP_BETWEEN_PADDLE_AND_BRICKS;

        let rows = (
            bricks_total_height / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)
        ).floor() as i32;
        let columns = (
            bricks_total_width / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)
        ).floor() as i32;

        for row in 0..rows {
            for column in 0..columns {
                let brick_pos = vec2(
                    offset_x + column as f32 * (
                        BRICK_SIZE.x + GAP_BETWEEN_BRICKS
                    ),
                    offset_y + row as f32 * (
                        BRICK_SIZE.y + GAP_BETWEEN_BRICKS
                    )
                );

                commands.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: brick_pos.extend(0.0),
                            ..default()
                        },
                        sprite: Sprite {
                            color: BRICK_COLOUR,
                            custom_size: Some(BRICK_SIZE),
                            ..default()
                        },
                        ..default()
                    },
                    Brick,
                    Collider { size: BRICK_SIZE },
                ));
            }
        }
    }

    // Scoreboard
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOUR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOUR,
                ..default()
            })
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
    ));
}

fn move_paddle(
    input: Res<Input<KeyCode>>,
    time_step: Res<FixedTime>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let mut paddle_transform = query.single_mut();

    let mut direction = 0.0;
    // Left
    if input.any_pressed([KeyCode::A, KeyCode::Left]) {
        direction -= 1.0;
    }
    // Right
    if input.any_pressed([KeyCode::D, KeyCode::Right]) {
        direction += 1.0;
    }

    let new_x = paddle_transform.translation.x
        + direction
        * PADDLE_SPEED
        * time_step.period.as_secs_f32();

    let new_x = new_x.min(
        RIGHT_WALL - (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5
    );
    let new_x = new_x.max(
        LEFT_WALL + (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5
    );

    paddle_transform.translation.x = new_x;
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>,
    time_step: Res<FixedTime>,
) {
    let dt = time_step.period.as_secs_f32();
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;
    }
}

fn check_ball_collisions(
    mut commands: Commands,
    mut score: ResMut<Scoreboard>,
    collision_sound: Res<CollisionSound>,
    mut ball_query: Query<(&mut Velocity, &Transform, &Ball)>,
    collider_query: Query<(Entity, &Transform, &Collider, Option<&Brick>)>,
) {
    for (
        mut ball_velocity,
        ball_transform,
        ball
    ) in &mut ball_query {
        for (
            other_entity,
            transform,
            other,
            opt_brick,
        ) in &collider_query {
            let collision = collide(
                ball_transform.translation,
                ball.size,
                transform.translation,
                other.size,
            );

            let mut reflect_x = false;
            let mut reflect_y = false;
            if let Some(collision) = collision {
                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.0,
                    Collision::Right => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                    Collision::Inside => { /* Do nothing */ },
                }

                if reflect_x {
                    ball_velocity.x *= -1.0;
                }
                if reflect_y {
                    ball_velocity.y *= -1.0;
                }

                if opt_brick.is_some() {
                    score.score += 1;
                    commands.entity(other_entity).despawn();
                }

                // Play sound
                commands.spawn(AudioBundle {
                    source: collision_sound.clone(),
                    settings: PlaybackSettings {
                        volume: Volume::Absolute(
                            VolumeLevel::new(0.1)
                        ),
                        ..PlaybackSettings::DESPAWN
                    },
                });
            }
        }
    }
}

fn update_scoreboard(score: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = score.score.to_string();
}
