use std::collections::HashMap;

use schema::RcStr;
use unit::Unit;

pub fn subtype(child: &Unit, parent: &Unit) -> bool {
    check_opt(&child.const_, &parent.const_, |c, p| c == p)
        && check_opt(&child.multiple_of, &parent.multiple_of, |c, p| c % p == 0.)
        && check_opt(&child.maximum, &parent.maximum, |c, p| c.min(*p) == *c)
        && check_opt(&child.minimum, &parent.minimum, |c, p| c.max(*p) == *c)
        && check_opt(&child.max_length, &parent.max_length, PartialOrd::le)
        && check_opt(&child.min_length, &parent.min_length, PartialOrd::ge)
        && child.pattern.is_subset(&parent.pattern)
        && check_opt(
            &child.additional_items,
            &parent.additional_items,
            subtype_box,
        ) && check_opt(&child.items, &parent.items, subtype_box)
        && check_tuple(&child.tuple, &parent.tuple)
        && check_opt(&child.max_items, &parent.max_items, PartialOrd::le)
        && check_opt(&child.min_items, &parent.min_items, PartialOrd::ge)
        && (child.unique_items || !parent.unique_items)
        && check_opt(
            &child.max_properties,
            &parent.max_properties,
            PartialOrd::le,
        )
        && check_opt(
            &child.min_properties,
            &parent.min_properties,
            PartialOrd::ge,
        )
        && child.required.is_subset(&parent.required)
        // TODO: we should check properties and additional_props together.
        && check_opt(
            &child.additional_props,
            &parent.additional_props,
            subtype_box,
        ) && check_opt(&child.property_names, &parent.property_names, subtype_box)
        && check_opt(&child.contains, &parent.contains, subtype_box)
        && check_opt(&child.format, &parent.format, |c, p| c == p)
        && check_props(&child.properties, &parent.properties)
        && check_props(&child.pattern_props, &parent.pattern_props)
        && check_opt(&child.type_, &parent.type_, |c, p| c == p)
}

#[allow(borrowed_box)]
fn subtype_box(child: &Box<Unit>, parent: &Box<Unit>) -> bool {
    subtype(&*child, &*parent)
}

fn check_opt<T>(child: &Option<T>, parent: &Option<T>, chk: impl Fn(&T, &T) -> bool) -> bool {
    match (child, parent) {
        (Some(c), Some(p)) => chk(c, p),
        (None, Some(_)) => false,
        _ => true,
    }
}

fn check_tuple(child: &[Unit], parent: &[Unit]) -> bool {
    if child.len() != parent.len() {
        return false;
    }

    child
        .into_iter()
        .zip(parent.into_iter())
        .all(|(s, p)| subtype(s, p))
}

fn check_props(child: &HashMap<RcStr, Unit>, parent: &HashMap<RcStr, Unit>) -> bool {
    for (prop, p) in parent {
        if let Some(c) = child.get(prop) {
            if !subtype(c, p) {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}
