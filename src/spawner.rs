use crate::prelude::*;

pub(crate) fn spawn_player(ecs: &mut World, pos: Point) {
    let _  = ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}
