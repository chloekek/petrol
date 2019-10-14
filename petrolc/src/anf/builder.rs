use Arenas;
use Value;
use anf::*;

/// The ANF builder provides an EDSL for generating ANF bindings. The ANF builder
/// takes care of generating binding names and accumulating bindings, which you
/// can turn into an ANF expression when done.
pub struct AnfBuilder<'a>
{
    next_local: Local,
    bindings:   Vec<Binding<'a>>,
}

impl<'a> AnfBuilder<'a>
{
    /// Create a new ANF builder with no bindings.
    pub fn new() -> Self
    {
        Self{
            next_local: Local{id: 0},
            bindings:   Vec::new(),
        }
    }

    /// Return the constructed bindings along with a result simple. To simplify
    /// lifetime management, the bindings will be moved to the arena.
    pub fn finish(self, arenas: &'a Arenas<'a>, result: Simple<'a>) -> Anf<'a>
    {
        let bindings = arenas.bindings.alloc(self.bindings);
        Anf{bindings, result}
    }

    fn bind(&mut self, expression: Complex<'a>) -> Simple<'a>
    {
        let result = self.next_local;
        self.next_local.id += 1;

        let binding = Binding{result, expression};
        self.bindings.push(binding);

        Simple::Local(result)
    }

    /// Create a local simple. This is provided for consistency, but does not use
    /// self.
    pub fn local(&self, local: Local) -> Simple<'a>
    {
        Simple::Local(local)
    }

    /// Create a quote simple. This is provided for consistency, but does not use
    /// self.
    pub fn quote(&self, value: &'a Value<'a>) -> Simple<'a>
    {
        Simple::Quote(value)
    }

    /// Bind a CallRoutine expression and return its result.
    pub fn call_routine(
        &mut self,
        routine: Global<'a>,
        arguments: &'a [Simple<'a>],
    ) -> Simple<'a>
    {
        self.bind(Complex::CallRoutine{routine, arguments})
    }
}
