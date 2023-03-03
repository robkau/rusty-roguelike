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
