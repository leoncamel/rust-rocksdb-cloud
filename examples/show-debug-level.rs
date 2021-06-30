use rocksdb::{LogLevel, Options};

pub fn main() {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.set_db_log_level(LogLevel::Debug);

    let loglevel = opts.get_db_log_level();

    println!("hello, rocksdb-main");
    println!("LogLevel is {:?}", loglevel);

    // println!("{:?}", opts);
}

