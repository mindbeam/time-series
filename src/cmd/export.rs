use std::{
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
        .open(&path)?;

    for i in store.iter() {
        if let Ok((k, v)) = i {
            file.write_vectored(&[IoSlice::new(&v), IoSlice::new(b"\n")])?;
        }
    }

    Ok(())
}
