use Value;

pub mod builder;

#[derive(Clone, Copy, Debug)]
pub enum Item<'a>
{
    Routine{
        name:       Global<'a>,
        parameters: &'a [Local],
        body:       Anf<'a>,
    },
}

#[derive(Clone, Copy, Debug)]
pub struct Anf<'a>
{
    pub bindings: &'a [Binding<'a>],
    pub result:   Simple<'a>,
}

#[derive(Clone, Copy, Debug)]
pub struct Binding<'a>
{
    pub result:     Local,
    pub expression: Complex<'a>,
}

#[derive(Clone, Copy, Debug)]
pub enum Complex<'a>
{
    CallRoutine{
        routine:   Global<'a>,
        arguments: &'a [Simple<'a>],
    },
}

#[derive(Clone, Copy, Debug)]
pub enum Simple<'a>
{
    Local(Local),
    Quote(&'a Value<'a>),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Global<'a>
{
    pub name: &'a [u8],
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Local
{
    pub id: u64,
}
