use std::str::FromStr;
use std::ops::Rem;
use std::fmt::Display;

pub type Hashable = Vec<u8>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashKey {
    Bits32(u32),
    Bits64(u64),
}

impl Default for HashKey {
    fn default() -> Self {
        return HashKey::from(0 as u32);
    }
}

impl Display for HashKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashKey::Bits32(x) => write!(f, "{}", x),
            HashKey::Bits64(x) => write!(f, "{}", x),
        }
    }
}

impl From<u32> for HashKey {
    fn from(value: u32) -> Self {
        return HashKey::Bits32(value);
    }
}

impl From<u64> for HashKey {
    fn from(value: u64) -> Self {
        return HashKey::Bits64(value);
    }
}

impl Into<u32> for HashKey {
    fn into(self) -> u32 {
        match self {
            HashKey::Bits32(x) => x,
            HashKey::Bits64(x) => x as u32,
        }
    }
}

impl Into<u64> for HashKey {
    fn into(self) -> u64 {
        match self {
            HashKey::Bits32(x) => x as u64,
            HashKey::Bits64(x) => x,
        }
    }
}

impl Rem<u32> for HashKey {
    type Output = u32;

    fn rem(self, rhs: u32) -> Self::Output {
        match self {
            HashKey::Bits32(x) => x % rhs,
            HashKey::Bits64(x) => x as u32 % rhs,
        }
    }
}

impl Rem<u64> for HashKey {
    type Output = u64;

    fn rem(self, rhs: u64) -> Self::Output {
        match self {
            HashKey::Bits32(x) => x as u64 % rhs,
            HashKey::Bits64(x) => x % rhs,
        }
    }
}

impl HashKey {
    pub fn as_u32(self) -> u32 {
        return self.into();
    }

    pub fn as_u64(self) -> u64 {
        return self.into();
    }
}

// First-order hash functions

#[derive(Debug, Clone)]
pub struct FNV1A {
    name: String,
}

impl Default for FNV1A {
    fn default() -> Self {
        return Self {
            name: "fnv1a".to_string(),
        };
    }
}

impl FNV1A {
    fn hash(h: &Hashable) -> HashKey {
        let mut result = 0x811c9dc5 as u32;

        for d in h {
            result ^= *d as u32;
            result = result.wrapping_mul(0x01000193 as u32);
        }

        return HashKey::from(result);
    }
}

#[derive(Debug, Clone)]
pub struct XXHash32 {
    name: String,
}

impl Default for XXHash32 {
    fn default() -> Self {
        return Self {
            name: "xxhash32".to_string(),
        };
    }
}

impl XXHash32 {
    fn hash(h: &Hashable) -> HashKey {
        const PRIME1: u32 = 0x9E3779B1;
        const PRIME2: u32 = 0x85EBCA6B;
        const PRIME3: u32 = 0xC2B2AE35;
        const PRIME4: u32 = 0x27D4EB2F;
        const PRIME5: u32 = 0x165667B1;

        let mut res = PRIME5.wrapping_add(h.len() as u32);

        let chunks = h.chunks_exact(4);
        let remainder = chunks.remainder();

        for chunk in chunks {
            let k = u32::from_le_bytes(chunk.try_into().unwrap());
            res = res.wrapping_add(k.wrapping_mul(PRIME1));
            res = res.rotate_left(13).wrapping_mul(PRIME2);
        }

        let mut rem = 0u32;
        for &b in remainder {
            rem = (rem << 8) | b as u32;
        }
        if !remainder.is_empty() {
            res = res.wrapping_add(rem.wrapping_mul(PRIME3));
            res = res.rotate_left(13).wrapping_mul(PRIME2);
        }

        res ^= res >> 16;
        res = res.wrapping_mul(PRIME4);
        res ^= res >> 13;
        res = res.wrapping_mul(PRIME1);
        res ^= res >> 16;

        return HashKey::from(res);
    }
}

#[derive(Debug, Clone)]
pub struct Murmur3 {
    name: String,
}

impl Default for Murmur3 {
    fn default() -> Self {
        return Self {
            name: "murmur3".to_string(),
        };
    }
}

