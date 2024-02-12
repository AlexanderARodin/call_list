
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
    use crate::prelude::*;
    use crate::from_toml_table;

    use toml::Table;
    use raalog::log;

    #[test]
    fn check_path_spliter_77() -> ResultOf< () > {
        let tml = r#"
                    [workflows]
                    a.b = 3
                    x = 77
                    "#
                    .parse::<Table>()?;
        match get_value_by_path( &tml, "workflows.x" ) {
            Some(toml::Value::Integer(i)) => {
                if *i == 77 {
                    return Ok(());
                }else{
                    return Err(Box::from( "invalid Integer value" ));
                }
            },
            None => {
                return Err(Box::from( "must not be a None" ));
            },
            _ => {
                return Err(Box::from( "wrong Value type" ));
            },
        }
    }

    #[test]
    fn check_path_spliter_3() -> ResultOf< () > {
        let tml = r#"
                    [workflows]
                    a.b = 3
                    "#
                    .parse::<Table>()?;
        match get_value_by_path( &tml, "workflows.a.b" ) {
            Some(toml::Value::Integer(i)) => {
                if *i == 3 {
                    return Ok(());
                }else{
                    return Err(Box::from( "invalid Integer value" ));
                }
            },
            None => {
                return Err(Box::from( "incorrect Value type" ));
            },
            _ => {
                return Err(Box::from( "must NOT be a None" ));
            },
        }
    }

    #[test]
    fn error_invalid_wf() -> ResultOf< () > {
        let tml = r#"
                    a.b = 3
                    "#
                    .parse::<Table>()?;
        match from_toml_table( &tml, "a.b" ) {
            Err(e) => {
                log::error(&e.to_string());
                Ok(())
            },
            _ => {
                return Err(Box::from( "has to be Error" ));
            },
        }
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
        assert_eq!( 
            val, 
            validator, 
            "incorrect path spliting" 
        );
    }
}

