//use crate::prelude::*;

pub fn get_value_by_path<'a>( tbl: &'a toml::Table, path: &'a str ) -> Option< &'a toml::Value > {
    let elems = split_dot_path( path );
    return get_sub_value(tbl, &elems);
 }


pub(super) fn split_dot_path( path: &str ) -> Vec<&str> {
    let s = path.split(".");
    return s.collect();
}

fn get_sub_value<'a>( tbl: &'a toml::Table, path_elems: &[&'a str] ) -> Option< &'a toml::Value > {
    match path_elems.len() {
        0 => None,
        1 => {
            tbl.get(path_elems[0])
        },
        _ => {
            if let Some(toml::Value::Table(sub_tbl)) = tbl.get(path_elems[0]) {
                get_sub_value(sub_tbl, path_elems.get(1..)? )
            }else{
                None
            }
        },
    }
}


//  //  //  //  //  //  //  //
//      TESTs
//  //  //  //  //  //  //  //
#[cfg(test)]
mod utils_path {
    use super::*;

    use toml::Table;
    use raalog::log;

    #[test]
    fn check_path_spliter_77() {
        let tml = r#"
                    [workflows]
                    a.b = 3
                    x = 77
                    "#
                    .parse::<Table>().unwrap();
        let mist;
        match get_value_by_path( &tml, "workflows.x" ) {
            Some(toml::Value::Integer(i)) => {
                if *i == 77 {
                    mist = "";
                }else{
                    mist = "invalid Integer value";
                }
            },
            None => {
                mist = "wrong Value type";
            },
            _ => {
                mist = "must not be a None";
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }

    #[test]
    fn check_path_spliter_3() {
        let tml = r#"
                    [workflows]
                    a.b = 3
                    "#
                    .parse::<Table>().unwrap();
        let mist;
        match get_value_by_path( &tml, "workflows.a.b" ) {
            Some(toml::Value::Integer(i)) => {
                if *i == 3 {
                    mist = "";
                }else{
                    mist = "invalid Integer value";
                }
            },
            None => {
                mist = "wrong Value type";
            },
            _ => {
                mist = "must not be a None";
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }

    #[test]
    fn error_invalid_wf() {
        let tml = r#"
                    #[workflows]
                    a.b = 3
                    "#
                    .parse::<Table>().unwrap();
        let mist;
        match crate::from_toml_table( &tml, "a.b" ) {
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
}

//  //  //  //  //  //  //  //
//      TESTs
//  //  //  //  //  //  //  //
#[cfg(test)]
mod utils_basic {
    use super::*;

    #[test]
    fn check() {
        let path = "path.to.array";
        let validator = vec![
            "path", "to", "array",
        ];
        let val = split_dot_path(path);
        assert_eq!( val, validator, "incorrect path spliting" );
    }
}