impl Murmur3 {
    fn hash(h: &Hashable) -> HashKey {
        const SEED: u32 = 0x8286ff1d;
        const C1: u32 = 0xcc9e2d51;
        const C2: u32 = 0x1b873593;
        const C3: u32 = 0xe6546b64;
        const C4: u32 = 0x85ebca6b;
        const C5: u32 = 0xc2b2ae35;

        let data = h;
        let len = data.len();
        let mut hash = SEED;
        let mut i = 0;

        // Process 4-byte chunks
        while i + 4 <= len {
            let mut k = u32::from_le_bytes(data[i..i + 4].try_into().unwrap());
            k = k.wrapping_mul(C1);
            k = k.rotate_left(15);
            k = k.wrapping_mul(C2);
            hash ^= k;
            hash = hash.rotate_left(13);
            hash = hash.wrapping_mul(5).wrapping_add(C3);
            i += 4;
        }

        // Handle remaining bytes
        let mut k: u32 = 0;
        let remaining = len - i;
        if remaining >= 3 {
            k ^= (data[i + 2] as u32) << 16;
        }
        if remaining >= 2 {
            k ^= (data[i + 1] as u32) << 8;
        }
        if remaining >= 1 {
            k ^= data[i] as u32;
            k = k.wrapping_mul(C1);
            k = k.rotate_left(15);
            k = k.wrapping_mul(C2);
            hash ^= k;
        }

        // Finalization
        hash ^= len as u32;
        hash ^= hash >> 16;
        hash = hash.wrapping_mul(C4);
        hash ^= hash >> 13;
        hash = hash.wrapping_mul(C5);
        hash ^= hash >> 16;

        return HashKey::from(hash);
    }
}

#[derive(Debug, Clone)]
pub struct XXHash64 {
    name: String,
}

impl Default for XXHash64 {
    fn default() -> Self {
        return Self {
            name: "xxhash64".to_string(),
        };
    }
}

impl XXHash64 {
    fn hash(h: &Hashable) -> HashKey {
        const PRIME1: u64 = 0x9e3779b185ebca87;
        const PRIME2: u64 = 0xc2b2ae3d27d4eb4f;
        const PRIME3: u64 = 0x165667b19e3779f9;
        const PRIME4: u64 = 0x85ebca77c2b2ae63;
        const PRIME5: u64 = 0x27d4eb2f165667c5;

        let data = h;
        let len = data.len();
        let mut hash: u64;
        let mut i = 0;

        if len >= 32 {
            let mut v1 = PRIME1.wrapping_add(PRIME2);
            let mut v2 = PRIME2;
            let mut v3 = 0u64;
            let mut v4 = PRIME1.wrapping_neg();

            while i + 32 <= len {
                v1 = v1.wrapping_add(
                    u64::from_le_bytes(data[i..i + 8].try_into().unwrap()).wrapping_mul(PRIME2),
                );
                v1 = v1.rotate_left(31).wrapping_mul(PRIME1);

                v2 = v2.wrapping_add(
                    u64::from_le_bytes(data[i + 8..i + 16].try_into().unwrap())
                        .wrapping_mul(PRIME2),
                );
                v2 = v2.rotate_left(31).wrapping_mul(PRIME1);

                v3 = v3.wrapping_add(
                    u64::from_le_bytes(data[i + 16..i + 24].try_into().unwrap())
                        .wrapping_mul(PRIME2),
                );
                v3 = v3.rotate_left(31).wrapping_mul(PRIME1);

                v4 = v4.wrapping_add(
                    u64::from_le_bytes(data[i + 24..i + 32].try_into().unwrap())
                        .wrapping_mul(PRIME2),
                );
                v4 = v4.rotate_left(31).wrapping_mul(PRIME1);

                i += 32;
            }

            hash = v1
                .rotate_left(1)
                .wrapping_add(v2.rotate_left(7))
                .wrapping_add(v3.rotate_left(12))
                .wrapping_add(v4.rotate_left(18));

            v1 = v1.wrapping_mul(PRIME2);
            v1 = v1.rotate_left(31).wrapping_mul(PRIME1);
            hash ^= v1;
            hash = hash.wrapping_mul(PRIME1).wrapping_add(PRIME4);

            v2 = v2.wrapping_mul(PRIME2);
            v2 = v2.rotate_left(31).wrapping_mul(PRIME1);
            hash ^= v2;
            hash = hash.wrapping_mul(PRIME1).wrapping_add(PRIME4);

            v3 = v3.wrapping_mul(PRIME2);
            v3 = v3.rotate_left(31).wrapping_mul(PRIME1);
            hash ^= v3;
            hash = hash.wrapping_mul(PRIME1).wrapping_add(PRIME4);

            v4 = v4.wrapping_mul(PRIME2);
            v4 = v4.rotate_left(31).wrapping_mul(PRIME1);
            hash ^= v4;
            hash = hash.wrapping_mul(PRIME1).wrapping_add(PRIME4);
        } else {
            hash = PRIME5;
        }

        hash = hash.wrapping_add(len as u64);

        while i + 8 <= len {
            let k = u64::from_le_bytes(data[i..i + 8].try_into().unwrap());
            let mut k_val = k.wrapping_mul(PRIME2);
            k_val = k_val.rotate_left(31).wrapping_mul(PRIME1);
            hash ^= k_val;
            hash = hash
                .rotate_left(27)
                .wrapping_mul(PRIME1)
                .wrapping_add(PRIME4);
            i += 8;
        }

        while i + 4 <= len {
            let k = u32::from_le_bytes(data[i..i + 4].try_into().unwrap()) as u64;
            hash ^= k.wrapping_mul(PRIME1);
            hash = hash
                .rotate_left(23)
                .wrapping_mul(PRIME2)
                .wrapping_add(PRIME3);
            i += 4;
        }

        while i < len {
            let k = data[i] as u64;
            hash ^= k.wrapping_mul(PRIME5);
            hash = hash.rotate_left(11).wrapping_mul(PRIME1);
            i += 1;
        }

        hash ^= hash >> 33;
        hash = hash.wrapping_mul(PRIME2);
        hash ^= hash >> 29;
        hash = hash.wrapping_mul(PRIME3);
        hash ^= hash >> 32;

        return HashKey::from(hash);
    }
}

