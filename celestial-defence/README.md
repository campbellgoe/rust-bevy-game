If you get invalid surface when trying to run your app, consider setting the WGPU_BACKEND environment variable and running your app again.

`export WGPU_BACKEND=VULKAN`

---

error log
error[E0428]: the name `Position` is defined multiple times
   --> src/main.rs:163:1
    |
11  | struct Position(Vec2);
    | ---------------------- previous definition of the type `Position` here
...
163 | struct Position(Vec2);
    | ^^^^^^^^^^^^^^^^^^^^^^ `Position` redefined here
    |
    = note: `Position` must be defined only once in the type namespace of this module

error[E0412]: cannot find type `Meteor` in this scope
  --> src/main.rs:71:34
   |
71 |     query: Query<&Position, With<Meteor>>,
   |                                  ^^^^^^ not found in this scope
   |
help: you might be missing a type parameter
   |
69 | fn adjust_time_scale<Meteor>(
   |                     ++++++++

error[E0425]: cannot find value `CRITICAL_DISTANCE` in this scope
  --> src/main.rs:83:36
   |
83 |     time.0 = if closest_distance < CRITICAL_DISTANCE {
   |                                    ^^^^^^^^^^^^^^^^^ not found in this scope

error[E0425]: cannot find value `TIME_STEP` in this scope
  --> src/main.rs:92:36
   |
92 |         position.0 += velocity.0 * TIME_STEP * time.0;
   |                                    ^^^^^^^^^ not found in this scope

error[E0425]: cannot find value `G` in this scope
   --> src/main.rs:109:30
    |
109 |                 let force = (G * body_i.mass * body_j.mass) / (distance * distance);
    |                              ^
    |
help: a local variable with a similar name exists
    |
109 |                 let force = (i * body_i.mass * body_j.mass) / (distance * distance);
    |                              ~
help: you might be missing a const parameter
    |
97  | fn apply_gravity<const G: /* Type */>(
    |                 +++++++++++++++++++++

error[E0425]: cannot find value `TIMESTEP` in this scope
   --> src/main.rs:115:35
    |
115 |         vel_i.0 += acceleration * TIMESTEP;
    |                                   ^^^^^^^^ not found in this scope

error[E0425]: cannot find value `TIMESTEP` in this scope
   --> src/main.rs:116:30
    |
116 |         pos_i.0 += vel_i.0 * TIMESTEP;
    |                              ^^^^^^^^ not found in this scope

error[E0425]: cannot find value `G` in this scope
   --> src/main.rs:128:28
    |
128 |             let gravity = -G * body.mass / projected_pos.length_squared();
    |                            ^ not found in this scope
    |
help: you might be missing a const parameter
    |
119 | fn draw_trajectories<const G: /* Type */>(
    |                     +++++++++++++++++++++

error[E0425]: cannot find value `TIMESTEP` in this scope
   --> src/main.rs:129:68
    |
129 |             projected_vel += projected_pos.normalize() * gravity * TIMESTEP;
    |                                                                    ^^^^^^^^ not found in this scope

error[E0425]: cannot find value `TIMESTEP` in this scope
   --> src/main.rs:130:46
    |
130 |             projected_pos += projected_vel * TIMESTEP;
    |                                              ^^^^^^^^ not found in this scope

error[E0425]: cannot find value `G` in this scope
   --> src/main.rs:146:28
    |
146 |             let gravity = -G * body.mass / projected_pos.length_squared();
    |                            ^ not found in this scope
    |
help: you might be missing a const parameter
    |
137 | fn draw_predicted_paths<const G: /* Type */>(
    |                        +++++++++++++++++++++

error[E0425]: cannot find value `TIMESTEP` in this scope
   --> src/main.rs:147:68
    |
147 |             projected_vel += projected_pos.normalize() * gravity * TIMESTEP;
    |                                                                    ^^^^^^^^ not found in this scope

error[E0425]: cannot find value `TIMESTEP` in this scope
   --> src/main.rs:148:46
    |
148 |             projected_pos += projected_vel * TIMESTEP;
    |                                              ^^^^^^^^ not found in this scope

error[E0412]: cannot find type `Input` in this scope
   --> src/main.rs:170:16
    |
170 |     input: Res<Input<KeyCode>>,
    |                ^^^^^ help: a struct with a similar name exists: `InMut`
    |
   ::: /home/george/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.15.2/src/system/input.rs:175:1
    |
175 | pub struct InMut<'a, T: ?Sized>(pub &'a mut T);
    | ------------------------------- similarly named struct `InMut` defined here

error[E0425]: cannot find value `ATMOSPHERE_RADIUS` in this scope
   --> src/main.rs:194:38
    |
194 |             if position.0.length() > ATMOSPHERE_RADIUS {
    |                                      ^^^^^^^^^^^^^^^^^ not found in this scope

error[E0425]: cannot find value `TIMESTEP` in this scope
   --> src/main.rs:201:42
    |
201 |             velocity.0 += thrust_force * TIMESTEP;
    |                                          ^^^^^^^^ not found in this scope

error[E0425]: cannot find value `TIMESTEP` in this scope
   --> src/main.rs:202:52
    |
202 |             rocket.fuel -= thrust_force.length() * TIMESTEP;
    |                                                    ^^^^^^^^ not found in this scope

error[E0425]: cannot find value `TIMESTEP` in this scope
   --> src/main.rs:205:36
    |
205 |         position.0 += velocity.0 * TIMESTEP;
    |                                    ^^^^^^^^ not found in this scope

error[E0412]: cannot find type `Meteor` in this scope
   --> src/main.rs:212:85
    |
212 |     mut meteors: Query<(Entity, &mut Velocity, &mut Position, &CelestialBody), With<Meteor>>,
    |                                                                                     ^^^^^^ not found in this scope
    |
help: you might be missing a type parameter
    |
209 | fn check_collisions<Meteor>(
    |                    ++++++++

error[E0425]: cannot find value `IMPACT_RADIUS` in this scope
   --> src/main.rs:218:27
    |
218 |             if distance < IMPACT_RADIUS {
    |                           ^^^^^^^^^^^^^ not found in this scope

error[E0425]: cannot find value `ROCKET_MASS` in this scope
   --> src/main.rs:220:49
    |
220 |                 meteor_vel.0 += rocket_vel.0 * (ROCKET_MASS / meteor_body.mass);
    |                                                 ^^^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `Input` in this scope
   --> src/main.rs:264:16
    |
264 |     input: Res<Input<KeyCode>>,
    |                ^^^^^ help: a struct with a similar name exists: `InMut`
    |
   ::: /home/george/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.15.2/src/system/input.rs:175:1
    |
175 | pub struct InMut<'a, T: ?Sized>(pub &'a mut T);
    | ------------------------------- similarly named struct `InMut` defined here

warning: use of deprecated struct `bevy::prelude::SpriteBundle`: Use the `Sprite` component instead. Inserting it will now also insert `Transform` and `Visibility` automatically.
  --> src/main.rs:27:9
   |
27 |         SpriteBundle {
   |         ^^^^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default

warning: use of deprecated struct `bevy::prelude::SpriteBundle`: Use the `Sprite` component instead. Inserting it will now also insert `Transform` and `Visibility` automatically.
  --> src/main.rs:41:9
   |
41 |         SpriteBundle {
   |         ^^^^^^^^^^^^

warning: use of deprecated struct `bevy::prelude::SpriteBundle`: Use the `Sprite` component instead. Inserting it will now also insert `Transform` and `Visibility` automatically.
  --> src/main.rs:55:9
   |
55 |         SpriteBundle {
   |         ^^^^^^^^^^^^

warning: use of deprecated struct `bevy::prelude::SpriteBundle`: Use the `Sprite` component instead. Inserting it will now also insert `Transform` and `Visibility` automatically.
   --> src/main.rs:241:9
    |
241 |         SpriteBundle {
    |         ^^^^^^^^^^^^

error[E0119]: conflicting implementations of trait `bevy::prelude::Component` for type `Position`
   --> src/main.rs:162:10
    |
10  | #[derive(Component)]
    |          --------- first implementation here
...
162 | #[derive(Component)]
    |          ^^^^^^^^^ conflicting implementation for `Position`
    |
    = note: this error originates in the derive macro `Component` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0560]: struct `bevy::prelude::SpriteBundle` has no field named `texture`
  --> src/main.rs:28:13
   |
28 |             texture: asset_server.load("textures/earth.png"), // Replace with a circle texture
   |             ^^^^^^^ `bevy::prelude::SpriteBundle` does not have this field
   |
   = note: available fields are: `sprite`, `global_transform`, `visibility`, `inherited_visibility`, `view_visibility`, `sync`

warning: use of deprecated field `bevy::prelude::SpriteBundle::transform`: Use the `Sprite` component instead. Inserting it will now also insert `Transform` and `Visibility` automatically.
  --> src/main.rs:29:13
   |
29 | /             transform: Transform {
30 | |                 scale: Vec3::splat(10.0), // Adjust size
31 | |                 ..Default::default()
32 | |             },
   | |_____________^

error[E0560]: struct `bevy::prelude::SpriteBundle` has no field named `texture`
  --> src/main.rs:42:13
   |
42 |             texture: asset_server.load("textures/moon.png"), // Replace with a smaller circle texture
   |             ^^^^^^^ `bevy::prelude::SpriteBundle` does not have this field
   |
   = note: available fields are: `sprite`, `global_transform`, `visibility`, `inherited_visibility`, `view_visibility`, `sync`

warning: use of deprecated field `bevy::prelude::SpriteBundle::transform`: Use the `Sprite` component instead. Inserting it will now also insert `Transform` and `Visibility` automatically.
  --> src/main.rs:43:13
   |
43 | /             transform: Transform {
44 | |                 scale: Vec3::splat(2.5),
45 | |                 ..Default::default()
46 | |             },
   | |_____________^

error[E0560]: struct `bevy::prelude::SpriteBundle` has no field named `texture`
  --> src/main.rs:56:13
   |
56 |             texture: asset_server.load("textures/asteroid.png"),
   |             ^^^^^^^ `bevy::prelude::SpriteBundle` does not have this field
   |
   = note: available fields are: `sprite`, `global_transform`, `visibility`, `inherited_visibility`, `view_visibility`, `sync`

warning: use of deprecated field `bevy::prelude::SpriteBundle::transform`: Use the `Sprite` component instead. Inserting it will now also insert `Transform` and `Visibility` automatically.
  --> src/main.rs:57:13
   |
57 | /             transform: Transform {
58 | |                 scale: Vec3::splat(1.3),
59 | |                 ..Default::default()
60 | |             },
   | |_____________^

error[E0599]: no variant or associated item named `GRAY` found for enum `bevy::prelude::Color` in the current scope
   --> src/main.rs:132:57
    |
132 |             gizmos.circle_2d(projected_pos, 1.0, Color::GRAY); 
    |                                                         ^^^^ variant or associated item not found in `Color`

error[E0599]: no variant or associated item named `GRAY` found for enum `bevy::prelude::Color` in the current scope
   --> src/main.rs:150:57
    |
150 |             gizmos.circle_2d(projected_pos, 1.0, Color::GRAY);
    |                                                         ^^^^ variant or associated item not found in `Color`

error[E0599]: no variant or associated item named `Up` found for enum `bevy::prelude::KeyCode` in the current scope
   --> src/main.rs:176:35
    |
176 |         if input.pressed(KeyCode::Up) {
    |                                   ^^ variant or associated item not found in `KeyCode`

error[E0599]: no variant or associated item named `Down` found for enum `bevy::prelude::KeyCode` in the current scope
   --> src/main.rs:179:35
    |
179 |         if input.pressed(KeyCode::Down) {
    |                                   ^^^^ variant or associated item not found in `KeyCode`

error[E0599]: no variant or associated item named `Left` found for enum `bevy::prelude::KeyCode` in the current scope
   --> src/main.rs:182:35
    |
182 |         if input.pressed(KeyCode::Left) {
    |                                   ^^^^ variant or associated item not found in `KeyCode`

error[E0599]: no variant or associated item named `Right` found for enum `bevy::prelude::KeyCode` in the current scope
   --> src/main.rs:185:35
    |
185 |         if input.pressed(KeyCode::Right) {
    |                                   ^^^^^ variant or associated item not found in `KeyCode`

error[E0560]: struct `bevy::prelude::SpriteBundle` has no field named `texture`
   --> src/main.rs:242:13
    |
242 |             texture: asset_server.load("textures/rocket.png"), // Rocket sprite
    |             ^^^^^^^ `bevy::prelude::SpriteBundle` does not have this field
    |
    = note: available fields are: `sprite`, `global_transform`, `visibility`, `inherited_visibility`, `view_visibility`, `sync`

warning: use of deprecated field `bevy::prelude::SpriteBundle::transform`: Use the `Sprite` component instead. Inserting it will now also insert `Transform` and `Visibility` automatically.
   --> src/main.rs:243:13
    |
243 | /             transform: Transform {
244 | |                 scale: Vec3::splat(5.0),
245 | |                 ..Default::default()
246 | |             },
    | |_____________^

error[E0308]: mismatched types
   --> src/main.rs:269:19
    |
269 |     apply_gravity(&mut meteor_query);
    |     ------------- ^^^^^^^^^^^^^^^^^ expected `Query<'_, '_, (&mut Velocity, ..., ...)>`, found `&mut Query<'_, '_, (&mut ..., ..., ...)>`
    |     |
    |     arguments to this function are incorrect
    |
    = note:         expected struct `bevy::prelude::Query<'_, '_, (&mut Velocity, &mut Position, &CelestialBody), _>`
            found mutable reference `&mut bevy::prelude::Query<'_, '_, (&mut Velocity, &mut Position, &CelestialBody), _>`
note: function defined here
   --> src/main.rs:97:4
    |
97  | fn apply_gravity(
    |    ^^^^^^^^^^^^^
98  |     mut query: Query<(&mut Velocity, &mut Position, &CelestialBody)>,
    |     ----------------------------------------------------------------
help: consider removing the borrow
    |
269 -     apply_gravity(&mut meteor_query);
269 +     apply_gravity(meteor_query);
    |

error[E0308]: mismatched types
   --> src/main.rs:271:21
    |
271 |     rocket_movement(&mut rocket_query, &input);
    |     --------------- ^^^^^^^^^^^^^^^^^ expected `Query<'_, '_, (&mut Velocity, ..., ...)>`, found `&mut Query<'_, '_, (&mut ..., ..., ...)>`
    |     |
    |     arguments to this function are incorrect
    |
    = note:         expected struct `bevy::prelude::Query<'_, '_, (&mut Velocity, &mut Position, &mut Rocket), _>`
            found mutable reference `&mut bevy::prelude::Query<'_, '_, (&mut Velocity, &mut Position, &mut Rocket), _>`
note: function defined here
   --> src/main.rs:168:4
    |
168 | fn rocket_movement(
    |    ^^^^^^^^^^^^^^^
169 |     mut query: Query<(&mut Velocity, &mut Position, &mut Rocket)>,
    |     -------------------------------------------------------------
help: consider removing the borrow
    |
271 -     rocket_movement(&mut rocket_query, &input);
271 +     rocket_movement(rocket_query, &input);
    |

error[E0308]: arguments to this function are incorrect
   --> src/main.rs:278:5
    |
278 |     draw_trajectories(&mut gizmos, &meteor_query);
    |     ^^^^^^^^^^^^^^^^^
    |
note: expected `Gizmos<'_, '_>`, found `&mut Gizmos<'_, '_>`
   --> src/main.rs:278:23
    |
278 |     draw_trajectories(&mut gizmos, &meteor_query);
    |                       ^^^^^^^^^^^
    = note:         expected struct `bevy::prelude::Gizmos<'_, '_, _, _>`
            found mutable reference `&mut bevy::prelude::Gizmos<'_, '_, _, _>`
note: expected `Query<'_, '_, (&Position, &Velocity, &...)>`, found `&Query<'_, '_, (&mut Velocity, ..., ...)>`
   --> src/main.rs:278:36
    |
278 |     draw_trajectories(&mut gizmos, &meteor_query);
    |                                    ^^^^^^^^^^^^^
    = note: expected struct `bevy::prelude::Query<'_, '_, (&Position, &Velocity, &CelestialBody), _>`
            found reference `&bevy::prelude::Query<'_, '_, (&mut Velocity, &mut Position, &CelestialBody), _>`
note: function defined here
   --> src/main.rs:119:4
    |
119 | fn draw_trajectories(
    |    ^^^^^^^^^^^^^^^^^
120 |     mut gizmos: Gizmos,
    |     ------------------
121 |     query: Query<(&Position, &Velocity, &CelestialBody)>,
    |     ----------------------------------------------------
help: consider removing the borrow
    |
278 -     draw_trajectories(&mut gizmos, &meteor_query);
278 +     draw_trajectories(gizmos, &meteor_query);
    |

error[E0308]: mismatched types
   --> src/main.rs:279:22
    |
279 |     update_positions(&mut transform_query);
    |     ---------------- ^^^^^^^^^^^^^^^^^^^^ expected `Query<'_, '_, (&mut Transform, &Position)>`, found `&mut Query<'_, '_, (&mut Transform, &...)>`
    |     |
    |     arguments to this function are incorrect
    |
    = note:         expected struct `bevy::prelude::Query<'_, '_, (&mut bevy::prelude::Transform, &Position), _>`
            found mutable reference `&mut bevy::prelude::Query<'_, '_, (&mut bevy::prelude::Transform, &Position), _>`
note: function defined here
   --> src/main.rs:252:4
    |
252 | fn update_positions(mut query: Query<(&mut Transform, &Position)>) {
    |    ^^^^^^^^^^^^^^^^ ---------------------------------------------
help: consider removing the borrow
    |
279 -     update_positions(&mut transform_query);
279 +     update_positions(transform_query);
    |

Some errors have detailed explanations: E0119, E0308, E0412, E0425, E0428, E0560, E0599.
For more information about an error, try `rustc --explain E0119`.
warning: `celestial-defense` (bin "celestial-defense") generated 8 warnings
error: could not compile `celestial-defense` (bin "celestial-defense") due to 37 previous errors; 8 warnings emitted