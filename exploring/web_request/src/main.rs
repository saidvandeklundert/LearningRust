use serde;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::io::Read;
use structopt::StructOpt;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

// cargo run -- -u www.nu.nl
fn main() {
    let args = Args::from_args();
    println!("{:?}", args);
    println!("{:?}", args.url);
    let url = args.url;
    if let Some(url) = url {
        req(&url).unwrap();
    }
    //simple_req();
}

#[derive(Debug, StructOpt)]
#[structopt(name = "target website", about = "reqwest a website")]
struct Args {
    /// target hosts
    #[structopt(short = "u", long)]
    url: Option<String>,
}

fn req(url: &str) -> Result<(), Box<dyn Error>> {
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);
    let o: serde_json::Value = serde_json::from_str(&body).unwrap();
    println!("{:?}", &o);

    let struct_example: HttpBin = serde_json::from_str(&body).unwrap();
    println!("{:?}", &struct_example);

    Ok(())
}

fn simple_req() -> Result<(), Box<dyn Error>> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);
    let o: serde_json::Value = serde_json::from_str(&body).unwrap();
    println!("{:?}", &o);

    let struct_example: HttpBin = serde_json::from_str(&body).unwrap();
    println!("{:?}", &struct_example);

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct HttpBin {
    origin: String,
    url: String,
}
