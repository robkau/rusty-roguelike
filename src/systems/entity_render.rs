use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub(crate) fn entity_render(ecs: &SubWorld<'_>, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    let _ = draw_batch.target(1);

    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            let _ = draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Batch error"); // todo z order from screen_width and screen_size instead. ?
}
