use std::env;
use std::time::Duration;

use error_chain::error_chain;
use reqwest::{header, Client, ClientBuilder, Response};
use serde::Deserialize;

error_chain! {
    foreign_links {
        EnvVarError(env::VarError);
        HttpRequestError(reqwest::Error);
    }
}

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[derive(Deserialize)]
struct ApiResponse {
    dependencies: Vec<Dependency>,
    meta: Meta,
}

#[derive(Deserialize)]
struct Dependency {
    crate_id: String,
}

#[derive(Deserialize)]
struct Meta {
    total: u32,
}

struct ReverseDependencies {
    crate_id: String,
    dependencies: <Vec<Dependency> as IntoIterator>::IntoIter,
    client: reqwest::blocking::Client,
    page: u32,
    per_page: u32,
    total: u32,
}

impl ReverseDependencies {
    fn of(crate_id: &str) -> Result<Self> {
        Ok(ReverseDependencies {
            crate_id: crate_id.to_owned(),
            dependencies: vec![].into_iter(),
            client: reqwest::blocking::Client::new(),
            page: 0,
            per_page: 100,
            total: 0,
        })
    }

    fn try_next(&mut self) -> Result<Option<Dependency>> {
        if let Some(dependency) = self.dependencies.next() {
            return Ok(Some(dependency));
        }

        if self.page > 0 && self.page * self.per_page >= self.total {
            return Ok(None);
        }

        self.page += 1;
        let url: String = format!(
            "https://crates.io/api/v1/crates/{}/reverse_dependencies?page={}&per_page={}",
            self.crate_id, self.page, self.per_page
        );

        let response: ApiResponse = self.client.get(&url).send()?.json::<ApiResponse>()?;

        if response.dependencies.is_empty() {
            return Err("Missing field `dependecies`in the response)".into());
        }
        self.dependencies = response.dependencies.into_iter();
        self.total = response.meta.total;
        Ok(self.dependencies.next())
    }
}

impl Iterator for ReverseDependencies {
    type Item = Result<Dependency>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(dependency)) => Some(Ok(dependency)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

#[tokio::main]
pub async fn query_github_api() -> Result<()> {
    let request_url: String = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    println!("Request URL: {}", request_url);

    let client = Client::new();
    let response = client
        .get(&request_url)
        .header(header::USER_AGENT, "rust-cookbook")
        .send()
        .await?;

    let body: String = response.text().await?;

    let users: Vec<User> = serde_json::from_str(&body).unwrap();

    for user in users {
        println!("User login; {}, user ID: {}", user.login, user.id);
    }

    Ok(())
}

#[tokio::main]
pub async fn check_whether_gh_api_exists() -> Result<()> {
    let user: &str = "TheLionCoder";
    let request_url: String = format!("https://api.github.com/users/{}", user);

    println!("{}", request_url);

    let timeout: Duration = Duration::new(5, 0);
    let client: Client = ClientBuilder::new().timeout(timeout).build()?;

    let response: Response = client
        .get(&request_url)
        .header(header::USER_AGENT, "rust-cookbook")
        .send()
        .await?;

    if response.status().is_success() {
        println!("User {} exists", user);
    } else {
        println!("User {} does not exist", user);
    }
    Ok(())
}

pub fn consume_paginated_api() -> Result<()> {
    for dependency in ReverseDependencies::of("serde")? {
        println!("reverse dependency: {}", dependency?.crate_id);
    }
    Ok(())
}