#[derive(Debug, Clone)]
pub enum FOHash {
    FNV1A(FNV1A),
    XXHash32(XXHash32),
    Murmur3(Murmur3),
    XXHash64(XXHash64),
}

impl Default for FOHash {
    fn default() -> Self {
        return FOHash::Murmur3(Murmur3::default());
    }
}

impl FromStr for FOHash {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fnv1a" => Ok(FOHash::FNV1A(FNV1A::default())),
            "xxhash32" => Ok(FOHash::XXHash32(XXHash32::default())),
            "murmur3" => Ok(FOHash::Murmur3(Murmur3::default())),
            "xxhash64" => Ok(FOHash::XXHash64(XXHash64::default())),
            _ => Err("Cannot find a corresponding first-order hash. Expected: fnv1a, xxhash32, murmur3, xxhash64".into()),
        }
    }
}

impl FOHash {
    pub fn hash(&self, h: &Hashable) -> HashKey {
        match self {
            FOHash::FNV1A(_) => FNV1A::hash(h),
            FOHash::XXHash32(_) => XXHash32::hash(h),
            FOHash::Murmur3(_) => Murmur3::hash(h),
            FOHash::XXHash64(_) => XXHash64::hash(h),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            FOHash::FNV1A(h) => h.name.as_str(),
            FOHash::XXHash32(h) => h.name.as_str(),
            FOHash::Murmur3(h) => h.name.as_str(),
            FOHash::XXHash64(h) => h.name.as_str(),
        }
    }

    pub fn is_64bits(&self) -> bool {
        match self {
            FOHash::FNV1A(_) => false,
            FOHash::XXHash32(_) => false,
            FOHash::Murmur3(_) => false,
            FOHash::XXHash64(_) => true,
        }
    }
}

// Second-order hash functions

#[derive(Debug, Clone, Copy)]
pub enum HashSeed {
    Bits32(u32),
    Bits64(u64),
    Bits128(u128),
}

impl Display for HashSeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashSeed::Bits32(x) => write!(f, "{}", x),
            HashSeed::Bits64(x) => write!(f, "{}", x),
            HashSeed::Bits128(x) => write!(f, "{}", x),
        }
    }
}

impl From<u32> for HashSeed {
    fn from(value: u32) -> Self {
        return HashSeed::Bits32(value);
    }
}

impl From<u64> for HashSeed {
    fn from(value: u64) -> Self {
        return HashSeed::Bits64(value);
    }
}

impl From<u128> for HashSeed {
    fn from(value: u128) -> Self {
        return HashSeed::Bits128(value);
    }
}

impl Into<u32> for HashSeed {
    fn into(self) -> u32 {
        match self {
            HashSeed::Bits32(x) => x,
            HashSeed::Bits64(x) => x as u32,
            HashSeed::Bits128(x) => x as u32,
        }
    }
}

impl Into<u64> for HashSeed {
    fn into(self) -> u64 {
        match self {
            HashSeed::Bits32(x) => x as u64,
            HashSeed::Bits64(x) => x,
            HashSeed::Bits128(x) => x as u64,
        }
    }
}

impl Into<u128> for HashSeed {
    fn into(self) -> u128 {
        match self {
            HashSeed::Bits32(x) => x as u128,
            HashSeed::Bits64(x) => x as u128,
            HashSeed::Bits128(x) => x,
        }
    }
}

