use rocksdb::{Options, DB, LogLevel};

fn main() {
    // NB: db is automatically closed at end of lifetime
    let path = "/tmp/hello-rust-rocksdb";
    {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_db_log_level(LogLevel::Debug);

        let db = DB::open(&opts, path).unwrap();
        for i in 1..10 {
          let key = format!("key_{:08}", i);
          let val = format!("val_{:016}", i);
          let _ = db.put(key.as_bytes(), val.as_bytes());
        }

        db.put(b"my key", b"my value").unwrap();
        match db.get(b"my key") {
            Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }
        db.delete(b"my key").unwrap();
    }
    // let _ = DB::destroy(&Options::default(), path);
}
