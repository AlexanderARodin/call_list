use std::error::Error;


//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
mod backend;

#[allow(dead_code)]
pub enum CallItem {
    Simple(String),
    WithParam(String, String),
}

pub fn from_toml_value( value: &toml::Value ) -> Result< Vec<CallItem> , Box<dyn Error> > {
    let mut list:Vec<CallItem> = Vec::new();
    backend::push_value( &mut list, value )?;
    Ok( list )
}




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

