use tokio::time::timeout;

#[tokio::test]
async fn test_webapp() {
    log::info!("running test for webapp");
    if let Ok(res) = timeout(std::time::Duration::from_secs(5), webapp::run()).await {
        res.expect("webapp exited before timeout")
    };
}
