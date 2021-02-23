use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Kitty {}

pub fn from_page(body: String) -> Vec<Kitty> {
    let list = Vec::new();
    list
}

impl Kitty {
    pub fn as_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
    fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).unwrap()
    }

    pub fn seen(&self, db: &sled::Tree) -> bool {
        let bytes = self.as_bytes();
        db.contains_key(bytes).unwrap()
    }
}

#[test]
fn test_from_page() {
    let page = include_str!("../test_page.txt");
    let kitties = from_page(page.to_string());
    dbg!(kitties);
    assert!(false);
}