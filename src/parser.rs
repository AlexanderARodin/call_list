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
        if self.nest_lvl > 5 {
            let msg = format!( "<enter_script_by>: allowed max subscripts nesting is <{}>", self.nest_lvl-1 );
            return Err(Box::from( msg ));
        }else{
            self.nest_lvl += 1;
        }
        match get_value_by_path( self.tbl, path) {
            Some(toml::Value::Array(val_arr)) => {
                for val in val_arr {
                    self.process_one_script_item(val)?;
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

    fn process_one_script_item( &mut self, value: &Value) -> ResultOf< () > {
        match value {
            Value::String(s) => {
                return self.push_simple_item( s );
            },
            Value::Table(tbl) => {
                return self.iterate_table( tbl );
            },
            Value::Array(arr) => {
                return self.iterate_array( arr );
            },
            _ => {
                let msg = format!( "<push_script_value>: unsupported value <{value}>" );
                return Err(Box::from( msg ));
            },
        }
    }

    fn iterate_array( &mut self, arr: &Vec<Value> ) -> ResultOf< () > {
        for val in arr {
            self.follow_subscript_link( val )?;
        }
        Ok(())
    }

    fn follow_subscript_link( &mut self, link: &Value ) -> ResultOf< () > {
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

    fn iterate_table( &mut self, tbl: &toml::Table ) -> ResultOf< () > {
        for (key, value) in tbl {
            self.push_table_item( key, value )?;
            //self.process_one_pair( key, value )?;
        }
        Ok(())
    }
    fn process_one_pair( &mut self, key: &str, value: &Value ) -> ResultOf< () > {
        match value {
            Value::String(param) => {
                return self.push_with_param( key, param );
            },
            _ => {
                let msg = format!( "<process_one_pair>: unsupported pair <{}:{}>", key, value );
                return Err(Box::from( msg ));
            },
        }
    }

    //  //  //  //  //  //  //
    fn push_table_item( &mut self, key: &str, value: &Value ) -> ResultOf< () > {
        let item: CallItem = construct_table_callitem( key, value )?;
        self.list.push(
            item
        );
        Ok(())
    }
    fn push_with_param( &mut self, key: &str, param: &str ) -> ResultOf< () > {
        self.list.push(
            CallItem::new( key )
                        .append(param)
        );
        Ok(())
    }
    fn push_simple_item( &mut self, cmd: &str ) -> ResultOf< () > {
        self.list.push(
            CallItem::new(cmd)
        );
        Ok(())
    }
}
    //  //  //  //  //  //  //
fn construct_table_callitem( key: &str, value: &Value ) -> ResultOf< CallItem > {
    match value {
        Value::String( s ) => {
            return Ok( CallItem::new( key ).append( s ) );
        },
        Value::Table( sub_tbl ) => {
            if sub_tbl.len() > 1 {
                let msg = format!( "<construct_table_callitem>: subtable must have single item" );
                return Err(Box::from( msg ));
            }
            for (sub_key,sub_value) in sub_tbl.iter() {
                return Ok( CallItem::new_pair( key,
                                           construct_table_callitem(sub_key, sub_value)?
                                           ) );
            }
            let msg = format!( "<construct_table_callitem>: block must be unreachable" );
            return Err(Box::from( msg ));
        },
        _ => {
            let msg = format!( "<construct_table_callitem>: unsupported table item <{}>", value );
            return Err(Box::from( msg ));
        },
    }
}

