use serde;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::io::Read;

fn main() {
    /*
    o_json = '["foo", {"bar" : ["baz", null, 1.0,2]}]';
    o = json.loads(o_json)
    print(o)
    */
    let o_json = r#"
{
    "args": {},
                "headers": {
      "Accept": "*/*",
      "Host": "httpbin.org",
      "X-Amzn-Trace-Id": "Root=1-60f96022-64d8f64d0bf3040b5ef895d6"
    },
    "origin": "158.175.119.4",
    "url": "http://httpbin.org/get"
}"#;
    let o: serde_json::Value = serde_json::from_str(&o_json).unwrap();
    println!("{:?}", &o);
    /*
    s = json.dumps(["foo", {"bar": ("baz", "s", 1.0,2)}])
    print(s)
    */
    let o = json!(["foo", {"bar": ("baz", "s", 1.0,2)}]);
    println!("{:?}", &o);
    /*
    o = json.loads(s)
    print(o)
    */
    let s = &o.to_string();
    println!("{}", &s);

    // Next section, JSON with a struct:
    let bob = Human {
        name: "Bob".to_string(),
        age: 12,
        verified: true,
        job: None,
        parents: ["Alice".to_string(), "Carl".to_string()].to_vec(),
    };
    // Serialize to JSON:
    let json = serde_json::to_string(&bob).unwrap();
    println!("{}", &json);
    // deserialize again:
    let person: Human = serde_json::from_str(&json).unwrap();
    println!("{:?}", person);
    simple_req();
    another_simple_req();
}

// derive Debug makes it printable
// derive Deserialize and Serialize adds decoder and encoder magic to the class
#[derive(Debug, Deserialize, Serialize)]
struct Human {
    name: String,
    age: usize,
    job: Option<String>,
    verified: bool,
    parents: Vec<String>,
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
fn another_simple_req() -> Result<(), Box<dyn Error>> {
    let mut res = reqwest::blocking::get("https://pipl.ir/v1/getPerson")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);
    let o: serde_json::Value = serde_json::from_str(&body).unwrap();
    println!("{:?}", &o);
    println!("Putting the data into the struct:");
    let struct_example: Data = serde_json::from_str(&body).unwrap();
    println!("{:?}", &struct_example);
    println!("\n\nDude is making {}", &struct_example.person.work.salary);
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    person: Person,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub education: Education,
    pub marriage: Marriage,
    #[serde(rename = "online_info")]
    pub online_info: OnlineInfo,
    pub personal: Personal,
    pub work: Work,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Education {
    pub certificate: String,
    pub university: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Marriage {
    pub married: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OnlineInfo {
    pub email: String,
    #[serde(rename = "ip_address")]
    pub ip_address: String,
    #[serde(rename = "ipv6_address")]
    pub ipv6_address: String,
    pub password: String,
    #[serde(rename = "password_md5")]
    pub password_md5: String,
    #[serde(rename = "user_agent")]
    pub user_agent: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Personal {
    pub age: i64,
    pub blood: String,
    pub born: ::serde_json::Value,
    #[serde(rename = "born_place")]
    pub born_place: String,
    pub cellphone: String,
    pub city: String,
    pub country: String,
    #[serde(rename = "eye_color")]
    pub eye_color: String,
    #[serde(rename = "father_name")]
    pub father_name: String,
    pub gender: String,
    pub height: String,
    #[serde(rename = "last_name")]
    pub last_name: String,
    pub name: String,
    #[serde(rename = "national_code")]
    pub national_code: String,
    pub religion: String,
    #[serde(rename = "system_id")]
    pub system_id: String,
    pub weight: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Work {
    #[serde(rename = "country_code")]
    pub country_code: String,
    pub insurance: bool,
    pub position: String,
    pub salary: String,
}
