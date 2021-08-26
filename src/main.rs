use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{Error, Write};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Err de fichier")]
struct FichierError {
    #[from]
    source: Error
}

fn main() -> Result<(), FichierError> {
    println!("Start");
    let buff = String::from("Sample buffer data");
    action(buff)?;
    println!("Stop");
    Ok(())
}

fn action(buff :String) -> Result<String, FichierError> {
    let mut f = match File::open("hello.txt") {
        Ok(f) => f,
        Err(a_error) => {
            return Err(FichierError {source: a_error});
        }
    };
    f.write_all(buff.as_bytes());
    Ok(buff)
}
