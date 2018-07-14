use std::collections::HashMap;
use std::iter::{self, FromIterator};

use either::Either;
use itertools::Itertools;

use merger;
use schema::{Items, RcMixed, RcStr, Schema, Type, Types};
use unit::{Point, Unit};

pub fn spread(schema: Schema) -> Vec<Unit> {
    let res = if let Schema::Complex(res) = schema {
        *res
    } else {
        unimplemented!();
    };

    let mut unit = Unit {
        multiple_of: res.multiple_of,
        max_length: res.max_length,
        min_length: res.min_length,
        max_items: res.max_items,
        min_items: res.min_items,
        max_properties: res.max_properties,
        min_properties: res.min_properties,
        unique_items: res.unique_items.unwrap_or(false),
        required: res.required.unwrap_or_else(Vec::new),
        format: res.format,
        const_: res.const_,
        ..Unit::default()
    };

    unit.maximum = match (&res.maximum, &res.exclusive_maximum) {
        (Some(inc), Some(exc)) => Some(Point::inc(*inc).min(Point::exc(*exc))),
        (Some(inc), None) => Some(Point::inc(*inc)),
        (None, Some(exc)) => Some(Point::exc(*exc)),
        (None, None) => None,
    };

    unit.minimum = match (&res.minimum, &res.exclusive_minimum) {
        (Some(inc), Some(exc)) => Some(Point::inc(*inc).max(Point::exc(*exc))),
        (Some(inc), None) => Some(Point::inc(*inc)),
        (None, Some(exc)) => Some(Point::exc(*exc)),
        (None, None) => None,
    };

    if let Some(pattern) = res.pattern {
        unit.pattern.push(pattern);
    }

    let types = match res.type_ {
        Some(Types::One(type_)) => {
            unit.type_ = Some(type_);
            Vec::new()
        }
        Some(Types::Any(types)) => types,
        None => Vec::new(),
    };

    let enums = res.enum_.unwrap_or_else(Vec::new);
    let additional_items = res.additional_items.map_or_else(Vec::new, spread);
    let additional_properties = res.additional_properties.map_or_else(Vec::new, spread);
    let property_names = res.property_names.map_or_else(Vec::new, spread);
    let contains = res.contains.map_or_else(Vec::new, spread);
    let items = match res.items {
        Some(Items::Array(schema)) => spread(schema),
        Some(Items::Tuple(_)) => unimplemented!(),
        None => Vec::new(),
    };

    let properties = res.properties.map_or_else(Vec::new, spread_map);
    let pattern_props = res.pattern_properties.map_or_else(Vec::new, spread_map);
    let any_of = res.any_of.unwrap_or_else(Vec::new);
    let all_of = res.all_of.unwrap_or_else(Vec::new);

    let mut it = iter::once(unit);

    // TODO: find compromise between performance and monomorphization size.
    let mut it = spread_nested(&mut it, &types, save_type);
    let mut it = spread_nested(&mut it, &enums, save_const);
    let mut it = spread_nested(&mut it, &additional_items, save_additional_items);
    let mut it = spread_nested(&mut it, &additional_properties, save_additional_props);
    let mut it = spread_nested(&mut it, &property_names, save_property_names);
    let mut it = spread_nested(&mut it, &contains, save_contains);
    let mut it = spread_nested(&mut it, &items, save_items);
    let mut it = spread_nested(&mut it, &properties, save_properties);
    let it = spread_nested(&mut it, &pattern_props, save_pattern_props);

    let it = spread_any_of(it, any_of);
    let it = spread_all_of(it, all_of);
    // TODO: one_of
    // TODO: not

    it.collect()
}

fn spread_nested<'a, T: Clone>(
    units: &'a mut Iterator<Item = Unit>,
    nested: &'a [T],
    save: fn(&mut Unit, T),
) -> impl Iterator<Item = Unit> + 'a {
    if nested.is_empty() {
        return Either::Left(units);
    }

    let it = units.flat_map(move |unit| {
        nested.into_iter().map(move |nest| {
            let mut unit = unit.clone();
            save(&mut unit, nest.clone());
            unit
        })
    });

    Either::Right(it)
}

fn spread_map(map: HashMap<RcStr, Schema>) -> Vec<HashMap<RcStr, Unit>> {
    map.into_iter()
        .map(|(key, value)| {
            spread(value)
                .into_iter()
                .map(|unit| (key.clone(), unit))
                .collect::<Vec<_>>()
        })
        .multi_cartesian_product()
        .map(HashMap::from_iter)
        .collect()
}

fn spread_any_of(
    common: impl Iterator<Item = Unit>,
    schemas: Vec<Schema>,
) -> impl Iterator<Item = Unit> {
    if schemas.is_empty() {
        return Either::Left(common);
    }

    let variants = schemas.into_iter().flat_map(spread).collect::<Vec<_>>();

    // TODO: remove unnecessary `src` cloning.
    let it = common.cartesian_product(variants).map(|(src, mut dst)| {
        merger::merge(&mut dst, &src);
        dst
    });

    Either::Right(it)
}

fn spread_all_of(
    common: impl Iterator<Item = Unit>,
    schemas: Vec<Schema>,
) -> impl Iterator<Item = Unit> {
    if schemas.is_empty() {
        return Either::Left(common);
    }

    let it = iter::once(common.collect())
        .chain(schemas.into_iter().map(spread))
        .multi_cartesian_product()
        .map(|mut units| {
            let mut dst = units.swap_remove(0);

            for src in units {
                merger::merge(&mut dst, &src);
            }

            dst
        });

    Either::Right(it)
}

fn save_type(dst: &mut Unit, type_: Type) {
    dst.type_ = Some(type_);
}

fn save_const(dst: &mut Unit, const_: RcMixed) {
    // TODO: resolve conflicts beetween enum and const.
    dst.const_ = Some(const_);
}

fn save_additional_items(dst: &mut Unit, unit: Unit) {
    dst.additional_items = Some(Box::new(unit));
}

fn save_additional_props(dst: &mut Unit, unit: Unit) {
    dst.additional_props = Some(Box::new(unit));
}

fn save_property_names(dst: &mut Unit, unit: Unit) {
    dst.property_names = Some(Box::new(unit));
}

fn save_contains(dst: &mut Unit, unit: Unit) {
    dst.contains = Some(Box::new(unit));
}

fn save_items(dst: &mut Unit, unit: Unit) {
    dst.items = Some(Box::new(unit));
}

fn save_properties(dst: &mut Unit, props: HashMap<RcStr, Unit>) {
    dst.properties = props;
}

fn save_pattern_props(dst: &mut Unit, props: HashMap<RcStr, Unit>) {
    dst.pattern_props = props;
}
