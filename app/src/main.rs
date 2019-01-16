extern crate actix_web;
extern crate scraper;

use std::str;

use actix_web::{client, server, App, Error, HttpMessage, HttpRequest, HttpResponse};

use futures::{future::ok as fut_ok, Future};

use scraper::Html;
use scraper::Selector;

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

fn greet_async(_req: HttpRequest) -> impl Future<Item=HttpResponse, Error=Error> {
    let url = "https://www.nytimes.com/2018/10/01/opinion/justice-kavanaugh-recuse-himself.html";
    fetch_page(url).and_then(|html| {
        let doc = Html::parse_document(&html[..]);
        let mut result = String::new();
        let selectors = [
           "meta[property=\"og:description\"]",
           "meta[property=\"og:title\"]",
           "meta[property=\"og:locale\"]",
           "meta[property=\"og:type\"]",
           "meta[property=\"og:url\"]",
           "meta[property=\"og:image\"]",
           "meta[property=\"og:video\"]"
        ];
        for selector in &selectors {
            let sel = Selector::parse(selector).unwrap();
            for el in doc.select(&sel) {
                result = format!("{}/n{}", result, el.value().attr("content").unwrap_or(""));
                break;
            }
        }
        let resp = HttpResponse::Ok().body(result);
        Ok(resp.into())
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
