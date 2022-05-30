#[macro_use]
extern crate serde;
extern crate actix_web;
extern crate serde_derive;
extern crate serde_json;

use api;
use hyper;
use std::io;

pub fn run() {
    let mut client = hyper::client::new();

    let resp = client.send("http://httpbin.org/", Delay { delay: 1 });

    println!("{:?}", resp);
}

struct Delay {
    delay: u8,
}

#[derive(Deserialize)]
struct Info {
    origin: String,
    headers: BTreeMap<String, String>,
}

impl api::Api for Delay {
    type Reply = Info;
    type Body = io::Empty;
    type Error = serde_json::Error;

    fn method(&self) -> api::Method {
        api::Method::Get
    }

    fn path(&self) -> api::Query {
        api::Query::new()
    }

    fn headers(&self) -> api::Headers {
        let mut headers = api::Headers::new();

        headers.insert("X-Request-ID", "abcde");

        headers
    }

    fn body(&self) -> io::Empty {
        io::empty()
    }

    // section for parse
    fn parse<R: actix_web::HttpResponse>(&self, resp: &mut R) -> Result<Info, serde_json::Error> {
        serde_json::from_reader(resp.body())
    }
}
