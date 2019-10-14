use Value;
use anf::Binding;
use typed_arena::Arena;

/// Collection of arenas that can be passed around easily.
pub struct Arenas<'a>
{
    /// The ANF builder uses vecs to allocate arrays of bindings. Once finished,
    /// the vecs are moved into this arena so that they can be sliced into by
    /// ANF expressions.
    pub bindings: &'a Arena<Vec<Binding<'a>>>,
    pub values:   &'a Arena<Value<'a>>,
}
