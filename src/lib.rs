#![crate_name = "github"]
#![crate_type = "rlib"]

extern crate hyper;
extern crate rustc_serialize;

use client::Client;
use users::UserService;

pub struct GitHub {
    pub users: UserService,
}

impl GitHub {
    pub fn new(tok: String) -> GitHub {
        let client = Client::new(tok);
        GitHub {
            users: UserService::new(client),
        }
    }
}

pub mod client;
pub mod users;
