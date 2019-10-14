/// Source position.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Position
{
    line:   u32,
    column: u32,
}
