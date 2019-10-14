use Value;

////////////////////////////////////////////////////////////////////////////////

pub trait Parser<'a>
{
    type Item: 'a;
    fn parse(&self, value: &Value<'a>) -> Option<Self::Item>;
}

////////////////////////////////////////////////////////////////////////////////

pub fn atom<'a>() -> impl Parser<'a, Item=&'a [u8]>
{
    Atom
}

struct Atom;

impl<'a> Parser<'a> for Atom
{
    type Item = &'a [u8];
    fn parse(&self, value: &Value<'a>) -> Option<Self::Item>
    {
        value.atom_name()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn cons<'a>() -> impl Parser<'a, Item=(&'a Value<'a>, &'a Value<'a>)>
{
    Cons
}

struct Cons;

impl<'a> Parser<'a> for Cons
{
    type Item = (&'a Value<'a>, &'a Value<'a>);
    fn parse(&self, value: &Value<'a>) -> Option<Self::Item>
    {
        value.list_uncons()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn nil<'a>() -> impl Parser<'a, Item=()>
{
    Nil
}

struct Nil;

impl<'a> Parser<'a> for Nil
{
    type Item = ();
    fn parse(&self, value: &Value<'a>) -> Option<Self::Item>
    {
        if value.is_nil() {
            Some(())
        } else {
            None
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn list<'a>() -> impl Parser<'a, Item=Vec<&'a Value<'a>>>
{
    List
}

struct List;

impl<'a> Parser<'a> for List
{
    type Item = Vec<&'a Value<'a>>;
    fn parse(&self, mut value: &Value<'a>) -> Option<Self::Item>
    {
        let mut result = Vec::new();

        loop {
            if let Some((head, tail)) = value.list_uncons() {
                result.push(head);
                value = tail;
                continue;
            }

            if value.is_nil() {
                break;
            }

            return None;
        }

        Some(result)
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn triplet<'a>()
    -> impl Parser<'a, Item=(&'a Value<'a>, &'a Value<'a>, &'a Value<'a>)>
{
    Triplet
}

struct Triplet;

impl<'a> Parser<'a> for Triplet
{
    type Item = (&'a Value<'a>, &'a Value<'a>, &'a Value<'a>);
    fn parse(&self, mut value: &Value<'a>) -> Option<Self::Item>
    {
        macro_rules! elem
        {
            () => {
                if let Some((head, tail)) = value.list_uncons() {
                    value = tail;
                    head
                } else {
                    return None;
                }
            }
        }

        let elem0 = elem!();
        let elem1 = elem!();
        let elem2 = elem!();

        if !value.is_nil() {
            return None;
        }

        Some((elem0, elem1, elem2))
    }
}
