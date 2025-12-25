use std::path::PathBuf;
use regex::Regex;
use super::hash::{FOHash,SOHash,Hashable};

#[derive(Debug, Clone)]
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

impl ItemType {
    pub fn hashable(&self) -> Hashable {
        match self {
            ItemType::Str(s) => s.as_bytes().to_vec(),
            ItemType::I64(i64) => i64.to_le_bytes().to_vec(),
            ItemType::I32(i32) => i32.to_le_bytes().to_vec(),
            ItemType::U64(u64) => u64.to_le_bytes().to_vec(),
            ItemType::U32(u32) => u32.to_le_bytes().to_vec(),
            _ => Hashable::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Item {
    data: ItemType,
    foh: u32,
}

impl Item {
    pub fn new(item_type: ItemType, hasher: &FOHash) -> Item {
        return Item {
            foh: hasher.hash(&item_type.hashable()),
            data: item_type,
        }
    }

    pub fn key(&self) -> u32 {
        return self.foh;
    }
}

#[derive(Debug, Default, Clone)]
pub struct Bucket {
    items: Vec<Item>,
    so_hash: SOHash,
}

type Buckets = Vec<Bucket>;

#[derive(Debug, Default)]
pub struct PHash {
    buckets: Buckets,
    fo_hash: FOHash,
}

impl PHash {
    pub fn from_file(file_path: &PathBuf) -> Result<PHash, Box<dyn std::error::Error>> {
        let fo_hasher = FOHash::default();

        let mut phash = PHash::default();

        let file_content = std::fs::read_to_string(file_path).expect("Unable to read file");

        let sep = Regex::new(r"([\n,]+)").expect("Invalid regex");

        // We use n / 2 as the number of buckets. Could be changed to n / 4
        let n = (sep.find_iter(file_content.as_str()).size_hint().1.unwrap_or(10) / 2) as u32;

        phash.buckets.resize(n as usize, Bucket::default());
        phash.fo_hash = FOHash::default();

        for s in sep.split(file_content.as_str()).into_iter() {
            let item = Item::new(ItemType::Str(s.to_string()), &phash.fo_hash);

            phash.buckets[(item.key() % n) as usize].items.push(item);
        }

        println!("{:?}", phash);

        return Ok(phash);
    }
}
