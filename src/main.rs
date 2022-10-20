use jsonwebtoken::{encode, Header, EncodingKey, };
use clap::{Arg, Command};
use serde::Deserialize;
use std::{io::Result, collections::HashMap, fs};

#[derive(Debug, Deserialize, PartialEq)]
pub struct Outer {
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

pub fn create_jwt(config: Outer) -> Result<String>  {
    let payload = &config.payload;
    let result = encode(&Header::default(), payload, &EncodingKey::from_secret("secret".as_ref())).unwrap(); 

    Ok(result)
}

pub fn read_file(file: String) -> Result<Outer> {
    let read = fs::read_to_string(file)?;

    let config: Outer = toml::from_str(&read)?;

    return Ok(config);
}

#[cfg(test)]
mod tests {
    use super::*;
    use::toml::Value;

    #[test]
    fn read_file_test() {
        let mut values: HashMap<String, toml::Value> = HashMap::new();
        values.insert("sub".to_string(),  toml::Value::String("1234567890".to_string()));
        values.insert("name".to_string(), toml::Value::String("John Doe".to_string()));
        values.insert("iat".to_string(),  Value::Integer(1516239022));

        let mock = Outer {
            payload: values,
        };

        assert_eq!(mock, read_file("./Config.toml".to_string()).unwrap());
    }
}
