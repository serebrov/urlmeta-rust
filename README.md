Async url parser web service: for a given URL, it loads a webpage, parses and returns json with metadata (title, description, etc).

Based on [actix-web](https://actix.rs/) and [scraper](https://docs.rs/scraper).

Similar node.js service is [here](https://github.com/serebrov/urlmeta-nodejs).
Similar python service is [here](https://github.com/serebrov/urlmeta-python).

Run `make up` to run docker setup or `cd app && cargo run` to run with `cargo`.

Curl example:

```
$ curl http://localhost:8000/url-parser\?target\=https://www.rust-lang.org/

{
    "description": "Empowering everyone to build reliable and efficient software.",
    "title": "Rust programming language",
    "locale": "en_US",
    "type": "website",
    "url": "",
    "image": "https://www.rust-lang.org/static/images/rust-social-wide.jpg",
    "video": ""
}

$ curl http://localhost:8000/url-parser\?target\=https://www.youtube.com/watch\?v\=fJ9rUzIMcZQ

{
    "description": "Subscribe to the official Queen channel Here http://bit.ly/Subscribe2Queen Taken from A Night At The Opera, 1975. Queen - 'Bohemian Rhapsody' Click here to b...",
    "title": "Queen - Bohemian Rhapsody (Official Video)",
    "locale": "",
    "type": "video.other",
    "url": "https://www.youtube.com/watch?v=fJ9rUzIMcZQ",
    "image": "https://i.ytimg.com/vi/fJ9rUzIMcZQ/hqdefault.jpg",
    "video": ""
}
```
