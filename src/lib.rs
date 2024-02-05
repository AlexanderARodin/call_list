mod prelude;
use prelude::*;

//mod backend;
mod utils;
mod call_item;
use call_item::CallItem;

mod parser;


//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //

//pub fn from_toml_value( value: &toml::Value ) -> ResultOf< Vec<CallItem> > {
//    let mut list:Vec<CallItem> = Vec::new();
//    backend::push_value( &mut list, value )?;
//    Ok( list )
//}


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
    fn check_sub_script() {
        let tml = r#"
                    run2 = [ 'fin', ['workflows.sc2'], ]
                    [workflows]
                    script = [ 'the', ['run2'], ]
                    sc2 = [ ['workflows.script'] ]
                    "#
                    .parse::<Table>().unwrap();
        let validator = vec![
                CallItem::Simple("the".to_string()),
                CallItem::Simple("fin".to_string()),
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
            Ok(list) => {
                mist = "must be an Error";
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }
}
/*
//  //  //  //  //  //  //  //
//      TESTs
//  //  //  //  //  //  //  //
#[cfg(test)]
mod values_to_call_list {
    use super::*;

    use toml::Table;
    use raalog::log;


    #[test]
    fn single_command_with_param() {
        let tml = r#"
                    cmds = { a = 'good', b = 'bad' }
                    "#
                    .parse::<Table>().unwrap();
        let validator = vec![
            "a(good)", 
            "b(bad)",
        ];
        let mist;
        let val = tml.get("cmds").unwrap();
        match from_toml_value( val ) {
            Err(e) => {
                mist = "has NOT to be Error";
                log::error(&e.to_string());
            },
            Ok(list) => {
                check_call_list( list, validator );
                mist = "";
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }

    #[test]
    fn array_command() {
        let tml = r#"
                    cmds = [
                        'test', 'another',
                        ['alpha', 'betta'],
                    ]
                    "#
                    .parse::<Table>().unwrap();
        let validator = vec![
            "test", "another","alpha","betta",
        ];
        let mist;
        let val = tml.get("cmds").unwrap();
        match from_toml_value( val ) {
            Err(e) => {
                mist = "has NOT to be Error";
                log::error(&e.to_string());
            },
            Ok(list) => {
                check_call_list( list, validator );
                mist = "";
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }

    #[test]
    fn single_command() {
        let tml = r#"
                    cmds = 'test'
                    "#
                    .parse::<Table>().unwrap();
        let validator = vec![
            "test",
        ];
        let mist;
        let val = tml.get("cmds").unwrap();
        match from_toml_value( val ) {
            Err(e) => {
                mist = "has NOT to be Error";
                log::error(&e.to_string());
            },
            Ok(list) => {
                check_call_list( list, validator );
                mist = "";
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }

    //  //  //  //  //  //  //
    #[test]
    fn unsupported() {
        let tml = r#"
                    cmds = 3
                    "#
                    .parse::<Table>().unwrap();
        let mist;
        let val = tml.get("cmds").unwrap();
        match from_toml_value( val ) {
            Err(e) => {
                mist = "";
                log::error(&e.to_string());
            },
            _ => {
                mist = "has to be Error";
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }

    //  //  //  //  //  //  //
    fn check_call_list( list: Vec<CallItem>, validator: Vec<&str> ) {
        if list.len() != validator.len() {
            println!("#> {} <--> {}", list.len(), validator.len() );
            assert!(false, "element number incorrect");
        }
        let mut vali_index = 0;
        for item in list {
            match item {
                CallItem::Simple(cmd) => {
                    let valid = validator[vali_index];
                    println!("#> {} <--> {}", cmd, valid );
                    if cmd != valid {
                        assert!(false, "not valid command");
                    }
                },
                CallItem::WithParam(cmd, param ) => {
                    let valid = validator[vali_index];
                    let checked = format!( "{}({})", cmd, param );
                    println!("#> {} <--> {}", checked, valid);
                    if checked != valid {
                        assert!(false, "not valid command");
                    }
                },
            }
            vali_index += 1;
        }
    }
}
*/
