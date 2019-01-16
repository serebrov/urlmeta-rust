extern crate actix_web;
extern crate scraper;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::str;

use actix_web::{client, server, App, Error, HttpMessage, HttpRequest, HttpResponse};

use futures::{future::ok as fut_ok, Future};

use scraper::Html;
use scraper::Selector;
use scraper::ElementRef;

fn fetch_page(url: &str) -> impl Future<Item=String, Error=Error> {
    println!("fetch page: {}", url);

    client::ClientRequest::get(url)
        .finish().unwrap()
        .send()
        .map_err(|err| {
            println!("error: {:?}", err);
            Error::from(err)        // <- convert SendRequestError to an Error
        })
        .and_then(
            |resp| {
                // println!("response: {:?}", resp);
                resp.body()          // <- this is MessageBody type, resolves to complete body
                .from_err()          // <- convert PayloadError to an Error
                .and_then(|body| {
                    // let data: String = std::str::from_utf8(&body).unwrap().clone();
                    let data: String = String::from(
                        std::str::from_utf8(&body).unwrap());
                    fut_ok(data)
                })
            }
        )
}

#[derive(Serialize, Deserialize)]
struct Metadata {
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

fn parse_page(html: &str) -> Metadata {
    let doc = Html::parse_document(&html);

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

fn greet_async(req: HttpRequest) -> impl Future<Item=HttpResponse, Error=Error> {
    let params = req.query();
    let url = params.get("target").expect("The `target` parameter is not specified");
    fetch_page(url).and_then(|html| {
        let result = parse_page(&html);
        let body = serde_json::to_string(&result)?;
        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    })
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/{name}", |r| r.with_async(greet_async))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
