use serde::{Serialize, Deserialize};
use scraper::{Html, Selector};

#[derive(Debug, Serialize, Deserialize)]
pub struct Kitty {
    pub title: String,
}

pub fn from_page(body: String) -> Vec<Kitty> {
    let mut list = Vec::new();
   
    let body: &str = body.as_str();
    let document = Html::parse_document(body);
    let listings = Selector::parse("li").unwrap();

    for element in document.select(&listings) {
        assert_eq!("li", element.value().name());
        let h3_selector = Selector::parse("h3").unwrap();
        for title in element.select(&h3_selector) {
            assert_eq!("h3", title.value().name());
            //log::info!("{}", title.inner_html());
            let kitty = Kitty { 
                title: title.inner_html(),
            };
            list.push(kitty);
        }      
    }

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
    let filename = "test_page.txt";
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    let body: &str = content.as_str();

    let document = Html::parse_document(body);
    let listings = Selector::parse("li").unwrap();

    for element in document.select(&listings) {
        assert_eq!("li", element.value().name());
        log::info!("{}", element.value().name());
    }
    
}