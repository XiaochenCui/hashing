use env_logger;
use log::{debug, info, error};
use rand::Rng;
use std::collections::HashMap;
use hashing::OpenAddressing;
use hashing::Elem;
// use hashing::VecElem;

#[test]
fn std_hashmap() {
    // setup
    env_logger::init();
    let mut origin_elems = Vec::new();
    // let mut origin_elems = VecElem::new();
    let pressure = 100000;
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
    let mut table = OpenAddressing::new();

    // insert
    for elem in &origin_elems {
        table.insert(elem.key, elem.value);
    }

    // check table
    assert_eq!(reference_table.len(), table.len());
    table.check();

    // lookup
    for (k, v)in &reference_table {
        if table.lookup(*k) != *v {
            error!("value inconsistent at key {}", k);

            // find k in origin_elems
            for e in &origin_elems {
                if e.key == *k {
                    info!("origin elems: {} -> {}", k, e.value);
                }
            }
            info!("reference table: {} -> {}", k, reference_table.get(k).unwrap());
            info!("table: {} -> {}", k, table.lookup(*k));
            panic!();
        }
    }

    assert_eq!(reference_table.len(), table.len());
    table.check();

    // removal
    for (k, _)in &reference_table {
        table.remove(k);
    }

    // reference_table.remove(&2);

    assert_eq!(0, table.len());
    table.check();
}
