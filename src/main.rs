#![feature(plugin, custom_derive)]
#![feature(default_type_parameter_fallback)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Deserializer};
use serde::de::{SeqVisitor, MapVisitor, Visitor, Error};

#[derive(Deserialize, Debug)]
struct AndExpression {
	children: Vec<Expression>,
	not: bool
}

#[derive(Deserialize, Debug)]
struct OrExpression {
	children: Vec<Expression>,
	not: bool
}

#[derive(Deserialize, Debug)]
struct CodeExpression(String);

#[derive(Debug)]
enum Expression {
	And(AndExpression),
	Or(OrExpression),
	Code(CodeExpression)
}

#[derive(Debug)]
enum Key {
  And,
  Or,
  Not
}

struct ExpressionVec(Vec<Expression>);

impl Deserialize for Key {
  fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
    where D: Deserializer,
  {
    let value = try!(serde_json::Value::deserialize(deserializer));
    let key = value.as_str();

    match key {
      Some("and") => Ok(Key::And),
      Some("or") => Ok(Key::Or),
      Some("not") => Ok(Key::Not),
      _ => Err(D::Error::unknown_field("key"))
    }
  }
}

impl Deserialize for ExpressionVec {
  fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
    where D: Deserializer,
  {
		struct ExpressionVecVisitor;

    impl Visitor for ExpressionVecVisitor {
      type Value = ExpressionVec;

      fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: SeqVisitor
        {
          let mut expressions: Vec<Expression> = Vec::new();

          while let Some(expression) = try!(visitor.visit::<Expression>()) {
            expressions.push(expression);
          }

          try!(visitor.end());

          Ok(ExpressionVec(expressions))
        }
    }

    deserializer.deserialize(ExpressionVecVisitor)
  }
}

impl Deserialize for Expression {
	fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
		where D: Deserializer,
	{
		struct ExpressionVisitor;

    impl Visitor for ExpressionVisitor {
      type Value = Expression;

      fn visit_str<E>(&mut self, value: &str) -> Result<Self::Value, E>
        where E: Error
        {
          return Ok(Expression::Code(CodeExpression(value.to_string())));
        }

      fn visit_map<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: MapVisitor
        {
          let mut operator = Key::Not;
          let mut expression_vec = ExpressionVec(Vec::<Expression>::new());
          let mut not: bool = false;

          while let Some(key) = try!(visitor.visit_key::<Key>()) {
            match key {
              Key::Not => {
                not = try!(visitor.visit_value::<bool>());
              },
              Key::And => {
                operator = Key::And;
                expression_vec = try!(visitor.visit_value::<ExpressionVec>());
              },
              Key::Or => {
                operator = Key::Or;
                expression_vec = try!(visitor.visit_value::<ExpressionVec>());
              }
            }
          }

          try!(visitor.end());

          let ExpressionVec(children) = expression_vec;

          return match operator {
            Key::And => Ok(Expression::And(AndExpression {children: children, not: not})),
            Key::Or => Ok(Expression::Or(OrExpression {children: children, not: not})),
            _ => Err(Error::unknown_field("unkonwn field"))
          }
        }
    }

		deserializer.deserialize_map(ExpressionVisitor)
	}
}

fn main() {
    let expression = "{\"and\":[\"some.value\",\"some.value\", {\"or\":[\"some.value\",\"some.value\"]}], \"not\": \"true\"}";

    let deserialized: Expression = serde_json::from_str(&expression).unwrap();

    println!("Origin expression: {:?}", deserialized);
}
