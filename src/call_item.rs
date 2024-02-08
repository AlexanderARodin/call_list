

//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //

#[derive(Debug)]
pub enum CallItem {
    Simple( Box<str> ),
    WithNested( Box<str>, Box<CallItem> ),
}
impl CallItem {

    pub fn new( s: &str ) -> Self {
        Self::Simple( s.into() )
    }

    pub fn new_pair( s: &str, nested: CallItem ) -> Self {
        Self::WithNested( s.into(), Box::new(nested) )
    }

    pub fn append(&self, s2: &str) -> Self {
        match &self {
            Self::Simple( a ) => {
                let nested = Self::new( &s2 );
                Self::WithNested(
                    a.clone(),
                    nested.into()
                )
            },
            Self::WithNested( a, b ) => {
                let new_b = b.append(s2);
                Self::WithNested(
                    a.clone(),
                    new_b.into()
                )
            },
        }
    }
}

impl PartialEq for CallItem {
    fn eq( &self, rh: &Self ) -> bool {
        match (self, rh) {
            (CallItem::Simple( lh_a ), CallItem::Simple( rh_a )) => {
                lh_a == rh_a
            },
            (CallItem::WithNested( lh_a, lh_b ), CallItem::WithNested( rh_a, rh_b )) => {
                if lh_a != rh_a {
                    return false;
                }
                return lh_b == rh_b;
            },
            _ => false,
        }
    }
}


//  //  //  //  //  //  //  //
//      TESTs
//  //  //  //  //  //  //  //
#[cfg(test)]
mod call_item_basic {
    use super::*;

    #[test]
    fn check_withparam_simple_not_2() {
        let a = CallItem::new( "111" ).append( "222" );
        let b = CallItem::new( "111" );
        assert_ne!(b,a);
    }
    #[test]
    fn check_withparam_simple_not_1() {
        let a = CallItem::new( "111" ).append( "222" );
        let b = CallItem::new( "111" );
        assert_ne!(a,b);
    }

    #[test]
    fn check_withparam_not_3() {
        let a = CallItem::new( "1_1" ).append( "222" );
        let b = CallItem::new( "111" ).append( "2_2" );
        assert_ne!(a,b);
    }
    #[test]
    fn check_withparam_not_2() {
        let a = CallItem::new( "111" ).append( "222" );
        let b = CallItem::new( "111" ).append( "2_2" );
        assert_ne!(a,b);
    }
    #[test]
    fn check_withparam_not_1() {
        let a = CallItem::new( "1_1" ).append( "222" );
        let b = CallItem::new( "111" ).append( "222" );
        assert_ne!(a,b);
    }

    #[test]
    fn check_withparam_ok() {
        let a = CallItem::new( "111" ).append( "222" );
        let b = CallItem::new( "111" ).append( "222" );
        assert_eq!(a,b);
    }

    #[test]
    fn check_simple_not() {
        let a = CallItem::new( "111" );
        let b = CallItem::new( "1_1" );
        assert_ne!(a,b);
    }
    #[test]
    fn check_simple_ok() {
        let a = CallItem::new( "111" );
        let b = CallItem::new( "111" );
        assert_eq!(a,b);
    }
}

