use async_std::task::block_on;
use std::{thread, time};
use futures::future::join;

async fn hello() {
    println!("Hello");
}

async fn connect_db() -> String {
    thread::sleep(time::Duration::from_secs(1));
    String::from("db client")
}

async fn connect_redis() -> String {
    thread::sleep(time::Duration::from_secs(1));
    String::from("redis client")
}

async fn main_executor() {
    hello().await;
    let (db, redis) = join(connect_db(), connect_redis()).await;
    println!("{} {}", db, redis);
}

fn main() {
    let now = time::Instant::now();
    block_on(main_executor());
    println!("executed in {:?}", now.elapsed());
}