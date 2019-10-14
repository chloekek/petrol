use Hash;
use Position;
use std::cell::Cell;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::Hasher;

/// In the compiler, values contain all their usual information, but also their
/// position in the source code. This allows source position to be included in
/// diagnostics and debug information.
#[derive(Clone)]
pub struct Value<'a>
{
    pub position: Option<Position>,
    // These fields are all private so that they cannot be updated in such a way
    // that the hash cache is compromised.
    tag:      [u8; 4],
    pointers: &'a [Value<'a>],
    bytes:    &'a [u8],
    hash:     Cell<Option<Hash>>,
}

impl<'a> Value<'a>
{
    /// Create a new value using the given information.
    pub fn new(
        position: Option<Position>,
        tag:      [u8; 4],
        pointers: &'a [Value<'a>],
        bytes:    &'a [u8],
    ) -> Self
    {
        let hash = Cell::new(None);
        Self{position, tag, pointers, bytes, hash}
    }

    /// Return the tag of the value.
    pub fn tag(&self) -> [u8; 4] { self.tag }

    /// Return the pointers of the value.
    pub fn pointers(&self) -> &'a [Value<'a>] { self.pointers }

    /// Return the bytes of the value.
    pub fn bytes(&self) -> &'a [u8] { self.bytes }

    /// Return the hash that uniquely identifies the value. Ignores the
    /// source position.
    pub fn hash(&self) -> Hash
    {
        match self.hash.get() {
            Some(hash) =>
                hash,
            None => {
                let hash = self.compute_hash();
                self.hash.set(Some(hash));
                hash
            },
        }
    }

    fn compute_hash(&self) -> Hash
    {
        // TODO: Switch to using SHA-256.

        let mut h = DefaultHasher::new();

        h.write(&self.tag);
        h.write_u32(self.pointers.len() as u32);
        h.write_u64(self.bytes.len() as u64);
        for p in self.pointers { h.write(&p.hash().0); }
        h.write(&self.bytes);

        let h = h.finish().to_be_bytes();
        Hash([h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7],
              h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7],
              h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7],
              h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]])
    }
}

impl<'a> fmt::Debug for Value<'a>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Value::new({:?}, {:?}, {:?}, {:?})",
               self.position, self.tag, self.pointers, self.bytes)
    }
}

/// List operations.
impl<'a> Value<'a>
{
    /// Return whether the value is a list.
    pub fn is_list(&self) -> bool
    {
        self.is_cons() || self.is_nil()
    }

    /// Return whether the value is a cons.
    pub fn is_cons(&self) -> bool
    {
        &self.tag == b"cons" && self.pointers.len() >= 2
    }

    /// Return whether the value is a nil.
    pub fn is_nil(&self) -> bool
    {
        &self.tag == b"nil "
    }

    /// If the value is a cons, return the head and tail of the list. If the
    /// value is not a cons, return None.
    pub fn list_uncons(&self) -> Option<(&'a Value<'a>, &'a Value<'a>)>
    {
        if self.is_cons() {
            Some((&self.pointers[0], &self.pointers[1]))
        } else {
            None
        }
    }
}

/// Atom operations.
impl<'a> Value<'a>
{
    /// Return whether the value is an atom.
    pub fn is_atom(&self) -> bool
    {
        &self.tag == b"atom"
    }

    /// If the value is an atom, return its name. If the value is not an atom,
    /// return None.
    pub fn atom_name(&self) -> Option<&'a [u8]>
    {
        if self.is_atom() {
            Some(&self.bytes)
        } else {
            None
        }
    }
}
