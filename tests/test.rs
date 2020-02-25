use bawikv::BawiKv;

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn it_works() {
        let bdb = BawiKv{
            file_path: String::from("test.db")
        };

        bdb.put("key-1", "1");
        bdb.put("key-2", "2");
        bdb.put("key-3", "3");

        println!("Result of key-2: {}", bdb.get("key-2").unwrap());
    }
}