Async url parser web service: for a given URL, it loads a webpage, parses and returns json with metadata (title, description, etc).

Based on [actix-web](https://actix.rs/) and [scraper](https://docs.rs/scraper).

Similar node.js service is [here](https://github.com/serebrov/urlmeta-nodejs).

Run `cd app && cargo run`.

Curl example:

```
curl http://localhost:8000/url-parser\?target\=https://www.nytimes.com
```
