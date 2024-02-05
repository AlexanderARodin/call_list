use crate::prelude::*;
use crate::call_item::CallItem;
use crate::utils::get_value_by_path;


//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
pub(crate) struct Parser<'a> {
    tbl: &'a toml::Table,
    pub nest_lvl: i32,
    pub list: Vec<CallItem>,
}

impl Parser<'_> {
    pub fn new<'a>( src_tbl: &'a toml::Table ) -> Parser<'a> {
        Parser {
            tbl: src_tbl,
            nest_lvl: 0,
            list: Vec::new() 
        }
    }

    pub fn start_parsing(&mut self, path: &str ) -> ResultOf< () > {
        self.enter_script_by( path )
    }
}

//  //  //  //  //  //  //  //
//     IMPL 
//  //  //  //  //  //  //  //
use toml::Value;

impl Parser<'_> {

    fn enter_script_by( &mut self, path: &str ) -> ResultOf< () > {
        if self.nest_lvl >= 5 {
            let msg = format!( "<enter_script_by>: nesting of subscripts muast be lower then <{}>", self.nest_lvl);
            return Err(Box::from( msg ));
        }else{
            self.nest_lvl += 1;
        }
        match get_value_by_path( self.tbl, path) {
            Some(toml::Value::Array(val_arr)) => {
                for val in val_arr {
                    self.push_script_value(val)?;
                }
                self.nest_lvl -= 1;
                return Ok(());
            },
            None => {
                let msg = format!( "<from_toml_table>: invalid path to workflow <{path}>" );
                return Err(Box::from( msg ));
            },
            _ => {
                let msg = format!( "<from_toml_table>: workflow must be an array" );
                return Err(Box::from( msg ));
            },
        }
    }

    fn push_script_value( &mut self, value: &Value) -> ResultOf< () > {
        match value {
            Value::String(s) => {
                return self.push_simple_item( s );
            },
            Value::Array(arr) => {
                return self.push_subscripts( arr );
            },
            _ => {
                let msg = format!( "<push_script_value>: unsupported value <{value}>" );
                return Err(Box::from( msg ));
            },
        }
    }

    fn push_subscripts( &mut self, arr: &Vec<Value> ) -> ResultOf< () > {
        for val in arr {
            self.push_subscript_link( val )?;
        }
        Ok(())
    }

    fn push_subscript_link( &mut self, link: &Value ) -> ResultOf< () > {
        match link {
            Value::String(path) => {
                return self.enter_script_by( path );
            },
            _ => {
                let msg = format!( "<push_subscript_link>: unsupported link <{link}>" );
                return Err(Box::from( msg ));
            },
        }
    }


    fn push_simple_item( &mut self, cmd: &str ) -> ResultOf< () > {
        self.list.push(
            CallItem::Simple(cmd.to_string())
        );
        Ok(())
    }
}


