use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{Error, Write};

struct FichierError;

impl Display for FichierError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Erreur de fichier")
    }
}

impl Debug for FichierError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Debug: Erreur de fichier")
    }
}

fn main() {
    println!("Start");
    let buff = String::from("Sample buffer data");
    match action(buff) {
        Ok(_) => {
            println!("OK");
        }
        Err(e) => {
            println!("KO {:?}", e);
        }
    }

    println!("Stop");
}

fn action(buff :String) -> Result<String, FichierError> {
    let mut f = match File::open("hello.txt") {
        Ok(f) => f,
        Err(a_error) => {
            return Err(FichierError {});
        }
    };
    f.write_all(buff.as_bytes());
    Ok(buff)
}
