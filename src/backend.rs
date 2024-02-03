use std::error::Error;
use toml::{ Table, Value };


//  //  //  //  //  //  //  //
//      backend
//  //  //  //  //  //  //  //
use super::CallItem;


pub fn push_value( list: &mut Vec<CallItem>, value: &Value ) -> Result< (), Box<dyn Error> > {
    match value {
        Value::String(cmd) => {
            push_simple_command( list, cmd )?;
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
fn push_command_array( list: &mut Vec<CallItem>, arr: &Vec<Value> ) -> Result< (), Box<dyn Error> > {
    for value in arr {
        push_value( list, value )?;
    }
    Ok(())
}

fn push_command_table( list: &mut Vec<CallItem>, tbl: &Table ) -> Result< (), Box<dyn Error> > {
    for (key, value) in tbl {
        push_key_value( list, key, value )?;
    }
    Ok(())
}

//  //  //  //  //  //  //  //
fn push_key_value( list: &mut Vec<CallItem>, key: &str, value: &Value ) -> Result< (), Box<dyn Error> > {
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
fn push_simple_command( list: &mut Vec<CallItem>, cmd: &str ) -> Result< (), Box<dyn Error> > {
    list.push(
            CallItem::Simple(cmd.to_string())
        );
    Ok(())
}

