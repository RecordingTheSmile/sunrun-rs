use lazy_static::lazy_static;

lazy_static! {
    static ref HTTP_CLIENT:reqwest::Client = {
        reqwest::ClientBuilder::new()
        .user_agent("sunrun-rs/1.0")
        .build()
        .unwrap()
    };
}

pub struct HttpClient;

impl HttpClient {
    pub fn client() -> &'static reqwest::Client {
        &HTTP_CLIENT
    }
}