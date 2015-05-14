extern crate hyper;
extern crate mime;

use hyper::client::{self, response};
use hyper::header::{UserAgent, Accept, QualityItem, Quality};
use hyper::mime::Mime;

//#[derive(Copy)]
pub struct Client {
    pub tok: String,
    pub client: client::Client,
}

static BASE_URL : &'static str  = "https://api.github.com";

impl Client {
    pub fn new(tok: String) -> Client {
        let client = client::Client::new();
        Client {
            tok: tok,
            client: client,
        }
    }

    pub fn request(&mut self, url: String) -> hyper::Result<response::Response> {
        let mime: Mime = "application/vnd.github.v3+json".parse().unwrap();
        let qual = QualityItem::new(mime, Quality(1));
        let mut accept_type = Vec::new();
        accept_type.push(qual);
        let url2 = format!("{}{}", BASE_URL, url);
        let mut client = String::new();
        client.push_str("RustGithubClient");
        return self.client.get(&*url2)
            .header(UserAgent(client))
            .header(Accept(accept_type))
            .send();
    }
}
