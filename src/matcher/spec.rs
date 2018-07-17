use super::*;

mod subtype {
    use schema::RcMixed;
    use unit::Point;

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
}
