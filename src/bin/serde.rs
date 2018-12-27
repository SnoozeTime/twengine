use serde_json::{Value, Error};
use serde_derive::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
}

fn main() {

    let data = r#"{
        "name":"John"
    }"#;

    let p: Person = serde_json::from_str(data).unwrap();

    println!("{:?}", p);
}


