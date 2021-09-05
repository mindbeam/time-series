use std::{
    convert::TryInto,
    fs::{File, OpenOptions},
    io::{BufRead, IoSlice, Write},
    path::PathBuf,
};

use crate::{
    module::hubitat::model::{Event, DTO},
    store::Store,
};

pub(crate) fn import(store: Store, input_file: PathBuf) -> Result<(), std::io::Error> {
    let path = input_file.as_path();
    let display = path.display();

    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .truncate(false)
        .open(&path)?;

    // let mut set = std::collections::HashSet::new();
    let lines = std::io::BufReader::new(file).lines();
    for maybe_line in lines {
        if let Ok(line) = maybe_line {
            if let Some(colon_idx) = line.find(':') {
                process_line(&store, &line[colon_idx + 1..])?;
            }
        }
    }

    Ok(())
}

fn process_line(store: &Store, line: &str) -> Result<(), std::io::Error> {
    let dto: DTO = serde_json::from_str(line).unwrap();
    // println!("{:?}", dto);
    let event = Event::from_dto(dto).unwrap();
    // set.insert(format!("{}: {}", dto.source, dto.name));

    // println!("{}", bincode::serialize(&event);
    Ok(())
}
