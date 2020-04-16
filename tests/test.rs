use bawikv::BawiKv;
use std::path::Path;

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        let path = Path::new("test.db");
        let bdb = BawiKv::open(path);
    }
}
