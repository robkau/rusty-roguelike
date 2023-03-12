pub(crate) use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Render {
    pub(crate) color: ColorPair,
    pub(crate) glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct WantsToMove {
    pub(crate) entity: Entity,
    pub(crate) destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct WantsToAttack {
    pub(crate) attacker: Entity,
    pub(crate) victim: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Health {
    pub(crate) current: i32,
    pub(crate) max: i32,
}

#[derive(Clone, PartialEq)]
pub(crate) struct Name (pub(crate) String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ChasingPlayer;