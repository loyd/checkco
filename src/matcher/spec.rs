use super::*;

mod subtype {
    use schema::RcMixed;

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
}
