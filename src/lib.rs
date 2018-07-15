#![allow(unknown_lints)]

extern crate either;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate itertools;
extern crate serde_json;

mod matcher;
mod merger;
mod schema;
mod spreader;
mod unit;

pub use schema::Schema;

#[derive(Debug)]
pub enum Verdict {
    Success,
    Failure,
}

pub fn check(derived: Schema, base: Schema) -> Verdict {
    let derived = spreader::spread(derived);
    let base = spreader::spread(base);

    //println!("========= DERIVED ====");
    //println!("{:#?}", derived);
    //println!("=========  BASE   ====");
    //println!("{:#?}", base);

    for d in &derived {
        let is_subtype = base.iter().any(|b| matcher::subtype(d, b));

        if !is_subtype {
            return Verdict::Failure;
        }
    }

    Verdict::Success
}
