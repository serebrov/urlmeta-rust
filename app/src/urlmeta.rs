extern crate scraper;

use std::str;
use scraper::Html;
use scraper::Selector;
use scraper::ElementRef;


#[derive(Serialize, Deserialize)]
pub struct Metadata {
    description: String,
    title: String,
    locale: String,
    r#type: String,
    url: String,
    image: String,
    video: String,
}


type RuleCallback = fn(el: ElementRef) -> Option<String>;


struct Rule {
    selector: String,
    extract: RuleCallback,
}

// Define a method of the structure.
impl Rule {
    pub fn new(selector: &str, extract: RuleCallback) -> Rule {
        Rule {
            selector: String::from(selector),
            extract,
        }
    }
}


fn find(doc: &Html, rules: &[Rule]) -> String {
    for rule in rules {
        let sel = Selector::parse(&rule.selector).unwrap();
        for el in doc.select(&sel) {
            match (rule.extract)(el) {
                Some(value) => return value,
                None => continue
            };
        }
    }
    String::from("")
}


fn get_content(el: ElementRef) -> Option<String> {
    return match el.value().attr("content") {
        Some(value) => Some(String::from(value)),
        None => None
    }
}


fn get_text(el: ElementRef) -> Option<String> {
    return Some(el.inner_html());
}


pub fn parse_page(html: &str) -> Metadata {
    let doc = Html::parse_document(&html);
    println!("parsing the document");

    let result = Metadata {
        description: find(&doc, &[
            Rule::new(r#"meta[property="og:description"]"#, get_content),
            Rule::new(r#"meta[name="description"]"#, get_content),
        ]),
        title: find(&doc, &[
            Rule::new(r#"meta[property="og:title"]"#, get_content),
            Rule::new("title", get_text)
        ]),
        locale: find(&doc, &[
            Rule::new(r#"meta[property="og:locale"]"#, get_content),
        ]),
        r#type: find(&doc, &[
            Rule::new(r#"meta[property="og:type"]"#, get_content),
        ]),
        url: find(&doc, &[
            Rule::new(r#"meta[property="og:url"]"#, get_content),
        ]),
        image: find(&doc, &[
            Rule::new(r#"meta[property="og:image"]"#, get_content),
        ]),
        video: find(&doc, &[
            Rule::new(r#"meta[property="og:video"]"#, get_content),
        ]),
    };
    result
}
