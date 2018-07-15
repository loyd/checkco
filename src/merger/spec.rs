use super::*;

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

#[test]
fn it_should_merge_if_nones() {
    let mut dst = Unit::default();

    assert!(merge(&mut dst, &Unit::default()));
    assert_eq!(dst, Unit::default());
}

mod const_ {
    use super::*;

    use schema::RcMixed;

    #[test]
    fn it_should_merge_if_unfilled() {
        test!(
            [const_] None, Some(RcMixed::from(42)) => Some(RcMixed::from(42))
        );
    }

    #[test]
    fn it_should_merge_if_equal() {
        test!(
            [const_] Some(RcMixed::from(42)), Some(RcMixed::from(42)) => Some(RcMixed::from(42))
        );
    }

    #[test]
    fn it_should_fail_if_different() {
        test!(
            [const_] Some(RcMixed::from(42)), Some(RcMixed::from(40)) => FAILED
        );
    }
}

mod type_ {
    use super::*;

    use schema::Type;

    #[test]
    fn it_should_merge_if_unfilled() {
        test!(
            [type_] None, Some(Type::Integer) => Some(Type::Integer)
        );
    }

    #[test]
    fn it_should_merge_if_equal() {
        test!(
            [type_] Some(Type::Integer), Some(Type::Integer) => Some(Type::Integer)
        );
    }

    #[test]
    fn it_should_fail_if_different() {
        test!(
            [type_] Some(Type::Integer), Some(Type::String) => FAILED
        );
    }
}

mod format {
    use super::*;

    use schema::RcStr;

    #[test]
    fn it_should_merge_if_unfilled() {
        test!(
            [format] None, Some(RcStr::from("foo")) => Some(RcStr::from("foo"))
        );
    }

    #[test]
    fn it_should_merge_if_equal() {
        test!(
            [format] Some(RcStr::from("foo")), Some(RcStr::from("foo")) => Some(RcStr::from("foo"))
        );
    }

    #[test]
    fn it_should_fail_if_different() {
        test!(
            [format] Some(RcStr::from("foo")), Some(RcStr::from("bar")) => FAILED
        );
    }
}

mod maximum {
    use super::*;

    use unit::{Point, Unit};

    #[test]
    fn it_should_merge_if_unfilled() {
        test!(
            [maximum] None, Some(Point::inc(42.)) => Some(Point::inc(42.))
        );
    }

    #[test]
    fn it_should_merge_if_equal() {
        test!(
            [maximum] Some(Point::inc(42.)), Some(Point::inc(42.)) => Some(Point::inc(42.))
        );
    }

    #[test]
    fn it_should_remain_minimum() {
        test!(
            [maximum] Some(Point::inc(42.)), Some(Point::inc(32.)) => Some(Point::inc(32.))
        );
    }

    #[test]
    fn it_should_prefer_exclusive() {
        test!(
            [maximum] Some(Point::inc(42.)), Some(Point::exc(42.)) => Some(Point::exc(42.))
        );
    }
}

mod minimum {
    use super::*;

    use unit::{Point, Unit};

    #[test]
    fn it_should_merge_if_unfilled() {
        test!(
            [minimum] None, Some(Point::inc(42.)) => Some(Point::inc(42.))
        );
    }

    #[test]
    fn it_should_merge_if_equal() {
        test!(
            [minimum] Some(Point::inc(42.)), Some(Point::inc(42.)) => Some(Point::inc(42.))
        );
    }

    #[test]
    fn it_should_remain_maximum() {
        test!(
            [minimum] Some(Point::inc(42.)), Some(Point::inc(32.)) => Some(Point::inc(42.))
        );
    }

    #[test]
    fn it_should_prefer_exclusive() {
        test!(
            [minimum] Some(Point::inc(42.)), Some(Point::exc(42.)) => Some(Point::exc(42.))
        );
    }
}
