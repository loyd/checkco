use std::cmp;
use std::collections::HashMap;

use schema::RcStr;
use unit::{Point, Unit};

pub fn merge(dst: &mut Unit, src: &Unit) -> bool {
    if !(merge_unique(&mut dst.const_, &src.const_)
        && merge_unique(&mut dst.type_, &src.type_)
        && merge_unique(&mut dst.format, &src.format)
        && merge_nested(&mut dst.items, &src.items)
        && merge_nested(&mut dst.additional_items, &src.additional_items)
        && merge_nested(&mut dst.additional_props, &src.additional_props)
        && merge_nested(&mut dst.property_names, &src.property_names)
        && merge_nested(&mut dst.contains, &src.contains)
        && merge_tuple(&mut dst.tuple, &src.tuple))
        && merge_props(&mut dst.properties, &src.properties)
        && merge_props(&mut dst.pattern_props, &src.pattern_props)
    {
        return false;
    }

    // TODO: multiple_of

    merge_point(&mut dst.maximum, &src.maximum, Point::min);
    merge_point(&mut dst.minimum, &src.minimum, Point::max);

    merge_count(&mut dst.max_length, &src.max_length, cmp::min);
    merge_count(&mut dst.min_length, &src.min_length, cmp::max);
    merge_count(&mut dst.max_items, &src.max_items, cmp::min);
    merge_count(&mut dst.min_items, &src.min_items, cmp::max);
    merge_count(&mut dst.max_properties, &src.max_properties, cmp::min);
    merge_count(&mut dst.min_properties, &src.min_properties, cmp::max);

    dst.unique_items = dst.unique_items || src.unique_items;

    merge_set(&mut dst.pattern, &src.pattern);
    merge_set(&mut dst.required, &src.required);

    true
}

fn merge_point<F>(dst: &mut Option<Point>, src: &Option<Point>, strategy: F)
where
    F: Fn(Point, Point) -> Point,
{
    *dst = match (*dst, *src) {
        (Some(d), Some(s)) => Some(strategy(d, s)),
        (x, y) => x.or(y),
    };
}

fn merge_count<T, F>(dst: &mut Option<T>, src: &Option<T>, strategy: F)
where
    T: Ord + Copy,
    F: Fn(T, T) -> T,
{
    *dst = match (&*dst, src) {
        (Some(a), Some(b)) => Some(strategy(*a, *b)),
        (x, y) => x.or(*y),
    };
}

fn merge_nested(dst: &mut Option<Box<Unit>>, src: &Option<Box<Unit>>) -> bool {
    if let Some(s) = src {
        if let Some(d) = dst {
            if !merge(d, s) {
                return false;
            }
        } else {
            *dst = src.clone();
        }
    }

    true
}

fn merge_tuple(dst: &mut Vec<Unit>, src: &[Unit]) -> bool {
    if dst.is_empty() {
        dst.extend(src.iter().cloned());
        return true;
    }

    // TODO: should we allow it if we get additional_props?
    if dst.len() != src.len() {
        return false;
    }

    for (d, s) in dst.iter_mut().zip(src.iter()) {
        if !merge(d, s) {
            return false;
        }
    }

    true
}

fn merge_set<T: PartialEq + Clone>(dst: &mut Vec<T>, src: &[T]) {
    for item in src {
        if !dst.contains(item) {
            dst.push(item.clone());
        }
    }
}

fn merge_unique<T: PartialEq + Clone>(dst: &mut Option<T>, src: &Option<T>) -> bool {
    if let Some(s) = src {
        if let Some(d) = dst {
            s == d
        } else {
            *dst = src.clone();
            true
        }
    } else {
        true
    }
}

fn merge_props(dst: &mut HashMap<RcStr, Unit>, src: &HashMap<RcStr, Unit>) -> bool {
    for (prop, s) in src {
        if let Some(d) = dst.get_mut(prop) {
            if !merge(d, s) {
                return false;
            }

            continue;
        }

        dst.insert(prop.clone(), s.clone());
    }

    true
}
