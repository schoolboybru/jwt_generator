use jsonwebtoken::{encode, Header, EncodingKey, };
use clap::{Arg, Command};
use serde::Deserialize;
use std::{io::Result, collections::HashMap, fs};

#[derive(Debug, Deserialize)]
struct Outer {
    payload: HashMap<String, toml::Value>
}

fn main() {
    get_arguments();
}

fn get_arguments() {
    let matches = Command::new("JWT Generator")
        .version("0.1.0")
        .author("Brandon Bachynski")
        .arg_required_else_help(true)
        .arg(Arg::with_name("file")
            .short('f')
            .long("file")
            .takes_value(true)
            .help("A file to be read"))
        .get_matches();

    let myfile = matches.value_of("file").expect("Invalid file");

    let config = read_file(myfile.to_string()).expect("Unable to read file");

    let jwt = create_jwt(config);

    println!("{:?}", jwt.unwrap());
}

fn create_jwt(config: Outer) -> Result<String>  {
    let payload = &config.payload;
    let result = encode(&Header::default(), payload, &EncodingKey::from_secret("secret".as_ref())).unwrap(); 

    Ok(result)
}

fn read_file(file: String) -> Result<Outer> {
    let read = fs::read_to_string(file)?;

    let config: Outer = toml::from_str(&read)?;

    return Ok(config);
}
