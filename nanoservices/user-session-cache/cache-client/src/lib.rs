use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use redis::aio::{ConnectionLike, MultiplexedConnection};
use redis::Value;
use std::error::Error;

async fn get_connection(address: &str) -> Result<MultiplexedConnection, NanoServiceError> {
    let client = redis::Client::open(address)
        .map_err(|e| NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown))?;

    let connection = client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown))?;

    Ok(connection)
}

fn unpack_result_string(result: Value) -> Result<String, NanoServiceError> {
    match result {
        Value::Status(s) => Ok(s),
        _ => Err(NanoServiceError::new(
            "Error converting the result into a string".to_string(),
            NanoServiceErrorStatus::Unknown,
        )),
    }
}
