extern crate actix_web;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

use std::str;
use actix_web::{client, server, App, Error, HttpMessage, HttpRequest, HttpResponse};
use futures::{future::ok as fut_ok, Future};

mod urlmeta;

fn fetch_page(url: &str) -> impl Future<Item=String, Error=Error> {
    println!("fetch page: {}", url);

    // client::ClientRequest::get(url)
    //     .finish().unwrap()
    //     .send()
    //     .map_err(|err| {
    //         println!("error: {:?}", err);
    //         Error::from(err)        // <- convert SendRequestError to an Error
    //     })
    //     .and_then(
    //         |resp| {
    //             // println!("response: {:?}", resp);
    //             resp.body()          // <- this is MessageBody type, resolves to complete body
    //             // .from_err()          // <- convert PayloadError to an Error
    //             .map_err(|err| {
    //                 println!("error: {:#?}", err);
    //                 Error::from(err)        // <- convert PavloadError to an Error
    //             })
    //             .and_then(|body| {
    //                 let data: String = String::from(
    //                     std::str::from_utf8(&body).unwrap());
    //                 fut_ok(data)
    //             })
    //         }
    //     )
    use std::mem;
    use std::io::{self, Cursor};
    use futures::{Future, Stream};
    use reqwest::r#async::{Client, Decoder};

    Client::new().get(url)
        .send()
        .and_then(|resp| {
            println!("{}", resp.status());

            let body = mem::replace(resp.body_mut(), Decoder::empty());
            body.concat2()
        })
        .map_err(|err| {
            println!("error: {:?}", err);
            Error::from(err)
        })
        .map(|body| {
            // let mut body = Cursor::new(body);
            // let _ = io::copy(&mut body, &mut io::stdout())
            //     .map_err(|err| {
            //         println!("stdout error: {}", err);
            //     });

            println!("body: {:?}", body);
            let data: String = String::from(
               std::str::from_utf8(&body).unwrap());
            fut_ok(data)
        })
}


fn greet_async(req: HttpRequest) -> impl Future<Item=HttpResponse, Error=Error> {
    let params = req.query();
    let url = params.get("target").expect("The `target` parameter is not specified");
    fetch_page(url).and_then(|html| {
        let result = urlmeta::parse_page(&html);
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
