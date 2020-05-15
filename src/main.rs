use async_std::task::{block_on, sleep};
use std::{thread, time};
use futures::future::join;

async fn hello() {
    println!("Hello");
}

async fn connect_db() -> String {
    sleep(time::Duration::from_secs(1)).await;
    String::from("db client")
}

async fn with_result() -> Result<String, ()> {
    Ok(String::from("hello from with_result"))
}

async fn connect_redis() -> String {
    sleep(time::Duration::from_secs(1)).await;
    String::from("redis client")
}

fn read_data(input: &str) -> String {
    thread::sleep(time::Duration::from_secs(1));
    format!("hello -> {}", input)
}

async fn main_executor() {
    hello().await;
    let (db, redis) = join(connect_db(), connect_redis()).await;
    println!("{} {}", db, redis);
    println!("{}", with_result().await.unwrap());

    let languages = vec!["Marathi", "telugu", "english"];
    let languages_result = languages.into_iter().map(|language|{
        read_data(language)
    }).collect::<Vec<String>>();
    println!("languages result {:?}", languages_result);
}

fn main() {
    let now = time::Instant::now();
    block_on(main_executor());
    println!("executed in {:?}", now.elapsed());
}