use std::collections::HashMap;

use reqwest::{header};

fn get_headers() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 6.3; WOW64; Trident/7.0; Touch; rv:11.0) like Gecko"));
    headers.insert(header::ACCEPT, header::HeaderValue::from_static("*/*"));
    headers.insert(header::ACCEPT_LANGUAGE, header::HeaderValue::from_static("zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2"));
    
    headers
}

fn get_quotes_id(stock_code: &str)-> Result<String, Box<dyn std::error::Error>> {
    if stock_code.len() == 0 {
        return Ok(("".to_string()));
    }

    let url = "http://searchapi.eastmoney.com/api/suggest/get";
    let headers = get_headers();
    let mut params = HashMap::new();
    
    params.insert("input", stock_code);
    params.insert("type", "14");
    params.insert("token", "D43BF722C8E33BDC906FB84D85E326E8");
    params.insert("count", "1");
    
    println!("params: {:?}, headers: {:#?}", params, headers);

    let client = reqwest::blocking::Client::new();

    let response = client
        .get(url)
        .headers(headers)
        .query(&params)
        .send()?;

    let body = response.text()?;

    println!("{:?}", body);

    return Ok(body);
}

fn get_stock_ticks(stock_code: &str) -> Result<(), Box<dyn std::error::Error>>{
      // Set up the URL and query parameters
      let url = "http://push2.eastmoney.com/api/qt/stock/fflow/kline/get";
      let mut params = HashMap::new();
      /*
          ('lmt', '0'),
          ('klt', '1'),
          ('secid', quote_id),
          ('fields1', 'f1,f2,f3,f7'),
          ('fields2', 'f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63'),
       */
      params.insert("lmt", "0");
      params.insert("klt", "1");
      params.insert("secid", stock_code);
      params.insert("fields1", "f1,f2,f3,f7");
      params.insert("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63");
  
      println!("{:#?}", params);
  
      // Set up the headers
      let headers = get_headers();
      println!("{:#?}", headers);
  
      let client = reqwest::blocking::Client::new();
  
      let response = client
          .get(url)
          .headers(headers)
          .query(&params)
          .send()?;
      let body = response.text()?;
      println!("{}", body);

      Ok(())
}

// Get daily data split by minutes
fn main() {
    
    let result = get_quotes_id("600519");

    println!("{:?}", result);

}