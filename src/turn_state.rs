#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}
