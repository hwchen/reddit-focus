#[macro_use]
extern crate failure;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use failure::Error;
use reqwest::header::{USER_AGENT, AUTHORIZATION};
use std::collections::HashMap;
use std::env;

const TOKEN_ENDPOINT: &str = "https://www.reddit.com/api/v1/access_token";
const MY_USER_AGENT: &str = "linux:reddit-focus:v0.0.1 (by /u/hwchen)";
const CLIENT_ID: &str = "8YAYsOZ17tKZ_g";

fn main() {
    if let Err(err) = run() {
        println!("{}", err.cause());
    }
}

fn run() -> Result<(), Error> {
    // get # of posts to get
    let mut args = env::args();
    let n: usize = args
        .by_ref()
        .skip(1)
        .next()
        .ok_or(format_err!("please enter number of posts to get"))?
        .parse()?;
    let subreddit = args.next().unwrap_or("rust".to_owned());

    // get reddit api call and
    let client = reqwest::Client::new();

    //let secret = env::var("REDDIT_FOCUS_SECRET")?;
    //let token = get_token(&client, TOKEN_ENDPOINT, CLIENT_ID, &secret)?.access_token;

    // get latest post id

    // now get posts since latest post id
    let token = "";
    let (after, new_posts) = get_new_posts(&client, &subreddit, token, n)?;

    for post in new_posts.iter() {
        println!("\n{}", post.title);
        if post.domain != "self.rust" {
            println!("  {}", post.url);
        }
        println!("  https://reddit.com{}", post.permalink);
    }

    //for header in res.headers().iter() {
    //    println!("{:?}", header);
    //}

    Ok(())
}

fn get_token(client: &reqwest::Client, token_endpoint: &str, client_id: &str, secret: &str) -> Result<TokenResponse, Error> {

    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");

    Ok(client.post(token_endpoint)
        .basic_auth(client_id, Some(secret))
        .form(&params)
        .send()?
        .json()?
    )
}

fn get_new_posts(
    client: &reqwest::Client,
    subreddit: &str,
    token: &str,
    n: usize,
    ) -> Result<(String, Vec<PostData>), Error>
{
    let url = format!("https://oauth.reddit.com/r/{}/new.json", subreddit);
    let res: RedditNew = client.get(&url)
        .header(USER_AGENT, MY_USER_AGENT)
        .header(AUTHORIZATION, format!("Bearer {}", token.to_owned()))
        .query(&[("limit", n)])
        .send()?
        .json()?;

    Ok((
        res.data.after.clone(),
        res.data.children.into_iter().map(|post| post.data).collect()
    ))
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
    pub scope: String,
}

#[derive(Debug, Deserialize)]
struct RedditNew {
    pub kind: String,
    pub data: RedditNewData,
}

#[derive(Debug, Deserialize)]
struct RedditNewData {
    pub after: String,
    pub children: Vec<Post>,
}

#[derive(Debug, Deserialize)]
struct Post {
    pub kind: String,
    pub data: PostData,
}

#[derive(Debug, Deserialize)]
struct PostData {
    pub id: String,
    pub title: String,
    pub name: String,
    pub author: String,
    pub subreddit_id: String,
    pub subreddit: String,
    pub subreddit_name_prefixed: String,
    pub selftext: String,
    pub permalink: String,
    pub domain: String,
    pub url: String,
}
