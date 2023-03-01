//! Documentation comment for rusty-roguelike game!

//clippy lints
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::restriction,
    clippy::nursery,
    clippy::cargo
)]
// lint groups
#![deny(
    future_incompatible,
    let_underscore,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility
)]
// lints not in groups
#![deny(
    macro_use_extern_crate,
    missing_abi,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_lifetimes,
    unused_results,
    variant_size_differences
)]
// warning lint groups
#![warn(warnings, unused)]

use bracket_lib::prelude::*;

mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;

mod prelude {
    pub(crate) use bracket_lib::prelude::*;
    pub(crate) use legion::systems::CommandBuffer;
    pub(crate) use legion::world::SubWorld;
    pub(crate) use legion::*;
    pub(crate) const SCREEN_WIDTH: i32 = 80;
    pub(crate) const SCREEN_HEIGHT: i32 = 50;
    pub(crate) const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub(crate) const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub(crate) use crate::camera::*;
    pub(crate) use crate::components::*;
    pub(crate) use crate::map::*;
    pub(crate) use crate::map_builder::*;
    pub(crate) use crate::spawner::*;
    pub(crate) use crate::systems::*;
}

use prelude::*;


fn main() -> BError {
    let ctx = BTermBuilder::new()
        .with_title("Rusty Roguelike")
        .with_fps_cap(30.)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(ctx, State::new())
}

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
    }
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        Self {
            ecs,
            resources,
            systems: build_scheduler(),
        }
    }
}
