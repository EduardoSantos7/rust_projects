use std::io::Read;

use tokio::{io::AsyncReadExt, time};
use log::Level;


fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

async fn sleeper() {
    log::info!("Sleeping");
    time::sleep(time::Duration::from_secs(1)).await;
    log::info!("Awake");
}

async fn reader() {
    log::info!("Reading some data");
    let mut f = tokio::fs::File::open("Cargo.toml").await.unwrap();
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.unwrap();
    log::info!("Reading {} bytes", contents.len());

    tokio::task::spawn_blocking(move || {
        fib(45);
    }).await.unwrap();

}

fn sleeper_sync() {
    log::info!("Sleeping");
    std::thread::sleep(std::time::Duration::from_secs(1));
    log::info!("Awake");
}

fn reader_sync() {
    log::info!("Reading some data");
    let mut f = std::fs::File::open("Cargo.toml").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    log::info!("Reading {} bytes", contents.len());

    fib(45);
}

async fn run_async() {
    tokio::join!(
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        sleeper());
}

fn run_sync() {
    for _ in 0..10 {
        reader_sync();
    }
    sleeper_sync();
}

#[tokio::main]
async fn main () {
    simple_logger::init_with_level(Level::Info).unwrap();
    let start = std::time::Instant::now();
    run_async().await;
    let end = std::time::Instant::now();
    let total_time = end - start;

    let start_sync = std::time::Instant::now();
    run_sync();
    let end_sync = std::time::Instant::now();
    let total_time_sync = end_sync - start_sync;

    println!("Took {:?} seconds for the async version", total_time);
    println!("Took {:?} seconds for the sync version", total_time_sync);
    println!("The async version is {} % faster", (1.0 - (total_time.as_secs_f64() / total_time_sync.as_secs_f64())) * 100.0);
}