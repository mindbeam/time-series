use std::{
    convert::TryInto,
    fs::{File, OpenOptions},
    io::{IoSlice, Write},
    path::PathBuf,
};

use crate::store::Store;

pub(crate) fn export(store: Store, output_file: PathBuf) -> Result<(), std::io::Error> {
    let path = output_file.as_path();
    let display = path.display();

    let mut file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;

    for i in store.iter() {
        if let Ok((k, v)) = i {
            file.write_vectored(&[
                IoSlice::new(format!("{}", read_be_u64(&k)).as_bytes()),
                IoSlice::new(b":"),
                IoSlice::new(&v),
                IoSlice::new(b"\n"),
            ])?;
        }
    }

    Ok(())
}

fn read_be_u64(input: &[u8]) -> u64 {
    let (int_bytes, _rest) = input.split_at(std::mem::size_of::<u64>());
    // *input = rest;
    u64::from_be_bytes(int_bytes.try_into().unwrap())
}
