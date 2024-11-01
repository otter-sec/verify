/// Size of a hash in bytes.
pub const HASH_BYTES: usize = 32;
/// Maximum string length of a base58 encoded hash.
const MAX_BASE58_LEN: usize = 44;

#[derive(Debug, Default)]
pub struct Hash(pub(crate) [u8; HASH_BYTES]);

#[cfg(any(kani, feature = "kani"))]
impl<'info> kani::Arbitrary for Hash {
    fn any() -> Self {
        Self(kani::any())
    }
}


pub fn hash(_val: &[u8]) -> Hash {
    // Return dummy hash if not running under kani
    // Else return a random hash
    #[cfg(not(feature = "kani"))]
    {
        Hash::default()
    }
    #[cfg(feature = "kani")]
    {
        kani::any()
    }
}

impl From<[u8; HASH_BYTES]> for Hash {
    fn from(from: [u8; 32]) -> Self {
        Self(from)
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl Hash {
    pub fn new(hash_slice: &[u8]) -> Self {
        Hash(<[u8; HASH_BYTES]>::try_from(hash_slice).unwrap())
    }

    pub const fn new_from_array(hash_array: [u8; HASH_BYTES]) -> Self {
        Self(hash_array)
    }

    pub fn to_bytes(self) -> [u8; HASH_BYTES] {
        self.0
    }
}

