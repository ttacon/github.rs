
use rustc_serialize::json;
use client::Client;
use std::io::Read;

pub struct UserService {
    pub client: Client,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct User {
    pub login: String,
    pub id: i32,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub site_admin: bool,
    pub name: String,
    pub company: String,
    pub blog: String,
    pub location: String,
    pub email: String,
    pub hireable: bool,
    pub bio: Option<String>,
    pub public_repos: i32,
    pub public_gists: i32,
    pub followers: i32,
    pub following: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl UserService {
    pub fn new(client: Client) -> UserService {
        UserService {
            client: client,
        }
    }

    pub fn get_user(&mut self, user_name: String)  -> User {
        let res = self.client.request(format!("{}{}", "/users/", user_name));
        let mut resp = res.unwrap();

        let mut content = String::new();
        resp.read_to_string(&mut content);

        let u: User = json::decode(&content).unwrap();
        return u;
    }
}

#[test]
fn get_user() {
    let mut username = String::new();
    username.push_str("ttacon");
    let mut us = UserService::new(Client::new(String::new()));
    let user = us.get_user(username);
    println!("user: {:?}", user);
}
