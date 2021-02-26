use serde::{Serialize, Deserialize};
use scraper::{Html, Selector};

#[derive(Debug, Serialize, Deserialize)]
pub struct Kitty {
    pub title: String,
    pub date: String,
    pub url: String,
}

fn is_ad(html: scraper::ElementRef) -> bool {
    let selector = Selector::parse("span").unwrap();
    for footer in html.select(&selector) {   
        for class in footer.value().classes() {
            if class.eq("mp-Listing-seller-link") {
                return true;
            }
        }
    }
    return false;
}

fn find_date(html: scraper::ElementRef) -> String {
    let selector = Selector::parse("span").unwrap();
    for footer in html.select(&selector) {   
        for class in footer.value().classes() {
            if class.eq("mp-Listing-date") {
                let text = footer.inner_html();
                return text;
            }
        }
    }
    return String::from("");
}

fn find_url(html: scraper::ElementRef) -> String {
    let selector = Selector::parse("a").unwrap();
    let mut url = String::new();
    for link in html.select(&selector) {
        for class in link.value().classes() {
            if class.eq("mp-Listing-coverLink") {
                url = format!(
                    "{}{}", 
                    "https://www.marktplaats.nl/",
                    link.value().attr("href").unwrap()
                ).to_string();
            }
        }
    }
    return url;
}

pub fn from_page(body: String) -> Vec<Kitty> {
    let mut list = Vec::new();
   
    let body: &str = body.as_str();
    let document = Html::parse_document(body);
    let listings = Selector::parse("li").unwrap();

    for element in document.select(&listings) {
        assert_eq!("li", element.value().name());

        let h3_selector = Selector::parse("h3").unwrap();
        if !is_ad(element) {
            for title in element.select(&h3_selector) {
                assert_eq!("h3", title.value().name());

                let url = find_url(element);
                let date = find_date(element);
                let kitty = Kitty { 
                    title: title.inner_html(),
                    date: date,
                    url: url,
                };
                list.push(kitty);
            }
        }      
    }

    list
}

impl Kitty {
    pub fn as_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
    /*
    fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).unwrap()
    }
    */
    pub fn seen(&self, db: &sled::Tree) -> bool {
        let bytes = self.title.as_bytes();
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