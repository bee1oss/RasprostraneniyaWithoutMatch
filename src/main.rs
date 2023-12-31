use std::fs::File;
use std::io::{Error, ErrorKind, Read};

fn main() {
    let path = "file.txt";
    match read_file_date(path) {
        Ok(data) => println!("File data:{}", data),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(_) => println!("File create"),
                Err(e) => panic!("Error creating file:{e}")
            },
            other => panic!("Error occured:{:?}", other),
        },
    }
}

fn read_file_date(path: &str) -> Result<String, Error> {//soru isareti operatoru sadece result donduren fonksiyonlarda kullanilabilir yani main icerisinde kullanilamaz.
    let mut data = String::new();
    File::open(path)?.read_to_string(&mut data)?;
    Ok(data)
}

