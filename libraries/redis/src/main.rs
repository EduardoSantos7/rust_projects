#![allow(unused)]
#![allow(deprecated)]


use std::{ error::Error, time::Duration };
use tokio::time::sleep;

use redis::{
    from_redis_value,
    streams::{ StreamRangeReply, StreamReadOptions, StreamReadReply },
    AsyncCommands,
    Client
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    // Create connection
    let client = Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_tokio_connection().await?;

    // Set/Get Key
    con.set("portal", "abc").await?;
    let mut result: String = con.get("portal").await?;
    println!("key: {}", result);

    // add to redis stream
    con.xadd("my_stream", "*", &[("name", "name-01"),
                                ("title", "title 01")]).await?;

    let len: i32 = con.xlen("my_stream").await?;
    println!("my stream len {}", len);

    // xrevrange to read the stream
    let result: Option<StreamRangeReply> = con.xrevrange_count("my_stream", "+", "-", 10).await?;
    if let Some(reply) = result {
        for stream_id in reply.ids {
            println!("xrevrange stream entity: {}", stream_id.id);
            for (name, value) in stream_id.map.iter() {
                println!("{}:{}", name, from_redis_value::<String>(value)?);
            }
        }
    }
    
    // Final wait and clean up
    con.del("portal").await?;    
    con.del("my_stream").await?;

    return Ok(())
}
