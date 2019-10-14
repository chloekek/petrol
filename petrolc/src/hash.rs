use std::fmt;

/// 256-bit hash.
#[derive(Clone, Copy)]
pub struct Hash(pub [u8; 32]);

impl fmt::Display for Hash
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        for b in self.0.iter() {
            write!(f, "{:02X}", b)?;
        }
        Ok(())
    }
}
