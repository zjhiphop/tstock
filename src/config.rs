// http headers
use reqwest::header::{self, HeaderMap};
use std::collections::HashMap;

fn get_headers() -> HeaderMap {

    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static("reqwest"));
    
    headers
}