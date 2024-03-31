use std::{env, io::Write};

use crate::error::Error;

pub fn test_log() -> Result<(), Error> {
    push_to_log(format!("\n"))
}

pub fn push_to_log(message: String) -> Result<(), Error> {
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(env::var("RSN_LOG_FILE")?)?;
    let mut log_writer = std::io::BufWriter::new(log_file);
    log_writer.write_all(message.as_bytes()).map_err(Error::Io)
}
