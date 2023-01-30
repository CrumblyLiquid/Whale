use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Index {
    // packages: Vec<Info>
    packages: HashMap<String, Info>,
}

impl Index {
    pub fn new(packages: HashMap<String, Info>) -> Self {
        Index { packages: packages }
    }

    pub fn to_vec(&self) -> Vec<Info> {
        self.packages.clone().into_values().collect::<Vec<Info>>()
        // &self.packages
    }

    pub fn is_allowed(&self, filename: &String) -> bool {
        self.packages.contains_key(filename)
    }

    pub fn get_info(&self, filename: &String) -> Option<&Info> {
        self.packages.get(filename)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Info {
    pub filename: String,
    pub name: String,
    pub summary: String,
    pub author: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Package {
    pub name: String,
    pub summary: String,
    pub author: String,
    pub native: String,
    pub foreign: String,
    pub inputs: Vec<Input>,
    pub words: Vec<Vec<String>>,
}

// impl Default for Package {
//     fn default() -> Self {
//         Self {
//             name: "".to_string(),
//             summary: "".to_string(),
//             author: "".to_string(),
//             // created: 0,
//             // modified: 0,
//             native: "".to_string(),
//             foreign: "".to_string(),
//             inputs: Vec::new(),
//             words: Vec::new(),
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Input {
    pub name: String,
    pub example: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}