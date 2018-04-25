extern crate failure;
extern crate serde_json;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

use failure::{err_msg, Error};

type Counts<'a> = HashMap<&'a str, i32>;
type JsonHash<'a> = HashMap<&'a str, HashMap<&'a str, &'a str>>;

fn aggregate<'a> (data: &'a JsonHash) -> HashMap<&'a str, Counts<'a>> {
    let mut agg = HashMap::new();
    for (_, val) in data {
        for (k, v) in val {
            let vals = agg.entry(*k).or_insert(Counts::new());
            let nodes = vals.entry(v).or_insert(0);
            *nodes += 1
        }
    }
    agg
}

fn inner_main() -> Result<(), Error> {
    let args: Vec<_> = env::args().collect();
    let inputfile = args.get(1).ok_or(err_msg("Usage: collections <inputfile>"))?;

    let mut input_string = String::new();
    File::open(inputfile)?.read_to_string(&mut input_string)?;
    let data = serde_json::from_str(&input_string)?;
    let agg = aggregate(&data);
    println!("{}", serde_json::to_string_pretty(&agg)?);

    Ok(())
}

fn main() {
    inner_main().unwrap();
}

#[cfg(test)]
#[macro_use] extern crate maplit;

mod tests {
    #[test]
    fn aggregate_works() {
        let data = hashmap!{
            "node1" => hashmap!{
                "application" => "forge",
                "version" => "10.0",
                "role" => "app-server",
            },
            "node2" => hashmap!{
                "application" => "forge",
                "version" => "10.0",
                "role" => "util",
                "location" => "slice",
            },
            "node3" => hashmap!{
                "application" => "forge",
                "version" => "9.5",
                "role" => "db",
            },
            "node4" => hashmap!{
                "application" => "anubis",
                "location" => "slice",
                "role" => "worker",
            },
        };

        let agg = ::aggregate(&data);
        assert_eq!(agg, hashmap!{
            "application" => hashmap!{
                "forge" => 3,
                "anubis" => 1,
            },
            "version" => hashmap!{
                "10.0" => 2,
                "9.5" => 1,
            },
            "role" => hashmap!{
                "app-server" => 1,
                "util" => 1,
                "db" => 1,
                "worker" => 1,
            },
            "location" => hashmap!{
                "slice" => 2,
            }
        });
    }
}
