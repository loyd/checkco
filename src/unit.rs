use std::collections::HashMap;

use schema::{RcMixed, RcStr, Type};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Unit {
    pub const_: Option<RcMixed>,
    pub multiple_of: Option<f64>,
    pub maximum: Option<Point>,
    pub minimum: Option<Point>,
    pub max_length: Option<u32>,
    pub min_length: Option<u32>,
    pub pattern: Vec<RcStr>,
    pub additional_items: Option<Box<Unit>>,
    pub items: Option<Box<Unit>>,
    pub tuple: Vec<Unit>,
    pub max_items: Option<u32>,
    pub min_items: Option<u32>,
    pub unique_items: bool,
    pub max_properties: Option<u32>,
    pub min_properties: Option<u32>,
    pub required: Vec<RcStr>,
    pub additional_props: Option<Box<Unit>>,
    pub property_names: Option<Box<Unit>>,
    pub contains: Option<Box<Unit>>,
    pub format: Option<RcStr>,
    // TODO: should we use a persistent structure here?
    pub properties: HashMap<RcStr, Unit>,
    pub pattern_props: HashMap<RcStr, Unit>,
    // TODO: dependencies
    pub type_: Option<Type>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub value: f64,
    pub inclusive: bool,
}

impl Point {
    pub fn exc(value: f64) -> Point {
        Point {
            value,
            inclusive: false,
        }
    }

    pub fn inc(value: f64) -> Point {
        Point {
            value,
            inclusive: true,
        }
    }

    // TODO: is it commutative?
    #[allow(float_cmp)]
    pub fn min(self, other: Point) -> Point {
        if self.value < other.value || self.value == other.value && other.inclusive {
            self
        } else {
            other
        }
    }

    #[allow(float_cmp)]
    pub fn max(self, other: Point) -> Point {
        if self.value > other.value || self.value == other.value && other.inclusive {
            self
        } else {
            other
        }
    }
}
