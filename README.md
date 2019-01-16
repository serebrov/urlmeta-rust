Async url parser web service: for a given URL, it loads a webpage, parses and returns json with metadata (title, description, etc).

Based on [actix-web](https://actix.rs/) and [scraper](https://docs.rs/scraper).

Similar node.js service is [here](https://github.com/serebrov/urlmeta-nodejs).

Run `cd app && cargo run`.

Curl example:

```
$ curl http://localhost:8000/url-parser\?target\=https://www.nytimes.com/2018/10/01/opinion/justice-kavanaugh-recuse-himself.html

{"description":"Given his blatant partisanship and personal animosity toward liberals, how could he be an effective member of the Supreme Court?","title":"Opinion | All the Ways a Justice Kavanaugh Would Have to Recuse Himself","locale":"","type":"article","url":"https://www.nytimes.com/2018/10/01/opinion/justice-kavanaugh-recuse-himself.html","image":"https://static01.nyt.com/images/2018/10/01/opinion/01Tribe/01Tribe-facebookJumbo.jpg","video":""}%
```
