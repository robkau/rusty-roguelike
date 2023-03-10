use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub(crate) fn hud (ecs: &SubWorld<'_>) {
    let mut health_query = <&Health>::query().filter(component::<Player>());

    let player_health = health_query.iter(ecs).nth(0).unwrap();

    let mut draw_batch = DrawBatch::new();
    let _ = draw_batch.target(2);
    let _ = draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");

    let _ = draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH*2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED,BLACK)
    );

    let _ = draw_batch.print_color_centered(
        0,
        format!(" Health: {} / {}",
        player_health.current,
        player_health.max
        ),
        ColorPair::new(WHITE,RED)
    );

    draw_batch.submit(10000).expect("Batch error");
}