impl HashSeed {
    pub fn as_u32(self) -> u32 {
        return self.into();
    }

    pub fn as_u64(self) -> u64 {
        return self.into();
    }

    pub fn as_u128(self) -> u128 {
        return self.into();
    }
}

#[derive(Debug, Clone)]
pub struct MXF {
    name: String,
    seed: HashSeed,
}

impl Default for MXF {
    fn default() -> Self {
        return Self {
            name: "mxf".to_string(),
            seed: HashSeed::Bits64(0),
        };
    }
}

impl MXF {
    fn hash(&self, key: HashKey) -> HashKey {
        let mut h: u64 = (key.as_u64()).wrapping_mul(self.seed.as_u64() | 1);
        h ^= h >> 33;
        h = h.wrapping_mul(0xff51afd7ed558ccd as u64);
        h ^= h >> 33;
        return HashKey::from(h as u32);
    }

    fn set_seed(&mut self, seed: HashSeed) {
        self.seed = seed;
    }
}

#[derive(Debug, Clone)]
pub struct MXF64 {
    name: String,
    seed: HashSeed,
}

impl Default for MXF64 {
    fn default() -> Self {
        return Self {
            name: "mxf64".to_string(),
            seed: HashSeed::Bits64(0),
        };
    }
}

impl MXF64 {
    fn hash(&self, key: HashKey) -> HashKey {
        let mut h: u64 = (key.as_u64()).wrapping_mul(self.seed.as_u64() | 1);
        h ^= h >> 33;
        h = h.wrapping_mul(0xff51afd7ed558ccd as u64);
        h ^= h >> 33;
        return HashKey::from(h);
    }

    fn set_seed(&mut self, seed: HashSeed) {
        self.seed = seed;
    }
}

#[derive(Debug, Clone)]
pub struct XorShift {
    name: String,
    seed: HashSeed,
}

impl Default for XorShift {
    fn default() -> Self {
        return Self {
            name: "xorshift".to_string(),
            seed: HashSeed::from(0 as u32),
        };
    }
}

impl XorShift {
    fn hash(&self, key: HashKey) -> HashKey {
        let mut h = key.as_u32().wrapping_mul(self.seed.as_u32() | 1);
        h = (h ^ 61) ^ (h >> 16);
        h = h.wrapping_mul(9);
        h = h ^ (h >> 4);
        h = h.wrapping_mul(0x27d4eb2d);
        h = h ^ (h >> 15);
        h ^= (1 + h) << 13;
        h ^= h >> 17;
        h ^= h << 5;
        return HashKey::from(h);
    }

    fn set_seed(&mut self, seed: HashSeed) {
        self.seed = seed;
    }
}

#[derive(Debug, Clone)]
pub enum SOHash {
    MXF(MXF),
    MXF64(MXF64),
    XorShift(XorShift),
}

impl Default for SOHash {
    fn default() -> Self {
        return SOHash::MXF(MXF::default());
    }
}

impl FromStr for SOHash {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mxf" => Ok(SOHash::MXF(MXF::default())),
            "mxf64" => Ok(SOHash::MXF64(MXF64::default())),
            "xorshift" => Ok(SOHash::XorShift(XorShift::default())),
            _ => {
                Err("Cannot find a corresponding second-order hash. Expected: mxf, xorshift".into())
            }
        }
    }
}

impl SOHash {
    pub fn hash(&self, key: HashKey) -> HashKey {
        match self {
            SOHash::MXF(x) => x.hash(key),
            SOHash::MXF64(x) => x.hash(key),
            SOHash::XorShift(x) => x.hash(key),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            SOHash::MXF(x) => x.name.as_str(),
            SOHash::MXF64(x) => x.name.as_str(),
            SOHash::XorShift(x) => x.name.as_str(),
        }
    }

    pub fn set_seed(&mut self, seed: HashSeed) {
        match self {
            SOHash::MXF(x) => x.set_seed(seed),
            SOHash::MXF64(x) => x.set_seed(seed),
            SOHash::XorShift(x) => x.set_seed(seed),
        }
    }

    pub fn seed(&self) -> HashSeed {
        match self {
            SOHash::MXF(x) => x.seed,
            SOHash::MXF64(x) => x.seed,
            SOHash::XorShift(x) => x.seed,
        }
    }

    pub fn is_64bits(&self) -> bool {
        match self {
            SOHash::MXF(_) => false,
            SOHash::MXF64(_) => true,
            SOHash::XorShift(_) => false,
        }
    }
}
