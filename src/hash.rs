use std::str::FromStr;

pub type Hashable = Vec<u8>;

// First-order hash functions

#[derive(Debug, Clone)]
pub struct FNV1A {
    name: String,
}

impl Default for FNV1A {
    fn default() -> Self {
        return Self {
            name: "fnv1a".to_string()
        };
    }
}

impl FNV1A {
    fn hash(h: &Hashable) -> u32 {
        let mut result = 0x811c9dc5 as u32;

        for d in h {
            result ^= *d as u32;
            result = result.wrapping_mul(0x01000193 as u32);
        }

        return result;
    }
}

#[derive(Debug, Clone)]
pub struct XXHash32 {
    name: String,
}

impl Default for XXHash32 {
    fn default() -> Self {
        return Self {
            name: "xxhash32".to_string()
        };
    }
}

impl XXHash32 {
    fn hash(h: &Hashable) -> u32 {
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

        return res;
    }
}

#[derive(Debug, Clone)]
pub struct Murmur3 {
    name: String,
}

impl Default for Murmur3 {
    fn default() -> Self {
        return Self {
            name: "murmur3".to_string()
        };
    }
}

impl Murmur3 {
    fn hash(h: &Hashable) -> u32 {
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
            let mut k = u32::from_le_bytes(data[i..i+4].try_into().unwrap());
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

        hash
    }
}

#[derive(Debug, Clone)]
pub enum FOHash {
    FNV1A(FNV1A),
    XXHash32(XXHash32),
    Murmur3(Murmur3),
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
            _ => Err("Cannot find a corresponding first-order hash. Expected: fnv1a, xxhash32".into()),
        }
    }
}

impl FOHash {
    pub fn hash(&self, h: &Hashable) -> u32 {
        match self {
            FOHash::FNV1A(_) => FNV1A::hash(h),
            FOHash::XXHash32(_) => XXHash32::hash(h),
            FOHash::Murmur3(_) => Murmur3::hash(h),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            FOHash::FNV1A(h) => h.name.as_str(),
            FOHash::XXHash32(h) => h.name.as_str(),
            FOHash::Murmur3(h) => h.name.as_str(),
        }
    }
}

// Second-order hash functions

#[derive(Debug, Clone)]
pub struct MXF {
    name: String,
    seed: u32,
}

impl Default for MXF {
    fn default() -> Self {
        return Self {
            name: "mxf".to_string(),
            seed: 0,
        };
    }
}

impl MXF {
    fn hash(&self, key: u32) -> u32 {
        let mut h: u64 = (key as u64).wrapping_mul((&self.seed | 1) as u64);
        h ^= h >> 33;
        h = h.wrapping_mul(0xff51afd7ed558ccd as u64);
        h ^= h >> 33;
        return h as u32;
    }

    fn set_seed(&mut self, seed: u32) {
        self.seed = seed;
    }
}

#[derive(Debug, Clone)]
pub struct XorShift {
    name: String,
    seed: u32,
}

impl Default for XorShift {
    fn default() -> Self {
        return Self {
            name: "xorshift".to_string(),
            seed: 0,
        }
    }
}

impl XorShift {
    fn hash(&self, key: u32) -> u32 {
        let mut h = key.wrapping_mul(self.seed | 1);
        h = (h ^ 61) ^ (h >> 16);
        h = h.wrapping_mul(9);
        h = h ^ (h >> 4);
        h = h.wrapping_mul(0x27d4eb2d);
        h = h ^ (h >> 15);
        h ^= (1 + h) << 13;
        h ^= h >> 17;
        h ^= h << 5;
        return h;
    }

    fn set_seed(&mut self, seed: u32) {
        self.seed = seed;
    }
}

#[derive(Debug, Clone)]
pub enum SOHash {
    MXF(MXF),
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
            "xorshift" => Ok(SOHash::XorShift(XorShift::default())),
            _ => Err("Cannot find a corresponding second-order hash. Expected: mxf, xorshift".into()),
        }
    }
}

impl SOHash {
    pub fn hash(&self, key: u32) -> u32 {
        match self {
            SOHash::MXF(x) => x.hash(key),
            SOHash::XorShift(x) => x.hash(key),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            SOHash::MXF(x) => x.name.as_str(),
            SOHash::XorShift(x) => x.name.as_str(),
        }
    }

    pub fn set_seed(&mut self, seed: u32) {
        match self {
            SOHash::MXF(x) => x.set_seed(seed),
            SOHash::XorShift(x) => x.set_seed(seed),
        }
    }

    pub fn seed(&self) -> u32 {
        match self {
            SOHash::MXF(x) => x.seed,
            SOHash::XorShift(x) => x.seed,
        }
    }
}
