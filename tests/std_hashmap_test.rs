use env_logger;
use log::{debug, info};
use rand::Rng;
use std::collections::HashMap;

struct Elem {
    key: u64,
    value: u64,
}

#[test]
fn std_hashmap() {
    // setup
    env_logger::init();
    let mut origin_elems = Vec::new();
    let pressure = 10000;
    // let pressure = 8;
    let mut rng = rand::thread_rng();
    let mut reference_table = HashMap::new();
    for _ in 0..pressure {
        let k = rng.gen_range(10000, 1000000);
        let v = rng.gen_range(1, 100);
        origin_elems.push(Elem { key: k, value: v });
        reference_table.insert(k, v);
    }
    info!(
        "setup finished, pressure: {}, table length: {}",
        origin_elems.len(),
        reference_table.len()
    );
    debug!("reference_table: {:?}", reference_table);

    // new
    use hashing::OpenAddressing;
    let mut table = OpenAddressing::new();

    // insert
    for elem in origin_elems.into_iter() {
        table.insert(elem.key, elem.value);
    }

    // lookup
    for (k, v)in reference_table.into_iter() {
        assert_eq!(table.lookup(k), v);
    }

    // removal
}
