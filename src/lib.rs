mod prelude;
use prelude::*;

mod utils;
mod call_item;
use call_item::CallItem;

mod parser;


//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //

pub fn from_toml_table( src_tbl: &toml::Table, path: &str ) -> ResultOf< Vec<CallItem> > {
    let mut p = parser::Parser::new(src_tbl);
    p.start_parsing( path )?;
    return Ok( p.list );
}




//  //  //  //  //  //  //  //
//      TESTs
//  //  //  //  //  //  //  //
#[cfg(test)]
mod new_feature {
    use super::*;

    use toml::Table;
    use raalog::log;


    #[test]
    fn simple_table() {
        let tml = r#"
                    sc2 = [ 
                        { a = 'A', b = 'B' },
                        'branch',
                        { some-thing = 'some-think' },
                    ]
                    "#
                    .parse::<Table>().unwrap();
        let validator = vec![
                CallItem::WithParam("a".to_string(), "A".to_string()),
                CallItem::WithParam("b".to_string(), "B".to_string()),
                CallItem::Simple("branch".to_string()),
                CallItem::WithParam("some-thing".to_string(), "some-think".to_string()),
        ];
        let mist;
        match from_toml_table( &tml, "sc2" ) {
            Err(e) => {
                mist = "must NOT be Errors";
                log::error(&e.to_string());
            },
            Ok(list) => {
                mist = "";
                assert_eq!( list, validator, "list are NOT identical {:?} - {:?}", list, validator );
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }

    #[test]
    fn subscripts_array() {
        let tml = r#"
                    sc2 = [ 'branch' ]
                    [workflows]
                    run0 = [ 'the0', ['workflows.run2'], ]
                    run2 = [ 'the2', ['sc2', 'workflows.run3'], ]
                    run3 = [ 'the3', ['workflows.run4'], ]
                    run4 = [ 'the4', ['workflows.run5'], ]
                    run5 = [ 'the5', 'fin', ]
                    "#
                    .parse::<Table>().unwrap();
        let validator = vec![
                CallItem::Simple("the0".to_string()),
                CallItem::Simple("the2".to_string()),
                CallItem::Simple("branch".to_string()),
                CallItem::Simple("the3".to_string()),
                CallItem::Simple("the4".to_string()),
                CallItem::Simple("the5".to_string()),
                CallItem::Simple("fin".to_string()),
        ];
        let mist;
        match from_toml_table( &tml, "workflows.run0" ) {
            Err(e) => {
                mist = "must NOT be Errors";
                log::error(&e.to_string());
            },
            Ok(list) => {
                mist = "";
                assert_eq!( list, validator, "list are NOT identical {:?} - {:?}", list, validator );
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }
    #[test]
    fn ok_with_nesting_lvl5() {
        let tml = r#"
                    [workflows]
                    run0 = [ 'the0', ['workflows.run1'], ]
                    run1 = [ 
                        'the1', ['workflows.run2'], 
                        #{ another = 'table' },
                        ['workflows.sc2'],
                    ]

                    run2 = [ 'the2', ['workflows.run3'], ]
                    run3 = [ 'the3', ['workflows.run4'], ]
                    run4 = [ 'the4', ['workflows.run5'], ]
                    run5 = [ 'the5', 'fin', ]
                    sc2 = [ 'branch' ]
                    "#
                    .parse::<Table>().unwrap();
        let validator = vec![
                CallItem::Simple("the0".to_string()),
                CallItem::Simple("the1".to_string()),
                CallItem::Simple("the2".to_string()),
                CallItem::Simple("the3".to_string()),
                CallItem::Simple("the4".to_string()),
                CallItem::Simple("the5".to_string()),
                CallItem::Simple("fin".to_string()),
                CallItem::Simple("branch".to_string()),
        ];
        let mist;
        match from_toml_table( &tml, "workflows.run0" ) {
            Err(e) => {
                mist = "must NOT be Errors";
                log::error(&e.to_string());
            },
            Ok(list) => {
                mist = "";
                assert_eq!( list, validator, "list are NOT identical {:?} - {:?}", list, validator );
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }
    #[test]
    fn error_with_nesting_lvl6() {
        let tml = r#"
                    [workflows]
                    run0 = [ 'the', ['workflows.run1'], ]
                    run1 = [ 'the', ['workflows.run2'], ]
                    run2 = [ 'the', ['workflows.run3'], ]
                    run3 = [ 'the', ['workflows.run4'], ]
                    run4 = [ 'the', ['workflows.run5'], ]
                    run5 = [ 'the', ['workflows.run6'], ]
                    run6 = [ 'the', 'fin', ]
                    sc2 = [ ['workflows.script'] ]
                    "#
                    .parse::<Table>().unwrap();
        let mist;
        match from_toml_table( &tml, "workflows.run0" ) {
            Err(e) => {
                mist = "";
                log::error(&e.to_string());
            },
            Ok(_) => {
                mist = "must be Errors";
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }

    #[test]
    fn check_simple() {
        let tml = r#"
                    [workflows]
                    case-1 = { a = 'good', b = 'bad' }
                    script = [ 'the', 'script', ]
                    "#
                    .parse::<Table>().unwrap();
        let validator = vec![
                CallItem::Simple("the".to_string()),
                CallItem::Simple("script".to_string()),
        ];
        let mist;
        match from_toml_table( &tml, "workflows.script" ) {
            Err(e) => {
                mist = "must NOT be Errors";
                log::error(&e.to_string());
            },
            Ok(list) => {
                mist = "";
                assert_eq!( list, validator, "list are NOT identical {:?} - {:?}", list, validator );
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }

    #[test]
    fn error_on_src_tbl() {
        let tml = r#"
                    cmds = { a = 'good', b = 'bad' }
                    "#
                    .parse::<Table>().unwrap();
        let mist;
        match from_toml_table( &tml, "cmds" ) {
            Err(e) => {
                mist = "";
                log::error(&e.to_string());
            },
            Ok(_) => {
                mist = "must be an Error";
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }
}

