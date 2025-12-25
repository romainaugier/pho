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
pub enum FOHash {
    FNV1A(FNV1A),
}

impl Default for FOHash {
    fn default() -> Self {
        return FOHash::FNV1A(FNV1A::default());
    }
}

impl FOHash {
    pub fn hash(&self, h: &Hashable) -> u32 {
        match self {
            FOHash::FNV1A(_) => FNV1A::hash(h),
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
        let mut h: u64 = (key as u64) * ((&self.seed | 1) as u64);
        h ^= h >> 33;
        h *= 0xff51afd7ed558ccd as u64;
        h ^= h >> 33;
        return h as u32;
    }

    fn set_seed(&mut self, seed: u32) {
        self.seed = seed;
    }
}

#[derive(Debug, Clone)]
pub enum SOHash {
    MXF(MXF),
}

impl Default for SOHash {
    fn default() -> Self {
        return SOHash::MXF(MXF::default());
    }
}

impl SOHash {
    pub fn hash(&self, key: u32) -> u32 {
        match self {
            SOHash::MXF(mxf) => mxf.hash(key),
        }
    }

    pub fn set_seed(&mut self, seed: u32) {
        match self {
            SOHash::MXF(mxf) => mxf.set_seed(seed),
        }
    }
}
