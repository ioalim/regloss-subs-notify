mod regloss;
mod error;
mod log;
mod token;
mod tweet;

#[tokio::main]
async fn main() {
    if let Err(e) = log::test_log() {
        eprintln!("{:#?}", e);
    } else {
        match tweet::post().await {
            Ok(_) => println!("Success"),
            Err(e) => {
                log::push_to_log(format!("{}\n", time::OffsetDateTime::now_utc())).unwrap();
                log::push_to_log(format!("{:#?}", e)).unwrap();
            }
        }
    }
}
