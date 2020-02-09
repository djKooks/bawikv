use bawidb::BawiDB;

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn it_works() {
        let bdb = BawiDB{
            file_path: String::from("test.db")
        };

        // bdb.put("key-1", "1");
        // bdb.put("key-2", "2");
        // bdb.put("key-3", "3");
        bdb.get("key-2");
    }
}