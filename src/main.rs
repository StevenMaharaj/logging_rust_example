mod flog;
use tracing::info;
// use tracing_subscriber;
// use std::fs::File;
use std::error::Error;
// use std::path::Path;
#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {

    flog::get_logger()?;

    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    info!(number_of_yaks, "preparing to shave yaks");

    let number_shaved = 2;
    info!(
        all_yaks_shaved = number_shaved == number_of_yaks,
        "yak shaving completed."
    );
    Ok(())
}