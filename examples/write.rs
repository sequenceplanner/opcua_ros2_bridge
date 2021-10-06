use r2r;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "testnode", "")?;
    let duration = std::time::Duration::from_millis(2500);

    let mut timer = node.create_wall_timer(duration)?;
    let publisher = node.create_publisher::<r2r::std_msgs::msg::String>("/command")?;

    let handle = tokio::task::spawn_blocking(move || loop {
        node.spin_once(std::time::Duration::from_millis(100));
    });

    let mut b = false;
    for _ in 1u8..10u8 {
        timer.tick().await?;
        let json = serde_json::json!({ "ns=4;i=26": b });
        let json_str = serde_json::to_string_pretty(&json).unwrap();
        println!("sending:\n{}\n", json_str);
        let msg = r2r::std_msgs::msg::String{data: json_str };
        publisher.publish(&msg)?;
        b = !b;
    }


    handle.await?;
    Ok(())
}
