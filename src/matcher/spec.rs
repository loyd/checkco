use super::*;

mod subtype {
    use std::collections::HashMap;

    use schema::{RcMixed, RcStr, Type};
    use unit::{Point, Unit};

    macro_rules! test {
        ([$field:ident] $child:expr, $parent:expr => $res:expr) => {{
            use super::subtype;
            use unit::Unit;

            let child = Unit {
                $field: $child,
                ..Unit::default()
            };

            let parent = Unit {
                $field: $parent,
                ..Unit::default()
            };

            assert_eq!(subtype(&child, &parent), $res);
        }};
    }

    macro_rules! test_max {
        ($field:ident) => {
            test!([$field] None, Some(42) => false);
            test!([$field] Some(42), None => true);
            test!([$field] Some(42), Some(43) => true);
            test!([$field] Some(42), Some(41) => false);
        };
    }

    macro_rules! test_min {
        ($field:ident) => {
            test!([$field] None, Some(42) => false);
            test!([$field] Some(42), None => true);
            test!([$field] Some(42), Some(43) => false);
            test!([$field] Some(42), Some(41) => true);
        };
    }

    macro_rules! test_nested {
        ($field:ident) => {
            let a = Box::new(Unit {
                required: vec![RcStr::from("a"), RcStr::from("b")].into_iter().collect(),
                ..Unit::default()
            });

            let b = Box::new(Unit {
                required: vec![RcStr::from("a")].into_iter().collect(),
                ..Unit::default()
            });

            let c = Box::new(Unit {
                required: vec![RcStr::from("c")].into_iter().collect(),
                ..Unit::default()
            });

            test!([$field] None, Some(a.clone()) => false);
            test!([$field] Some(a.clone()), None => true);
            test!([$field] Some(a.clone()), Some(c) => false);
            test!([$field] Some(a.clone()), Some(b) => true);
        };
    }

    macro_rules! test_props {
        ($field:ident) => {
            let a = Unit {
                required: vec![RcStr::from("a"), RcStr::from("b")].into_iter().collect(),
                ..Unit::default()
            };

            let mut ha = HashMap::new();
            ha.insert(RcStr::from("a"), a);

            let b = Unit {
                required: vec![RcStr::from("a")].into_iter().collect(),
                ..Unit::default()
            };

            let mut hb = HashMap::new();
            hb.insert(RcStr::from("a"), b);

            let c = Unit {
                required: vec![RcStr::from("c")].into_iter().collect(),
                ..Unit::default()
            };

            let mut hc = HashMap::new();
            hc.insert(RcStr::from("a"), c);

            test!([$field] HashMap::new(), ha.clone() => false);
            test!([$field] ha.clone(), HashMap::new() => true);
            test!([$field] ha.clone(), hc => false);
            test!([$field] ha.clone(), hb => true);
        };
    }

    #[test]
    fn it_should_check_const() {
        test!([const_] None, Some(RcMixed::from(42)) => false);
        test!([const_] Some(RcMixed::from(42)), None => true);
        test!([const_] Some(RcMixed::from(42)), Some(RcMixed::from(42)) => true);
    }

    #[test]
    fn it_should_check_multiple_of() {
        test!([multiple_of] None, Some(42.) => false);
        test!([multiple_of] Some(42.), None => true);
        test!([multiple_of] Some(42.), Some(42.) => true);
        test!([multiple_of] Some(84.), Some(42.) => true);
        test!([multiple_of] Some(83.), Some(42.) => false);
    }

    #[test]
    fn it_should_check_maximum() {
        test!([maximum] None, Some(Point::exc(42.)) => false);
        test!([maximum] Some(Point::exc(42.)), None => true);
        test!([maximum] Some(Point::exc(42.)), Some(Point::exc(42.)) => true);
        test!([maximum] Some(Point::inc(42.)), Some(Point::inc(42.)) => true);
        test!([maximum] Some(Point::exc(42.)), Some(Point::inc(42.)) => true);
        test!([maximum] Some(Point::inc(42.)), Some(Point::exc(42.)) => false);
        test!([maximum] Some(Point::inc(42.)), Some(Point::exc(43.)) => true);
        test!([maximum] Some(Point::inc(42.)), Some(Point::inc(43.)) => true);
        test!([maximum] Some(Point::inc(43.)), Some(Point::inc(42.)) => false);
    }

