mod prelude;
use prelude::*;

mod utils;
pub use utils::get_value_by_path as get_value_by_path;

mod call_item;
pub use call_item::CallItem as CallItem;


mod parser;


//  //  //  //  //  //  //  //
//      API
//  //  //  //  //  //  //  //

pub fn from_toml_table( src_tbl: &toml::Table, path: &str ) -> ResultOf< Vec<CallItem> > {
    let mut p = parser::Parser::new(src_tbl);
    p.start_parsing( path )?;
    return Ok( p.list );
}

