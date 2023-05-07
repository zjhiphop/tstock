use std::ops::Sub;
use std::{collections::HashMap, ptr::null, fmt::Error};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Local, Duration};
use chrono::format::ParseError;

use stock::search::{SearchResult};
use reqwest::{header};
use stock::ticks::QuoteResponse;

#[derive(Debug)]
struct StockTick {
    time: NaiveDateTime,
    open: f64,
    close: f64,
    interval: u8
}

impl StockTick {
    pub fn new(time: NaiveDateTime, open: f64, close: f64, time_klt: u8) -> StockTick {
        StockTick {
            time,
            open,
            close,
            interval: time_klt
        }
    }
}

fn get_headers() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 6.3; WOW64; Trident/7.0; Touch; rv:11.0) like Gecko"));
    headers.insert(header::ACCEPT, header::HeaderValue::from_static("*/*"));
    headers.insert(header::ACCEPT_LANGUAGE, header::HeaderValue::from_static("zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2"));
    
    headers
}

fn get_quotes_id(stock_code: &str)-> Result<SearchResult, Box<dyn std::error::Error>> {
    if stock_code.len() == 0 {
        return Err(Box::new(Error));
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

    let body: SearchResult = response.json::<SearchResult>()?;

    // todo: save query result to local db
    // println!("{:?}", body.quotation_code_table.data[0].quote_id);

    return Ok(body);
}

fn get_stock_ticks(quote_id: &str, time_klt: u8) -> Result<(Vec<String>), Box<dyn std::error::Error>>{
      // Set up the URL and query parameters
      let url = "https://push2his.eastmoney.com/api/qt/stock/kline/get";
      let mut params = HashMap::new();
      /*
          ('lmt', '0'),
          ('klt', '1'),
          ('secid', quote_id),
          ('fields1', 'f1,f2,f3,f7'),
          ('fields2', 'f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63'),
       */

      let today = Local::now();
      let ten_days_ago = today.sub(Duration::days(3)).format("%Y%m%d").to_string();
      let today = today.format("%Y%m%d").to_string();
      let time_range = format!("{}", time_klt);

      params.insert("beg", ten_days_ago.as_str());
      params.insert("end", today.as_str());
      params.insert("fqt", "1");
      params.insert("rtntype", "6");
      params.insert("klt", time_range.as_str()); // per day data
      params.insert("secid", quote_id);
      params.insert("fields1", "f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f11,f12,f13");
      params.insert("fields2", "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61");
  
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
    //   let body = response.text()?;
    //   println!("{}", body);
    let body: QuoteResponse = response.json::<QuoteResponse>()?;
    

      Ok((body.data.klines))
}

// Get daily data split by minutes
fn main() -> Result<(), ParseError> {
    
    let result = get_quotes_id("600519");
    let quote_id = &result.unwrap().quotation_code_table.data[0].quote_id;

    println!("{:?}", quote_id);
    let time_klt = 30; // tick duction by 30 mins
    let ticks = get_stock_ticks(quote_id, time_klt).unwrap();

    // println!("{:?}", ticks);

    // let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
    // println!("{}", date_only);

    // convert tick data to vec ticks list 
    let mut klineVec: Vec<StockTick> = vec![];

    
    let ticks_iter = ticks.iter();

    for tick in ticks_iter {
        let tick_item = tick.split(',').collect::<Vec<_>>();
        let date =  NaiveDateTime::parse_from_str(tick_item[0], "%Y-%m-%d %H:%M").unwrap();
        let close = tick_item[2].parse::<f64>().unwrap();
        let open =  tick_item[1].parse::<f64>().unwrap();

        println!("Date: {:?}", date);
        // println!("Tick: {:?}", tick_value);

        let item = StockTick::new(date, open, close, time_klt);

        klineVec.push(item)
    }

    println!("{:?}", klineVec);
    
    Ok(())
}