    #[test]
    fn it_should_check_minimum() {
        test!([minimum] None, Some(Point::exc(42.)) => false);
        test!([minimum] Some(Point::exc(42.)), None => true);
        test!([minimum] Some(Point::exc(42.)), Some(Point::exc(42.)) => true);
        test!([minimum] Some(Point::inc(42.)), Some(Point::inc(42.)) => true);
        test!([minimum] Some(Point::exc(42.)), Some(Point::inc(42.)) => true);
        test!([minimum] Some(Point::inc(42.)), Some(Point::exc(42.)) => false);
        test!([minimum] Some(Point::inc(42.)), Some(Point::exc(43.)) => false);
        test!([minimum] Some(Point::inc(42.)), Some(Point::inc(43.)) => false);
        test!([minimum] Some(Point::inc(43.)), Some(Point::inc(42.)) => true);
    }

    #[test]
    fn it_should_check_max_length() {
        test_max!(max_length);
    }

    #[test]
    fn it_should_check_min_length() {
        test_min!(min_length);
    }

    #[test]
    fn it_should_check_max_items() {
        test_max!(max_items);
    }

    #[test]
    fn it_should_check_min_items() {
        test_min!(min_items);
    }

    #[test]
    fn it_should_check_max_properties() {
        test_max!(max_properties);
    }

    #[test]
    fn it_should_check_min_properties() {
        test_min!(min_properties);
    }

    #[test]
    fn it_should_check_pattern() {
        test!([pattern]
              vec![RcStr::from("a")].into_iter().collect(),
              vec![RcStr::from("a")].into_iter().collect() => true);

        test!([pattern]
              vec![RcStr::from("a")].into_iter().collect(),
              vec![RcStr::from("b")].into_iter().collect() => false);

        test!([pattern]
              vec![RcStr::from("a"), RcStr::from("b")].into_iter().collect(),
              vec![RcStr::from("b")].into_iter().collect() => true);
    }

    #[test]
    fn it_should_check_required() {
        test!([required]
              vec![RcStr::from("a")].into_iter().collect(),
              vec![RcStr::from("a")].into_iter().collect() => true);

        test!([required]
              vec![RcStr::from("a")].into_iter().collect(),
              vec![RcStr::from("b")].into_iter().collect() => false);

        test!([required]
              vec![RcStr::from("a"), RcStr::from("b")].into_iter().collect(),
              vec![RcStr::from("b")].into_iter().collect() => true);
    }

    #[test]
    fn it_should_check_additional_items() {
        test_nested!(additional_items);
    }

    #[test]
    fn it_should_check_items() {
        test_nested!(items);
    }

    #[test]
    fn it_should_check_additional_props() {
        test_nested!(additional_props);
    }

    #[test]
    fn it_should_check_contains() {
        test_nested!(contains);
    }

    #[test]
    fn it_should_check_property_names() {
        test_nested!(property_names);
    }

    #[test]
    fn it_should_check_properties() {
        test_props!(properties);
    }

    #[test]
    fn it_should_check_pattern_props() {
        test_props!(pattern_props);
    }

    #[test]
    fn it_should_check_type() {
        test!([type_] None, Some(Type::Integer) => false);
        test!([type_] Some(Type::Integer), None => true);
        test!([type_] Some(Type::Integer), Some(Type::String) => false);
        test!([type_] Some(Type::Integer), Some(Type::Number) => true);
    }

    #[test]
    fn it_should_check_tuple() {
        let a = Unit {
            required: vec![RcStr::from("a"), RcStr::from("b")]
                .into_iter()
                .collect(),
            ..Unit::default()
        };

        let b = Unit {
            required: vec![RcStr::from("a")].into_iter().collect(),
            ..Unit::default()
        };

        let c = Unit {
            required: vec![RcStr::from("c")].into_iter().collect(),
            ..Unit::default()
        };

        test!([tuple] vec![a.clone()], vec![a.clone(), b.clone()] => false);
        test!([tuple] vec![a.clone()], vec![b.clone()] => true);
        test!([tuple] vec![a.clone()], vec![c.clone()] => false);
    }
}
