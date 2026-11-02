use std::time::Duration;

async fn fetch_label(label: &'static str, delay: Duration) -> &'static str {
    tokio::time::sleep(delay).await;
    label
}

pub async fn run() {
    let (first, second) = tokio::join!(
        fetch_label("first", Duration::from_millis(100)),
        fetch_label("second", Duration::from_millis(150)),
    );

    println!("completed concurrently: {first}, {second}");
}
