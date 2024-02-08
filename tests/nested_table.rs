

use toml::Table;
use raalog::log;


//  //  //  //  //  //  //  //
//      TESTs
//  //  //  //  //  //  //  //
use call_list::{ from_toml_table, CallItem };

#[test]
fn nested_tables() {
    let tml = r#"
                [workflows]
                sc2 = [ 
                    { a = 'A', b = 'B' },
                    'branch',
                    { some-thing = 'some-think' },
                    'beforing',
                    { root = { lvl2 = 'lvl3' } },
                    { root = { lvl2 = { lvl3 = 'lvl4' } } },
                ]
                "#
                .parse::<Table>().unwrap();
    let validator = vec![
            CallItem::new("a").append("A"),
            CallItem::new("b").append("B"),
            CallItem::new("branch"),
            CallItem::new("some-thing").append("some-think"),
            CallItem::new("beforing"),
            CallItem::new("root").append("lvl2").append("lvl3"),
            CallItem::new("root").append("lvl2").append("lvl3").append("lvl4"),
    ];
    let mist;
    match from_toml_table( &tml, "workflows.sc2" ) {
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

