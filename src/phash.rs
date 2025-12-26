use super::hash::{FOHash, Hashable, SOHash, HashKey, MXF64};
use regex::Regex;
use std::{path::PathBuf, str::FromStr};
use std::cmp::max;

// https://cmph.sourceforge.net/papers/esa09.pdf

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemType {
    Str(String),
    I64(i64),
    I32(i32),
    U64(u64),
    U32(u32),
}

impl Default for ItemType {
    fn default() -> Self {
        return ItemType::Str(String::default());
    }
}

impl ToString for ItemType {
    fn to_string(&self) -> String {
        match self {
            ItemType::Str(s) => format!("\"{}\"", s),
            ItemType::I64(i64) => i64.to_string(),
            ItemType::I32(i32) => i32.to_string(),
            ItemType::U64(u64) => u64.to_string(),
            ItemType::U32(u32) => u32.to_string(),
        }
    }
}

impl ItemType {
    pub fn hashable(&self) -> Hashable {
        match self {
            ItemType::Str(s) => s.as_bytes().to_vec(),
            ItemType::I64(i64) => i64.to_le_bytes().to_vec(),
            ItemType::I32(i32) => i32.to_le_bytes().to_vec(),
            ItemType::U64(u64) => u64.to_le_bytes().to_vec(),
            ItemType::U32(u32) => u32.to_le_bytes().to_vec(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Item {
    data: ItemType,
    key: HashKey,
    final_pos: u32,
}

impl Item {
    pub fn new(item_type: ItemType, hasher: &FOHash) -> Item {
        return Item {
            key: hasher.hash(&item_type.hashable()),
            data: item_type,
            final_pos: 0,
        };
    }

    pub fn item_type(&self) -> &ItemType {
        return &self.data;
    }

    pub fn key(&self) -> HashKey {
        return self.key;
    }
}

#[derive(Debug, Default, Clone)]
pub struct Bucket {
    items: Vec<Item>,
    so_hash: SOHash,
}

impl Bucket {
    pub fn new(so_hash: SOHash) -> Bucket {
        let mut bucket = Bucket::default();
        bucket.so_hash = so_hash;

        return bucket;
    }
    pub fn so_hash(&self) -> &SOHash {
        return &self.so_hash;
    }
}

type Buckets = Vec<Bucket>;

#[derive(Debug, Default)]
pub struct PHash {
    buckets: Buckets,
    fo_hash: FOHash,
    so_hash: SOHash,
    m: usize,
}

impl PHash {
    fn new(
        first_order_hash: &str,
        second_order_hash: &str,
    ) -> Result<PHash, Box<dyn std::error::Error>> {
        let mut phash = PHash::default();

        phash.fo_hash = FOHash::from_str(first_order_hash)?;

        phash.so_hash = SOHash::from_str(second_order_hash)?;

        if phash.fo_hash().is_64bits() && !phash.so_hash().is_64bits() {
            println!("Second-order hash {} is not 64-bits, switching to mxf64", phash.so_hash().name());
            phash.so_hash = SOHash::MXF64(MXF64::default());
        }

        return Ok(phash);
    }

    pub fn from_file(
        file_path: &PathBuf,
        first_order_hash: &str,
        second_order_hash: &str,
    ) -> Result<PHash, Box<dyn std::error::Error>> {
        println!("Generating perfect hash for file: \"{}\"", file_path.display());

        let mut phash = PHash::new(first_order_hash, second_order_hash)?;

        println!("First-order hash: {}", phash.fo_hash().name());
        println!("Second-order hash: {}", phash.so_hash().name());

        let file_content = std::fs::read_to_string(file_path).expect("Unable to read file");

        let sep = Regex::new(r"([\n,]+)").expect("Invalid regex");

        let mut m = 0;

        for _ in sep.find_iter(file_content.as_str()) {
            m += 1;
        }

        // We use m / 2 as the number of buckets. Could be changed to m / 4
        let n = ((m as f64) * 0.1) as usize;

        println!("Using {n} buckets");

        phash.buckets = vec![Bucket::new(phash.so_hash.clone()); n];

        for s in sep.split(file_content.as_str()).into_iter() {
            if s.len() == 0 {
                continue;
            }

            let item = Item::new(ItemType::Str(s.to_string()), &phash.fo_hash);
            let item_key = (item.key() % n as u32) as usize;

            // TODO: remove, can hurt performance
            if phash.buckets[item_key].items.iter().find(|x| x.data == item.data).is_some() {
                println!("Found duplicate: {}, removing it", item.data.to_string());
                m -= 1;
                continue;
            }

            if let Some(found) = phash.buckets[item_key].items.iter().find(|x| x.key() == item.key()) {
                println!("Found collision: {} / {} (key: {}), aborting",
                         item.data.to_string(),
                         found.data.to_string(),
                         item.key());
                m -= 1;
                continue;
            }

            phash.buckets[item_key].items.push(item);
        }

        println!("Found {m} items to process for the perfect hash table");
        phash.m = m;

        let mut sorted_buckets: Vec<&mut Bucket> = Vec::new();
        sorted_buckets.extend(&mut phash.buckets);
        sorted_buckets.sort_by_key(|item| std::cmp::Reverse(item.items.len()));

        let mut occupied = vec![false; m as usize];
        let total = sorted_buckets.iter().filter(|b| !b.items.is_empty()).count();
        let mut done = 0;

        for bucket in sorted_buckets.iter_mut() {
            if bucket.items.len() == 0 {
                continue;
            }

            let mut collision = true;

            // println!("{:?}", bucket.items);

            let mut candidate_pos: Vec<u32> = Vec::new();

            while collision {
                if bucket.so_hash.is_64bits() {
                    bucket.so_hash.set_seed(rand::random::<u64>().into());
                } else {
                    bucket.so_hash.set_seed(rand::random::<u32>().into());
                }

                collision = false;
                candidate_pos.clear();

                for item in bucket.items.iter_mut() {
                    let pos = bucket.so_hash.hash(item.key()) % m as u32;

                    if occupied[pos as usize] {
                        collision = true;
                        break;
                    }

                    if candidate_pos.iter().find(|&x| *x == pos).is_some() {
                        collision = true;
                        break;
                    }

                    candidate_pos.push(pos);

                    item.final_pos = pos;
                }

                if !collision {
                    for pos in candidate_pos.iter() {
                        occupied[*pos as usize] = true;
                    }
                }
            }

            done += 1;

            if done == total || done % max(1, total.strict_div_euclid(1000)) == 0 {
                print!(
                    "\rProgress: {}/{} ({:.1}%)   ",
                    done,
                    total,
                    (done as f64 / total as f64) * 100.0
                );
            }
        }

        println!("");

        return Ok(phash);
    }

    pub fn m(&self) -> usize {
        return self.m;
    }

    pub fn fo_hash(&self) -> &FOHash {
        return &self.fo_hash;
    }

    pub fn so_hash(&self) -> &SOHash {
        return &self.so_hash;
    }

    pub fn buckets(&self) -> &Buckets {
        return &self.buckets;
    }

    pub fn first_bucket(&self) -> Option<&Bucket> {
        return self.buckets.first();
    }

    pub fn first_item(&self) -> Option<&Item> {
        for bucket in self.buckets.iter() {
            if bucket.items.is_empty() {
                continue;
            }

            return bucket.items.first();
        }

        return None;
    }

    pub fn items(&self) -> Vec<&Item> {
        let mut result = vec![None; self.m];

        for bucket in self.buckets.iter() {
            for item in bucket.items.iter() {
                result[item.final_pos as usize] = Some(item);
            }
        }

        return result.into_iter().flatten().collect();
    }
}
