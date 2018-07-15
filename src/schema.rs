use std::collections::HashMap;
use std::rc::Rc;

use serde_json::Value;

pub type RcMixed = Rc_<Value>;
pub type RcStr = Rc_<String>;

#[allow(derive_hash_xor_eq)]
#[derive(Debug, Default, Hash, Clone, Eq, Deserialize)]
pub struct Rc_<T>(Rc<T>);

impl<T: PartialEq> PartialEq for Rc_<T> {
    fn eq(&self, other: &Rc_<T>) -> bool {
        Rc::ptr_eq(&self.0, &other.0) || self.0 == other.0
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Schema {
    Simple(bool),
    Complex(Box<Restrictions>),
}

#[derive(Debug, Deserialize)]
pub struct Restrictions {
    #[serde(rename = "$id")]
    pub id: Option<RcStr>,
    #[serde(rename = "$ref")]
    pub ref_: Option<RcStr>,
    #[serde(rename = "$schema")]
    pub schema: Option<RcStr>,
    pub title: Option<RcStr>,
    pub description: Option<RcStr>,
    pub default: Option<RcMixed>,
    #[serde(rename = "multipleOf")]
    pub multiple_of: Option<f64>,
    pub maximum: Option<f64>,
    #[serde(rename = "exclusiveMaximum")]
    pub exclusive_maximum: Option<f64>,
    pub minimum: Option<f64>,
    #[serde(rename = "exclusiveMinimum")]
    pub exclusive_minimum: Option<f64>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<u32>,
    #[serde(rename = "minLength")]
    pub min_length: Option<u32>,
    pub pattern: Option<RcStr>,
    #[serde(rename = "additionalItems")]
    pub additional_items: Option<Schema>,
    pub items: Option<Items>,
    #[serde(rename = "maxItems")]
    pub max_items: Option<u32>,
    #[serde(rename = "midItems")]
    pub min_items: Option<u32>,
    #[serde(rename = "uniqueItems")]
    pub unique_items: Option<bool>,
    #[serde(rename = "maxProperties")]
    pub max_properties: Option<u32>,
    #[serde(rename = "minProperties")]
    pub min_properties: Option<u32>,
    pub required: Option<Vec<RcStr>>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<Schema>,
    #[serde(rename = "additionalNames")]
    pub property_names: Option<Schema>,
    pub contains: Option<Schema>,
    pub format: Option<RcStr>,
    pub definitions: Option<HashMap<RcStr, Schema>>,
    pub properties: Option<HashMap<RcStr, Schema>>,
    #[serde(rename = "patternProperties")]
    pub pattern_properties: Option<HashMap<RcStr, Schema>>,
    pub dependencies: Option<HashMap<RcStr, Dependency>>,
    #[serde(rename = "enum")]
    pub enum_: Option<Vec<RcMixed>>,
    #[serde(rename = "const")]
    pub const_: Option<RcMixed>,
    #[serde(rename = "type")]
    pub type_: Option<Types>,
    #[serde(rename = "allOf")]
    pub all_of: Option<Vec<Schema>>,
    #[serde(rename = "anyOf")]
    pub any_of: Option<Vec<Schema>>,
    #[serde(rename = "oneOf")]
    pub one_of: Option<Vec<Schema>>,
    pub not: Option<Schema>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Types {
    One(Type),
    Any(Vec<Type>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Type {
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "null")]
    Null,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Items {
    Array(Schema),
    Tuple(Vec<Schema>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    Property(Vec<RcStr>),
    Schema(Schema),
}
