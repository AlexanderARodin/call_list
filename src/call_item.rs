

//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //

#[allow(dead_code)]
#[derive(Debug)]
pub enum CallItem {
    Item( Box<str>, Option< Box<CallItem> > ),
}
impl CallItem {
    pub fn new( s: &str ) -> Self {
        Self::Item( s.into(), None )
    }
    pub fn append(&self, s2: &str) -> Self {
        match &self {
            Self::Item( a, None ) => {
                Self::Item(
                    a.clone(),
                    Some( Self::new( &s2 ).into() )
                )
            },
            Self::Item( a, Some(b) ) => {
                let new_b = b.append(s2);
                Self::Item(
                    a.clone(),
                    Some( new_b.into() )
                )
            },
        }
    }
}

impl PartialEq for CallItem {
    fn eq( &self, rh: &Self ) -> bool {
        match (self, rh) {
            (CallItem::Item( lh_a, None), CallItem::Item( rh_a, None )) => {
                lh_a == rh_a
            },
            (CallItem::Item( lh_a, Some(lh_b) ), CallItem::Item( rh_a, Some( rh_b) )) => {
                if lh_a != rh_a {
                    return false;
                }
                return lh_b == rh_b;
            },
            _ => false,
        }
    }
}

/*
#[allow(dead_code)]
#[derive(Debug)]
pub enum CallItem {
    Simple(String),
    WithParam(String, String),
}

impl PartialEq for CallItem {
    fn eq( &self, rh: &Self ) -> bool {
        match (self, rh) {
            (CallItem::Simple(lh_s), CallItem::Simple(rh_s)) => {
                lh_s == rh_s
            },
            (CallItem::WithParam(lh_a, lh_b), CallItem::WithParam(rh_a,rh_b)) => {
                (lh_a == rh_a) && (lh_b == rh_b)
            },
            _ => false,
        }
    }
}
*/

//  //  //  //  //  //  //  //
//      TESTs
//  //  //  //  //  //  //  //
#[cfg(test)]
mod call_item_basic {
    use super::*;

    #[test]
    fn check_withparam_simple_not_2() {
        let a = CallItem::WithParam( "111".to_string(), "222".to_string() );
        let b = CallItem::Simple( "111".to_string() );
        assert_ne!(b,a);
    }
    #[test]
    fn check_withparam_simple_not_1() {
        let a = CallItem::WithParam( "111".to_string(), "222".to_string() );
        let b = CallItem::Simple( "111".to_string() );
        assert_ne!(a,b);
    }

    #[test]
    fn check_withparam_not_3() {
        let a = CallItem::WithParam( "1_1".to_string(), "222".to_string() );
        let b = CallItem::WithParam( "111".to_string(), "2_2".to_string() );
        assert_ne!(a,b);
    }
    #[test]
    fn check_withparam_not_2() {
        let a = CallItem::WithParam( "111".to_string(), "222".to_string() );
        let b = CallItem::WithParam( "111".to_string(), "2_2".to_string() );
        assert_ne!(a,b);
    }
    #[test]
    fn check_withparam_not_1() {
        let a = CallItem::WithParam( "1_1".to_string(), "222".to_string() );
        let b = CallItem::WithParam( "111".to_string(), "222".to_string() );
        assert_ne!(a,b);
    }

    #[test]
    fn check_withparam_ok() {
        let a = CallItem::WithParam( "111".to_string(), "222".to_string() );
        let b = CallItem::WithParam( "111".to_string(), "222".to_string() );
        assert_eq!(a,b);
    }

    #[test]
    fn check_simple_not() {
        let a = CallItem::Simple( "111".to_string() );
        let b = CallItem::Simple( "1_1".to_string() );
        assert_ne!(a,b);
    }
    #[test]
    fn check_simple_ok() {
        let a = CallItem::Simple( "111".to_string() );
        let b = CallItem::Simple( "111".to_string() );
        assert_eq!(a,b);
    }
}

