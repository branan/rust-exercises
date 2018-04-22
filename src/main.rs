extern crate serde_json;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use serde_json::{Value, to_string_pretty};

type Counts = HashMap<String, i32>;
fn aggregate(data: HashMap<String, Value>) -> HashMap<String, Counts> {
    let mut agg = HashMap::new();
    for (_, val) in data {
        if let Some(obj) = val.as_object() {
            for (k, v) in obj {
                let vals = agg.entry(k.clone()).or_insert(HashMap::<String,i32>::new());
                if let Some(value) = v.as_str() {
                    let nodes = vals.entry(value.to_string()).or_insert(0);
                    *nodes += 1
                } else {
                    println!("Cannot aggregate non-string values: {:?}", v)
                }
            }
        } else {
            println!("Not an Object: {:?}", val)
        }
    }
    agg
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let inputfile = if let Some(x) = args.get(1) { x } else {
        println!("Usage: collections <inputfile>");
        std::process::exit(1)
    };

    let file = File::open(inputfile).unwrap();
    let data: HashMap<String, Value> = serde_json::from_reader(file).unwrap();
    let agg = aggregate(data);
    println!("{}", to_string_pretty(&agg).unwrap())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
