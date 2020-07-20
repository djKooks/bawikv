use bawikv::BawiKv;
use std::path::Path;

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        let path = Path::new("test_db");
        let mut bdb = BawiKv::open(path).unwrap();
        for i in 0..10 {
            let key = format!("{}-{}", "key", i);
            let val = format!("{}-{}", "value", i);
            bdb.put(key, val);
        }
        /*
        println!("read value");
        let res = bdb.get("key-1".to_owned()).expect("problem!");
        println!("res => {:?}", res);
        match res {
            None => println!("No result!"),
            Some(val) => println!("value for key-3: {}", val),
        }
        */
    }
}
