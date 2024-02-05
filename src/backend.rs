use toml::{ Table, Value };

use crate::prelude::*;
use crate::utils;


//  //  //  //  //  //  //  //
//      backend
//  //  //  //  //  //  //  //
use super::CallItem;


pub fn push_sub_script( list: &mut Vec<CallItem>, src_tbl: &toml::Table, path: &str ) -> ResultOf< () > {
    match utils::get_value_by_path( src_tbl, path) {
        Some(toml::Value::Array(val_arr)) => {
            for val in val_arr {
                push_script_value(list, src_tbl, val)?;
            }
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

fn push_script_value( list: &mut Vec<CallItem>, src_tbl: &toml::Table, value: &Value) -> ResultOf< () > {
    match value {
        Value::String(s) => {
            return push_simple_item( list, s );
        },
        Value::Array(arr) => {
            return push_subscripts( list, src_tbl, arr );
        },
        _ => {
            let msg = format!( "<push_script_value>: unsupported value <{value}>" );
            return Err(Box::from( msg ));
        },
    }
}

fn push_subscripts( list: &mut Vec<CallItem>, src_tbl: &toml::Table, arr: &Vec<Value> ) -> ResultOf< () > {
    for val in arr {
        push_subscript_link( list, src_tbl, val )?;
    }
    Ok(())
}

fn push_subscript_link( list: &mut Vec<CallItem>, src_tbl: &toml::Table, link: &Value ) -> ResultOf< () > {
    match link {
        Value::String(path) => {
            return push_sub_script(list, src_tbl, path);
        },
        _ => {
            let msg = format!( "<push_subscript_link>: unsupported link <{link}>" );
            return Err(Box::from( msg ));
        },
    }
}









pub fn push_value( list: &mut Vec<CallItem>, value: &Value ) -> ResultOf< () > {
    match value {
        Value::String(cmd) => {
            push_simple_item( list, cmd )?;
        },
        Value::Array(arr) => {
            push_command_array( list, arr )?;
        },
        Value::Table(tbl) => {
            push_command_table( list, tbl )?;
        },
        _ => {
            return Err( Box::from( "<values_to_call_list>: unsupported command" ) );
        },
    }
    Ok(())
}

//  //  //  //  //  //  //  //
fn push_command_array( list: &mut Vec<CallItem>, arr: &Vec<Value> ) -> ResultOf< () > {
    for value in arr {
        push_value( list, value )?;
    }
    Ok(())
}

fn push_command_table( list: &mut Vec<CallItem>, tbl: &Table ) -> ResultOf< () > {
    for (key, value) in tbl {
        push_key_value( list, key, value )?;
    }
    Ok(())
}

//  //  //  //  //  //  //  //
fn push_key_value( list: &mut Vec<CallItem>, key: &str, value: &Value ) -> ResultOf< () > {
    match value {
        Value::String(param) => {
            list.push(
                CallItem::WithParam( key.to_string(), param.to_string() )
            );
        },
        _ => {
            return Err( Box::from( "<push_key_value>: unsupported argument" ) );
        },
    }
    Ok(())
}
fn push_simple_item( list: &mut Vec<CallItem>, cmd: &str ) -> ResultOf< () > {
    list.push(
            CallItem::Simple(cmd.to_string())
        );
    Ok(())
}

