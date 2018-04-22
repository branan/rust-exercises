extern crate serde_json;

use std::collections::HashMap;
use std::env;
use std::fs::File;

type Counts = HashMap<String, i32>;
type JsonHash = HashMap<String, HashMap<String, String>>;

fn aggregate(data: JsonHash) -> HashMap<String, Counts> {
    let mut agg = HashMap::new();
    for (_, val) in data {
        for (k, v) in val {
            let vals = agg.entry(k).or_insert(Counts::new());
            let nodes = vals.entry(v).or_insert(0);
            *nodes += 1
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
    let data: JsonHash = match serde_json::from_reader(file) {
        Ok(d) => d,
        Err(e) => { println!("{}", e); std::process::exit(1) }
    };
    let agg = aggregate(data);
    println!("{}", serde_json::to_string_pretty(&agg).unwrap())
}

#[cfg(test)]
#[macro_use] extern crate maplit;

mod tests {
    #[test]
    fn aggregate_works() {
        let data = hashmap!{
            String::from("node1") => hashmap!{
                String::from("application") => String::from("forge"),
                String::from("version") => String::from("10.0"),
                String::from("role") => String::from("app-server"),
            },
            String::from("node2") => hashmap!{
                String::from("application") => String::from("forge"),
                String::from("version") => String::from("10.0"),
                String::from("role") => String::from("util"),
                String::from("location") => String::from("slice"),
            },
            String::from("node3") => hashmap!{
                String::from("application") => String::from("forge"),
                String::from("version") => String::from("9.5"),
                String::from("role") => String::from("db"),
            },
            String::from("node4") => hashmap!{
                String::from("application") => String::from("anubis"),
                String::from("location") => String::from("slice"),
                String::from("role") => String::from("worker"),
            },
        };

        let agg = ::aggregate(data);
        assert_eq!(agg, hashmap!{
            String::from("application") => hashmap!{
                String::from("forge") => 3,
                String::from("anubis") => 1,
            },
            String::from("version") => hashmap!{
                String::from("10.0") => 2,
                String::from("9.5") => 1,
            },
            String::from("role") => hashmap!{
                String::from("app-server") => 1,
                String::from("util") => 1,
                String::from("db") => 1,
                String::from("worker") => 1,
            },
            String::from("location") => hashmap!{
                String::from("slice") => 2,
            }
        });
    }
}
