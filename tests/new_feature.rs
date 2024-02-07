

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

//  //  //  //  //  //  //  //
//      TESTs
//  //  //  //  //  //  //  //

#[test]
fn NestedItems() {
    let t = CallItem
                ::new( "a" )
                .append( "b" )
                .append( "c" )
                .append( "d" );
    pr( 0, &t );
}

fn pr( n: i32, item: &CallItem ) {
    match item {
        CallItem::Item( a, None ) => {
            println!( "{} --> ({}, None)", n, a );
        }
        CallItem::Item( a, Some(b) ) => {
            println!( "{} --> ({}, -- )", n, a );
            pr( n+1, &b );
        }
    }
}

