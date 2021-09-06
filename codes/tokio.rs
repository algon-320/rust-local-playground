//# tokio = { version = "1", features = ["full"] }
//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

#[tokio::main]
async fn main() {
    use std::time::Duration;

    let start = std::time::Instant::now();
    let fut = async move {
        tokio::time::sleep(Duration::from_secs(3)).await;
        123i32
    };
    std::thread::sleep(Duration::from_secs(3));
    let x = fut.await;
    dbg!(x);
    println!("{}", start.elapsed().as_millis());

    let start = std::time::Instant::now();
    let fut = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(3)).await;
        123i32
    });
    std::thread::sleep(Duration::from_secs(3));
    let x = fut.await;
    dbg!(x);
    println!("{}", start.elapsed().as_millis());
}
