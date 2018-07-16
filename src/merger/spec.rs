use super::*;

use unit::Unit;

macro_rules! test {
    ([$field:ident] $dst:expr, $src:expr => FAILED) => {
        let (dst, src) = ($dst, $src);
        let (_, ok_d) = test!(@case, $field, dst.clone(), src.clone());
        let (_, ok_r) = test!(@case, $field, src, dst);

        assert!(!ok_d);
        assert!(!ok_r)
    };
    ([$field:ident] $dst:expr, $src:expr => $res:expr) => {
        let (dst, src, res) = ($dst, $src, $res);
        let (dst_d, ok_d) = test!(@case, $field, dst.clone(), src.clone());
        let (dst_r, ok_r) = test!(@case, $field, src, dst);

        assert!(ok_d);
        assert!(ok_r);
        assert_eq!(dst_d.$field, res);
        assert_eq!(dst_r.$field, res);
    };
    (@case, $field:ident, $dst:expr, $src:expr) => {{
        use super::merge;
        use unit::Unit;

        let mut dst = Unit {
            $field: $dst,
            ..Unit::default()
        };

        let ok = merge(
            &mut dst,
            &Unit {
                $field: $src,
                ..Unit::default()
            },
        );

        (dst, ok)
    }};
}

macro_rules! make_max_tests {
    ($field:ident) => {
        mod $field {
            #[test]
            fn it_should_merge_if_unfilled() {
                test!([$field] None, Some(42) => Some(42));
            }

            #[test]
            fn it_should_select_minimum() {
                test!([$field] Some(32), Some(42) => Some(32));
            }
        }
    };
}

macro_rules! make_min_tests {
    ($field:ident) => {
        mod $field {
            #[test]
            fn it_should_merge_if_unfilled() {
                test!([$field] None, Some(42) => Some(42));
            }

            #[test]
            fn it_should_select_maximum() {
                test!([$field] Some(32), Some(42) => Some(42));
            }
        }
    };
}

macro_rules! make_nested_tests {
    ($field:ident) => {
        mod $field {
            use unit::Unit;

            #[test]
            fn it_should_merge_if_unfilled() {
                let (a, b) = (Unit::default(), Unit::default());
                test!([$field] None, Some(Box::new(a)) => Some(Box::new(b)));
            }

            #[test]
            fn it_should_merge_nested_unit() {
                let a = Unit {
                    max_items: Some(42),
                    ..Unit::default()
                };

                let b = Unit {
                    min_items: Some(32),
                    ..Unit::default()
                };

                let r = Unit {
                    max_items: Some(42),
                    min_items: Some(32),
                    ..Unit::default()
                };

                test!([$field] Some(Box::new(a)), Some(Box::new(b)) => Some(Box::new(r)));
            }

            #[test]
            fn it_should_fail_if_cannot_merge() {
                use schema::Type;

                let a = Unit {
                    type_: Some(Type::Integer),
                    ..Unit::default()
                };

                let b = Unit {
                    type_: Some(Type::Array),
                    ..Unit::default()
                };

                test!([$field] Some(Box::new(a)), Some(Box::new(b)) => FAILED);
            }
        }
    };
}

#[test]
fn it_should_merge_if_nones() {
    let mut dst = Unit::default();

    assert!(merge(&mut dst, &Unit::default()));
    assert_eq!(dst, Unit::default());
}

make_max_tests!(max_length);
make_min_tests!(min_length);
make_max_tests!(max_items);
make_min_tests!(min_items);
make_max_tests!(max_properties);
make_min_tests!(min_properties);

make_nested_tests!(items);
make_nested_tests!(additional_items);
make_nested_tests!(additional_props);
make_nested_tests!(property_names);
make_nested_tests!(contains);

mod const_ {
    use schema::RcMixed;

    #[test]
    fn it_should_merge_if_unfilled() {
        test!([const_] None, Some(RcMixed::from(42)) => Some(RcMixed::from(42)));
    }

    #[test]
    fn it_should_merge_if_equal() {
        test!([const_] Some(RcMixed::from(42)), Some(RcMixed::from(42)) => Some(RcMixed::from(42)));
    }

    #[test]
    fn it_should_fail_if_different() {
        test!([const_] Some(RcMixed::from(42)), Some(RcMixed::from(40)) => FAILED);
    }
}

