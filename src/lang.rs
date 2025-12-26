use crate::hash::HashSeed;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::any::type_name;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use super::phash::ItemType;

fn get_exe_dir() -> PathBuf {
    std::env::current_exe()
        .expect("Cannot get executable path")
        .parent()
        .expect("Cannot get executable dir")
        .to_path_buf()
}

#[derive(Deserialize, Debug, Clone)]
pub struct FOHashData {
    pub body: String,
    pub imports: Option<String>,
    pub typedefs: Option<String>,
}

#[derive(Deserialize, Debug)]
struct FOHashConfig {
    functions: HashMap<String, HashMap<String, FOHashData>>,
}

static FO_HASHES: Lazy<FOHashConfig> = Lazy::new(|| {
    let path = get_exe_dir().join("res").join("fo_hash.json");

    let content = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));

    return serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse {}: {}", path.display(), e));
});

#[derive(Deserialize, Debug, Clone)]
pub struct SOHashData {
    pub body: String,
    pub imports: Option<String>,
    pub typedefs: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SOHashConfig {
    functions: HashMap<String, HashMap<String, SOHashData>>,
}

static SO_HASHES: Lazy<SOHashConfig> = Lazy::new(|| {
    let path = get_exe_dir().join("res").join("so_hash.json");

    let content = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));

    return serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse {}: {}", path.display(), e));
});

#[derive(Deserialize, Debug, Clone)]
pub struct GetData {
    pub body: String,
}

#[derive(Deserialize, Debug)]
struct GetConfig {
    functions: HashMap<String, GetData>,
}

static GETS: Lazy<GetConfig> = Lazy::new(|| {
    let path = get_exe_dir().join("res").join("get.json");

    let content = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));

    return serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse {}: {}", path.display(), e));
});

#[derive(Debug)]
pub enum OutputLang {
    C,
    Python,
}

impl From<&str> for OutputLang {
    fn from(s: &str) -> OutputLang {
        match s {
            "c" => OutputLang::C,
            "py" => OutputLang::Python,
            _ => panic!("Cannot convert extension to Language Type"),
        }
    }
}

impl ToString for OutputLang {
    fn to_string(&self) -> String {
        match self {
            OutputLang::C => "c".to_string(),
            OutputLang::Python => "py".to_string(),
        }
    }
}

impl OutputLang {
    pub fn get_line_end(&self) -> &str {
        match self {
            OutputLang::C => ";",
            OutputLang::Python => "",
        }
    }

    pub fn map_seed(&self, seed: &HashSeed) -> &'static str {
        match self {
            OutputLang::C => match seed {
                HashSeed::Bits32(_) => "unsigned int",
                HashSeed::Bits64(_) => "unsigned long long",
                HashSeed::Bits128(_) => panic!("128-bits hash seeds are not supported in C"),
            },
            OutputLang::Python => "int",
        }
    }

    pub fn map_type<T>(&self, _: &T) -> &'static str {
        match self {
            OutputLang::C => match type_name::<T>() {
                "u32" => "unsigned int",
                "u64" => "unsigned long long",
                _ => panic!("Unknown Rust type to map"),
            },
            OutputLang::Python => match type_name::<T>() {
                "u32" => "int",
                "u64" => "int",
                _ => panic!("Unknown Rust type to map"),
            },
        }
    }

    pub fn get_comment_start(&self) -> &str {
        match self {
            OutputLang::C => "/*",
            OutputLang::Python => "#",
        }
    }

    pub fn get_comment_end(&self) -> &str {
        match self {
            OutputLang::C => "*/",
            OutputLang::Python => "",
        }
    }

    pub fn get_type(&self, t: &ItemType) -> &str {
        match self {
            OutputLang::C => match t {
                ItemType::Str(_) => "char*",
                ItemType::I32(_) => "int",
                ItemType::I64(_) => "long long",
                ItemType::U32(_) => "unsigned int",
                ItemType::U64(_) => "unsigned long long",
            },
            OutputLang::Python => match t {
                ItemType::Str(_) => "str",
                ItemType::I32(_) => "int",
                ItemType::I64(_) => "int",
                ItemType::U32(_) => "int",
                ItemType::U64(_) => "int",
            },
        }
    }

    pub fn get_imports_from_type(&self, t: &ItemType) -> Option<String> {
        match self {
            OutputLang::C => match t {
                ItemType::Str(_) => Some("#include <string.h>\n".to_string()),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn get_imports_for_test(&self, t: &ItemType) -> Option<String> {
        match self {
            OutputLang::C => match t {
                ItemType::Str(_) => Some("#include <assert.h>\n".to_string()),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn get_array_decl(&self) -> &str {
        match self {
            OutputLang::C => "const {type} {name}[{size}]",
            OutputLang::Python => "{name}",
        }
    }

    pub fn get_array_start(&self) -> &str {
        match self {
            OutputLang::C => "{",
            OutputLang::Python => "[",
        }
    }

    pub fn get_array_end(&self) -> &str {
        match self {
            OutputLang::C => "}",
            OutputLang::Python => "]",
        }
    }

    pub fn get_array_sep(&self) -> &str {
        match self {
            OutputLang::C => ",",
            OutputLang::Python => ",",
        }
    }

    pub fn get_key_address(&self, t: &ItemType) -> &str {
        match self {
            OutputLang::C => match t {
                ItemType::Str(_) => "",
                ItemType::I32(_) => "&",
                ItemType::I64(_) => "&",
                ItemType::U32(_) => "&",
                ItemType::U64(_) => "&",
            },
            _ => "",
        }
    }

    pub fn get_key_size(&self, t: &ItemType, key_name: &str) -> String {
        match self {
            OutputLang::C => match t {
                ItemType::Str(_) => format!("strlen({key_name})"),
                ItemType::I32(_) => "sizeof(int)".to_string(),
                ItemType::I64(_) => "sizeof(long int)".to_string(),
                ItemType::U32(_) => "sizeof(unsigned int)".to_string(),
                ItemType::U64(_) => "sizeof(unsigned long int)".to_string(),
            },
            _ => "".to_string(),
        }
    }

    pub fn get_key_conversion_start(&self, t: &ItemType) -> &str {
        match self {
            OutputLang::Python => match t {
                ItemType::Str(_) => "bytes(",
                _ => "",
            },
            _ => "",
        }
    }

    pub fn get_key_conversion_end(&self, t: &ItemType) -> &str {
        match self {
            OutputLang::Python => match t {
                ItemType::Str(_) => ".encode(errors=\"replace\"))",
                ItemType::I32(_) => "to_bytes(4)",
                ItemType::I64(_) => "to_bytes(8)",
                ItemType::U32(_) => "to_bytes(4)",
                ItemType::U64(_) => "to_bytes(8)",
            },
            _ => "",
        }
    }

    pub fn get_fo_hash_data(&self, name: &str) -> Option<FOHashData> {
        return FO_HASHES
            .functions
            .get(name)
            .and_then(|map| map.get(&self.to_string()))
            .cloned();
    }

    pub fn get_so_hash_data(&self, name: &str) -> Option<SOHashData> {
        return SO_HASHES
            .functions
            .get(name)
            .and_then(|map| map.get(&self.to_string()))
            .cloned();
    }

    pub fn get_get_data(&self) -> Option<GetData> {
        return GETS.functions.get(&self.to_string()).cloned();
    }
}
