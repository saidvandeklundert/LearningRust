use serde::{Deserialize, Serialize};
use serde_json::json;

fn main() {
    /*
    o_json = '["foo", {"bar" : ["baz", null, 1.0,2]}]';
    o = json.loads(o_json)
    print(o)
    */
    let o_json = r#"["foo", {"bar" : ["baz", null, 1.0,2]}]"#;
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
    let bob = Person {
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
    let person: Person = serde_json::from_str(&json).unwrap();
    println!("{:?}", person);
}

// derive Debug makes it printable
// derive Deserialize and Serialize adds decoder and encoder magic to the class
#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: usize,
    job: Option<String>,
    verified: bool,
    parents: Vec<String>,
}
