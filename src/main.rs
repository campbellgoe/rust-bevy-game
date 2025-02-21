use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_fpc::*;
use noise::{NoiseFn, Simplex};
use rand::Rng;

const CHUNK_SIZE: f32 = 32.0;
const CHUNK_RESOLUTION: usize = 32;
const CHUNK_GRID_SIZE: i32 = 3;

#[derive(Component)]
struct Chunk {
    x: i32,
    z: i32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(FpcPlugin) // Add the FPC plugin
        .add_systems(Startup, setup)
        .add_systems(Update, update_chunks)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Spawn FPC (First Person Controller)
    commands.spawn((
        FpcBundle {
            spatial: SpatialBundle::from_transform(
                Transform::from_xyz(0.0, 10.0, 0.0)
            ),
            ..default()
        },
        Player,
    ));

    // Initial chunks
    for x in -1..=1 {
        for z in -1..=1 {
            spawn_chunk(&mut commands, &mut meshes, &mut materials, x, z);
        }
    }
}

fn spawn_chunk(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    chunk_x: i32,
    chunk_z: i32,
) {
    let noise = Simplex::new(rand::thread_rng().gen());
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);

    let mut vertices: Vec<Vec3> = Vec::new();
    let mut normals: Vec<Vec3> = Vec::new();
    let mut uvs: Vec<Vec2> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for x in 0..=CHUNK_RESOLUTION {
        for z in 0..=CHUNK_RESOLUTION {
            let x_pos = x as f32 / CHUNK_RESOLUTION as f32 * CHUNK_SIZE;
            let z_pos = z as f32 / CHUNK_RESOLUTION as f32 * CHUNK_SIZE;
            let world_x = chunk_x as f32 * CHUNK_SIZE + x_pos;
            let world_z = chunk_z as f32 * CHUNK_SIZE + z_pos;

            let height = noise.get([world_x as f64 * 0.1, world_z as f64 * 0.1]) as f32 * 5.0;

            vertices.push(Vec3::new(x_pos, height, z_pos));
            
            // Calculate normal based on neighboring vertices
            let normal = if x > 0 && z > 0 {
                let dx = vertices[vertices.len() - 1].y - vertices[vertices.len() - 2].y;
                let dz = vertices[vertices.len() - CHUNK_RESOLUTION - 2].y - vertices[vertices.len() - 1].y;
                Vec3::new(-dx, 1.0, -dz).normalize()
            } else {
                Vec3::Y
            };
            normals.push(normal);
            
            uvs.push(Vec2::new(x as f32 / CHUNK_RESOLUTION as f32, z as f32 / CHUNK_RESOLUTION as f32));

            if x < CHUNK_RESOLUTION && z < CHUNK_RESOLUTION {
                let top_left = (z * (CHUNK_RESOLUTION + 1) + x) as u32;
                let top_right = top_left + 1;
                let bottom_left = ((z + 1) * (CHUNK_RESOLUTION + 1) + x) as u32;
                let bottom_right = bottom_left + 1;

                indices.extend_from_slice(&[top_left, bottom_left, top_right]);
                indices.extend_from_slice(&[top_right, bottom_left, bottom_right]);
            }
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices.clone())));

    let collision_indices: Vec<[u32; 3]> = indices.chunks(3).map(|chunk| [chunk[0], chunk[1], chunk[2]]).collect();

    let chunk_position = Vec3::new(chunk_x as f32 * CHUNK_SIZE, 0.0, chunk_z as f32 * CHUNK_SIZE);

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_translation(chunk_position),
            ..default()
        },
        Chunk { x: chunk_x, z: chunk_z },
        Collider::trimesh(vertices, collision_indices),
    ));
}

fn update_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<&Transform, With<Player>>,
    chunk_query: Query<(Entity, &Chunk)>,
) {
    let player_transform = player_query.single();
    let player_chunk_x = (player_transform.translation.x / CHUNK_SIZE).floor() as i32;
    let player_chunk_z = (player_transform.translation.z / CHUNK_SIZE).floor() as i32;

    let mut existing_chunks = Vec::new();
    for (entity, chunk) in chunk_query.iter() {
        if (chunk.x - player_chunk_x).abs() > CHUNK_GRID_SIZE / 2
            || (chunk.z - player_chunk_z).abs() > CHUNK_GRID_SIZE / 2
        {
            commands.entity(entity).despawn();
        } else {
            existing_chunks.push((chunk.x, chunk.z));
        }
    }

    for x in (player_chunk_x - CHUNK_GRID_SIZE / 2)..=(player_chunk_x + CHUNK_GRID_SIZE / 2) {
        for z in (player_chunk_z - CHUNK_GRID_SIZE / 2)..=(player_chunk_z + CHUNK_GRID_SIZE / 2) {
            if !existing_chunks.contains(&(x, z)) {
                spawn_chunk(&mut commands, &mut meshes, &mut materials, x, z);
            }
        }
    }
}

