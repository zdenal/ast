#![feature(plugin, custom_derive)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

// '{and: ["some.value","some.value"], not: "true"}'

#[derive(Deserialize, Debug)]
struct And {
  //#[serde(deserialize_with="deserialize_operator")]
  and: Vec<Code>,
  not: bool
}

#[derive(Deserialize, Debug)]
struct Or {
  or: Vec<String>,
  not: bool
}

#[derive(Deserialize, Debug)]
struct Code(String);

#[derive(Deserialize, Debug)]
pub enum Operator {
  And,
  Or,
  Code
}

//fn deserialize_operator<D>(de: &mut D) -> Result<Operator, D::Error>
        //where D: serde::Deserializer
    //{
        //let deser_result: serde_json::Value = try!(serde::Deserialize::deserialize(de));
        //match deser_result {
            //serde_json::Value::String(ref s) if &*s == "and" => Ok(Code),
            //serde_json::Value::String(ref s) if &*s == "or" => Ok(Operator::Or),
            //serde_json::Value::String(ref s) => Ok(Operator::Code),
            //_ => Err(serde::de::Error::custom("Unexpected value")),
        //}
    //}

fn main() {
    let expression = "{\"and\":[\"some.value\",\"some.value\"], \"not\": \"true\"}";

    let deserialized: And = serde_json::from_str(&expression).unwrap();

    println!("Origin expression: {:?}", deserialized);
}
