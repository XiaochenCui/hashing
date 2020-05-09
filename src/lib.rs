use log::{debug, info};
use std::fmt;

#[derive(Copy, Clone)]
#[derive(Default)]
pub struct Elem {
    pub key: u64,
    pub value: u64,
    empty: bool,
}

impl Elem {
    pub fn new(k: &u64, v: &u64) -> Elem {
        Elem{
            key: *k,
            value: *v,
            empty: false,
        }
    }
}

pub struct OpenAddressing {
    elem_list: Vec<Elem>,
    len: usize,
    cap: usize,
    empty: usize,
}

impl fmt::Display for OpenAddressing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elems = self.elem_list.to_owned();
        let mut content: String = "{".to_owned();
        for elem in elems.into_iter() {
            if elem.value != 0 {
                let elem_str = format!("{}: {},", elem.key, elem.value);
                content.push_str(&elem_str);
            }
        }
        content.push_str(&"}");
        write!(f, "{}", content,)
    }
}

impl fmt::Debug for OpenAddressing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elems = self.elem_list.to_owned();
        let mut content: String = "{".to_owned();
        for (index, elem) in elems.into_iter().enumerate() {
            let elem_str = format!(
                "(index: {}, key: {}, value: {}),",
                index, elem.key, elem.value
            );
            content.push_str(&elem_str);
        }
        content.push_str(&"}");
        write!(
            f,
            "len: {}, cap: {}, empty: {}, content: {}",
            self.len, self.cap, self.empty, content
        )
    }
}

impl OpenAddressing {
    pub fn new() -> OpenAddressing {
        const INIT_LEN: usize = 7;
        let mut elem_list = Vec::new();
        for _ in 0..INIT_LEN {
            elem_list.push(Elem {
                key: 0,
                value: 0,
                empty: true,
            });
        }
        debug!("table initialized, table size: {}", INIT_LEN);
        OpenAddressing {
            elem_list: elem_list,
            len: 0,
            cap: INIT_LEN,
            empty: INIT_LEN,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn insert(&mut self, key: u64, value: u64) {
        // check capacity
        if self.empty < 3 {
            debug!(
                "empty slots number is {}, we are going to rehash",
                self.empty
            );
            self.rehash();
        }

        let mut index = (key as usize) % self.cap;
        loop {
            // let holder = self.elem_list[index].key;
            let mut e = self.elem_list.get_mut(index).unwrap();
            if e.empty {
                // update e
                e.key = key;
                e.value = value;
                e.empty = false;

                // update table
                self.len += 1;
                self.empty -= 1;

                debug!("insert success, vec: {}", self);
                break;
            } else if e.key == key {
                // update value
                e.value = value;

                // no need to update table

                break;
            } else {
                index += 1;
                index %= self.cap;
                debug!("insert failed, update index to {}", index);
            }
        }
    }

    fn rehash(&mut self) {
        let old_elem_list = self.elem_list.to_owned();

        self.cap *= 2;
        let mut new_elem_list = Vec::new();
        for _ in 0..self.cap {
            new_elem_list.push(Elem {
                key: 0,
                value: 0,
                empty: true,
            });
        }
        self.elem_list = new_elem_list;
        self.empty = self.cap;
        self.len = 0;

        for e in old_elem_list.into_iter() {
            debug!("reinsert to new list: {}->{}", e.key, e.value);
            if e.key != 0 {
                self.insert(e.key, e.value)
            }
        }
        debug!("rehash finished, table: {}", self);
        self.check();
    }

    pub fn lookup(&mut self, k: u64) -> u64 {
        let mut index = k as usize % self.cap;
        loop {
            let e = self.elem_list.get(index).unwrap();
            if e.key == k {
                return e.value;
            } else {
                index += 1;
                index %= self.cap;
            }
        }
    }

    pub fn remove(&mut self, k: &u64) -> Option<u64> {
        let key = *k;
        let mut index = key as usize % self.cap;
        loop {
            let mut e = self.elem_list.get_mut(index).unwrap();

            debug!("index: {}", index);

            // check if terminate
            if e.key == 0{
                return None;
            }

            if e.key == key {
                if e.empty {
                    // aleady deleted
                    return None;
                } else {
                    e.empty = true;

                    // check length
                    assert!(self.len > 0);

                    self.len -= 1;
                    self.empty += 1;
                    return Some(e.value);
                }
            } else {
                index += 1;
                index %= self.cap;
            }
        }
    }

    pub fn check(&self) {
        debug!("start map checker");
        let mut empty = 0;
        for e in &self.elem_list {
            if e.empty {
                empty += 1;
            }
        }
        info!(
            "check finished, len: {}, cap: {}, empty: {}, actual empty: {}",
            self.len(),
            self.cap,
            self.empty,
            empty,
        );
        assert_eq!(empty, self.empty);
    }
}