mod type_ {
    use schema::Type;

    #[test]
    fn it_should_merge_if_unfilled() {
        test!([type_] None, Some(Type::Integer) => Some(Type::Integer));
    }

    #[test]
    fn it_should_merge_if_equal() {
        test!([type_] Some(Type::Integer), Some(Type::Integer) => Some(Type::Integer));
    }

    #[test]
    fn it_should_merge_integer_and_number() {
        test!([type_] Some(Type::Integer), Some(Type::Number) => Some(Type::Integer));
    }

    #[test]
    fn it_should_fail_if_different() {
        test!([type_] Some(Type::Integer), Some(Type::String) => FAILED);
    }
}

mod format {
    use schema::RcStr;

    #[test]
    fn it_should_merge_if_unfilled() {
        test!([format] None, Some(RcStr::from("foo")) => Some(RcStr::from("foo")));
    }

    #[test]
    fn it_should_merge_if_equal() {
        test!(
            [format] Some(RcStr::from("foo")), Some(RcStr::from("foo")) => Some(RcStr::from("foo"))
        );
    }

    #[test]
    fn it_should_fail_if_different() {
        test!([format] Some(RcStr::from("foo")), Some(RcStr::from("bar")) => FAILED);
    }
}

mod maximum {
    use unit::Point;

    #[test]
    fn it_should_merge_if_unfilled() {
        test!([maximum] None, Some(Point::inc(42.)) => Some(Point::inc(42.)));
    }

    #[test]
    fn it_should_merge_if_equal() {
        test!([maximum] Some(Point::inc(42.)), Some(Point::inc(42.)) => Some(Point::inc(42.)));
    }

    #[test]
    fn it_should_remain_minimum() {
        test!([maximum] Some(Point::inc(42.)), Some(Point::inc(32.)) => Some(Point::inc(32.)));
    }

    #[test]
    fn it_should_prefer_exclusive() {
        test!([maximum] Some(Point::inc(42.)), Some(Point::exc(42.)) => Some(Point::exc(42.)));
    }
}

mod minimum {
    use unit::Point;

    #[test]
    fn it_should_merge_if_unfilled() {
        test!([minimum] None, Some(Point::inc(42.)) => Some(Point::inc(42.)));
    }

    #[test]
    fn it_should_merge_if_equal() {
        test!([minimum] Some(Point::inc(42.)), Some(Point::inc(42.)) => Some(Point::inc(42.)));
    }

    #[test]
    fn it_should_remain_maximum() {
        test!([minimum] Some(Point::inc(42.)), Some(Point::inc(32.)) => Some(Point::inc(42.)));
    }

    #[test]
    fn it_should_prefer_exclusive() {
        test!([minimum] Some(Point::inc(42.)), Some(Point::exc(42.)) => Some(Point::exc(42.)));
    }
}

mod unique_items {
    #[test]
    fn it_should_remain_false() {
        test!([unique_items] false, false => false);
    }

    #[test]
    fn it_should_select_true() {
        test!([unique_items] false, true => true);
    }
}

mod tuple {
    use unit::Unit;

    #[test]
    fn it_should_merge_if_unfilled() {
        test!([tuple] vec![], vec![Unit::default()] => vec![Unit::default()]);
    }

    #[test]
    fn it_should_fail_if_different_shape() {
        test!([tuple] vec![Unit::default(), Unit::default()], vec![Unit::default()] => FAILED);
    }

    #[test]
    fn it_should_merge_appropriate_items() {
        let a = Unit {
            max_items: Some(42),
            ..Unit::default()
        };

        let b = Unit {
            min_items: Some(32),
            ..Unit::default()
        };

        let r = Unit {
            max_items: Some(42),
            min_items: Some(32),
            ..Unit::default()
        };

        test!([tuple] vec![a.clone(), a.clone()], vec![Unit::default(), b] => vec![a, r]);
    }

    #[test]
    fn it_should_fail_if_cannot_merge_items() {
        use schema::Type;

        let a = Unit {
            type_: Some(Type::Integer),
            ..Unit::default()
        };

        let b = Unit {
            type_: Some(Type::Array),
            ..Unit::default()
        };

        test!([tuple] vec![a.clone(), a.clone()], vec![Unit::default(), b] => FAILED);
    }
}
