use extendr_api::prelude::*;
use std::fs::File;
use arrow2::error::Error;
use arrow2::io::parquet::read;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

fn parquet_read_schema_internal(path: &str, stream_ptr: &str) -> std::result::Result<(), Error> {
    let mut reader = File::open(path)?;

    // we can read its metadata:
    let metadata = read::read_metadata(&mut reader)?;

    // and infer a [`Schema`] from the `metadata`.
    let schema = read::infer_schema(&metadata)?;

    Ok(())
}

#[extendr]
fn parquet_read_schema(path: &str, stream_ptr: &str) -> Result<()> {
    let result = parquet_read_schema_internal(path, stream_ptr);
    if result.is_err() {
        let e = result.expect_err("msg");
        Err(extendr_api::Error::from(e.to_string()))
    } else {
        Ok(())
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rparquet;
    fn hello_world;
    fn parquet_read_schema;
}
