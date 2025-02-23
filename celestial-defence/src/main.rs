use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use std::collections::HashMap;

const TIMESTEP: f32 = 0.016; // ~60 FPS update step
const G: f32 = 1.0; // Scaled gravitational constant for gameplay

const CRITICAL_DISTANCE: f32 = 100.0; // (Placeholder for potential slow-motion trigger)

// These constants are defined for future expansion (e.g. rocket physics)
const ATMOSPHERE_RADIUS: f32 = 1000.0;
const IMPACT_RADIUS: f32 = 10.0;
const ROCKET_MASS: f32 = 1e6;

#[derive(Component)]
struct CelestialBody {
    mass: f32,
    velocity: Vec2,
}

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

// Marker components for abstraction
#[derive(Component)]
struct Earth;

#[derive(Component)]
struct Moon;

#[derive(Component)]
struct Meteor;

#[derive(Resource)]
struct TimeScale(f32);

#[derive(Component)]
struct Rocket {
    fuel: f32,
    thrust: f32,
    in_space: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: WgpuSettings {
                backends: Some(Backends::GL),
                ..default()
            }
            .into(),
            ..default()
        }),)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(TimeScale(1.0))
        .add_systems(Startup, setup)
        .add_systems(Update, physics_update)
        .add_systems(Update, update_transforms)
        .add_systems(Update, draw_trajectories)
        .add_systems(Update, rocket_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Earth: Fixed at the center
    commands.spawn((
        Earth,
        CelestialBody { mass: 5.97e24, velocity: Vec2::ZERO },
        Position(Vec2::ZERO),
        Sprite {
            image: asset_server.load("textures/earth.png"), // Use a circle texture
            color: Color::WHITE, // Default color (you can change it if needed)
            ..Default::default()
        },
        Transform {
            scale: Vec3::splat(10.0), // Adjust size as needed
            ..Default::default()
        },
    ));

    // Moon: Orbiting Earth
    commands.spawn((
        Moon,
        CelestialBody { mass: 7.35e22, velocity: Vec2::new(0.0, 1022.0) },
        Position(Vec2::new(384_400.0, 0.0)), // Approximate Moon distance (scaled)
        Sprite {
            image: asset_server.load("textures/moon.png"), // Use a smaller circle texture
            color: Color::WHITE,
            ..Default::default()
        },
        Transform {
            scale: Vec3::splat(2.5),
            ..Default::default()
        },
    ));

    // Spawn one meteor (or asteroid) to test the simulation
    commands.spawn((
        Meteor,
        CelestialBody { mass: 1.0e10, velocity: Vec2::new(-500.0, 0.0) },
        Position(Vec2::new(600_000.0, 300_000.0)), // Starting position (scaled)
        Sprite {
            image: asset_server.load("textures/asteroid.png"),
            color: Color::WHITE,
            ..Default::default()
        },
        Transform {
            scale: Vec3::splat(1.3),
            ..Default::default()
        },
    ));
}

/// Physics update system that applies gravitational interactions between all celestial objects.
fn physics_update(
    mut query: Query<(Entity, &mut CelestialBody, &mut Position)>,
) {
    let bodies: Vec<(Entity, Vec2, f32)> = query
        .iter_mut()
        .map(|(e, body, pos)| (e, pos.0, body.mass))
        .collect();

    let mut accelerations: HashMap<Entity, Vec2> = HashMap::new();

    for (e1, pos1, mass1) in &bodies {
        let mut total_acc = Vec2::ZERO;
        for (e2, pos2, mass2) in &bodies {
            if e1 == e2 {
                continue;
            }
            let direction = *pos2 - *pos1;
            let distance = direction.length().max(1.0);
            let force = (G * mass1 * mass2) / (distance * distance);
            total_acc += direction.normalize() * (force / mass1);
        }
        accelerations.insert(*e1, total_acc);
    }

    for (e, mut body, mut pos) in query.iter_mut() {
        if let Some(acc) = accelerations.get(&e) {
            body.velocity += *acc * TIMESTEP;
            pos.0 += body.velocity * TIMESTEP;
        }
    }
}

/// Updates the Bevy transform for each entity based on its Position component.
fn update_transforms(
    mut query: Query<(&mut Transform, &Position)>,
) {
    for (mut transform, pos) in query.iter_mut() {
        transform.translation = Vec3::new(pos.0.x, pos.0.y, 0.0);
    }
}

fn draw_trajectories(
    mut gizmos: Gizmos,
    query: Query<(&Position, &Velocity, &CelestialBody)>,
) {
    for (pos, vel, body) in query.iter() {
        let mut projected_pos = pos.0;
        let mut projected_vel = vel.0;

        for _ in 0..100 {
            let gravity = -G * body.mass / projected_pos.length_squared();
            projected_vel += projected_pos.normalize() * gravity * TIMESTEP;
            projected_pos += projected_vel * TIMESTEP;

            gizmos.circle_2d(projected_pos, 1.0, Color::srgb(0.5, 0.5, 0.5)); // GRAY replacement
        }
    }
}

fn rocket_movement(
    mut query: Query<(&mut Velocity, &mut Position, &mut Rocket)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (mut velocity, mut position, mut rocket) in query.iter_mut() {
        let mut thrust_force = Vec2::ZERO;

        if input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            thrust_force.y += rocket.thrust;
        }
        if input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            thrust_force.y -= rocket.thrust;
        }
        if input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            thrust_force.x -= rocket.thrust;
        }
        if input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            thrust_force.x += rocket.thrust;
        }

        if !rocket.in_space {
            let air_resistance = 0.98;
            velocity.0 *= air_resistance;

            if position.0.length() > ATMOSPHERE_RADIUS {
                rocket.in_space = true;
            }
        }

        if rocket.fuel > 0.0 {
            velocity.0 += thrust_force * TIMESTEP;
            rocket.fuel -= thrust_force.length() * TIMESTEP;
        }

        position.0 += velocity.0 * TIMESTEP;
    }
}