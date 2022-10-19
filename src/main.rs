use prettytable::{Table, Row, Cell};
use rocksdb::{DB, IteratorMode, Options};
use regex::Regex;
use std::env;

fn main() {
   print_table();
}

fn print_table() {
    let collapse_newline_re = Regex::new(r"[\n]+").unwrap();
    let add_qualifier_newline = Regex::new(r"(?P<n>!..)").unwrap();

    let path = env::var("SURREALDB_PATH").expect("Set environment variable");
    {
        let db = DB::open_for_read_only(&Options::default(),path, false).unwrap();
        let iter = db.iterator(IteratorMode::Start); // Always iterates forward

        let mut table = Table::new();

        for item in iter {
            let (key, value) = item.unwrap();
            let key_str = String::from_utf8_lossy(&key)
                                    .into_owned()
                                    .replace("\0","\n");
            let fixed_key = collapse_newline_re.replace_all(&key_str,"\n");
            let fixed_key2 = add_qualifier_newline.replace_all(&fixed_key,"$n\n");

            let val_str = String::from_utf8_lossy(&value).into_owned();
            print!("{:?}\n", &collapse_newline_re.replace_all(&key_str,"\n"));
            table.add_row(Row::new(vec![
                Cell::new(&fixed_key2),
                Cell::new(&val_str)
            ]));
        }

        table.printstd();
        
    }
}
