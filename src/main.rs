use tokio::time::Instant;

#[tokio::main]
async fn main() {
    let instant = Instant::now();
    log::info!("🎉Started Application in {:.3?}", instant.elapsed());
}
