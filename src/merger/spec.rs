use super::*;

use unit::Unit;

macro_rules! test {
    ([$field:ident] $dst:expr, $src:expr => FAILED) => {
        let (_, ok) = test!(@case, $field, $dst, $src);
        let (_, ok_r) = test!(@case, $field, $src, $dst);

        assert!(!ok);
        assert!(!ok_r)
    };
    ([$field:ident] $dst:expr, $src:expr => $res:expr) => {
        let (dst, ok) = test!(@case, $field, $dst, $src);
        let (dst_r, ok_r) = test!(@case, $field, $src, $dst);

        assert!(ok);
        assert!(ok_r);
        assert_eq!(dst.$field, $res);
        assert_eq!(dst_r.$field, $res);
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
