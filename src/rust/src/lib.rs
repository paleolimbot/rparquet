use extendr_api::prelude::*;
use std::fs::File;
use arrow2::error::Error;
use arrow2::io::parquet::read;
use arrow2::datatypes::DataType;
use arrow2::datatypes::Field;
use arrow2::ffi;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

fn parquet_read_schema_internal(path: &str, schema_ptr: &str) -> std::result::Result<(), Error> {
    let mut reader = File::open(path)?;

    // we can read its metadata:
    let metadata = read::read_metadata(&mut reader)?;

    // and infer a [`Schema`] from the `metadata`.
    let schema = read::infer_schema(&metadata)?;
    let data_type = DataType::Struct(schema.fields);
    let field = Field::new("", data_type, false);

    let mut exported = ffi::export_field_to_c(&field);
    let stream_out_ptr_addr: usize = schema_ptr.parse().unwrap();
    let stream_out_ptr = stream_out_ptr_addr as *mut ffi::ArrowSchema;
    unsafe {
        std::ptr::swap_nonoverlapping(stream_out_ptr, &mut exported as *mut ffi::ArrowSchema, 1);
    }

    Ok(())
}

#[extendr]
fn parquet_read_schema(path: &str, schema_ptr: &str) -> Result<()> {
    let result = parquet_read_schema_internal(path, schema_ptr);
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
