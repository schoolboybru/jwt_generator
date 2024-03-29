use jsonwebtoken::{encode, Header, EncodingKey}; 
use clap::{Arg, Command};
use serde::Deserialize;
use std::{fs, collections::HashMap};

#[derive(Debug, Deserialize, PartialEq)]
pub struct SecretKey {
    value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Outer {
    payload: HashMap<String, toml::Value>,
    secretkey: SecretKey,
}

#[derive(Debug)]
enum JwtError {
    ReadFileErr(std::io::Error),
    CreateTokenErr(jsonwebtoken::errors::Error),
    TomlErr(toml::de::Error)
}

impl std::fmt::Display for JwtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JwtError::ReadFileErr(read_file_err) =>
                write!(f, "{}", read_file_err),
            JwtError::CreateTokenErr(create_token_err) =>
                write!(f, "{}", create_token_err),
            JwtError::TomlErr(toml_err) =>
                write!(f, "{}", toml_err),
        }
    }
}

impl From<std::io::Error> for JwtError {
    fn from(error: std::io::Error) -> Self {
        JwtError::ReadFileErr(error)
    }
}

impl From<jsonwebtoken::errors::Error> for JwtError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        JwtError::CreateTokenErr(error)
    }
}


impl From<toml::de::Error> for JwtError {
    fn from(error: toml::de::Error) -> Self {
        JwtError::TomlErr(error)
    }
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

    let myfile = matches.value_of("file").ok_or("Cannot find file");

    match myfile {
        Ok(file) => {
            let config_result = read_file(file.to_string());

            match config_result {
                Ok(token) => {
                    let jwt_result = create_jwt(&token);

                    match jwt_result {
                        Ok(jwt) => println!("{}", jwt),
                        Err(err) => println!("Error: {}", err)
                    }
                },
                Err(err) => println!("Error: {}", err)
            }

        },
        Err(err) => println!("Error: {}", err)
    }
}

fn create_jwt(config: &Outer) -> Result<String, JwtError>  {
    let payload = &config.payload;
    let secret = &config.secretkey.value.to_string();
    let result = encode(&Header::default(), payload, &EncodingKey::from_secret(secret.as_ref()))?; 

    Ok(result)
}

fn read_file(file: String) -> Result<Outer, JwtError> {
    let result = fs::read_to_string(file).map_err(JwtError::ReadFileErr)?;

    let config: Outer = toml::from_str(&result)?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{decode, DecodingKey, Validation};
    use::toml::Value;

    #[test]
    fn read_file_test() {
        let mut values: HashMap<String, toml::Value> = HashMap::new();
        values.insert("sub".to_string(),  toml::Value::String("1234567890".to_string()));
        values.insert("name".to_string(), toml::Value::String("John Doe".to_string()));
        values.insert("iat".to_string(),  Value::Integer(1516239022));

        let mock = Outer {
            payload: values,
            secretkey: SecretKey { value: "secretKey".to_string() }
        };

        assert_eq!(mock, read_file("./Config.toml".to_string()).unwrap());
    }

    #[test]
    fn create_jwt_test() {
        let mut values: HashMap<String, toml::Value> = HashMap::new();
        values.insert("sub".to_string(),  toml::Value::String("1234567890".to_string()));
        values.insert("name".to_string(), toml::Value::String("John Doe".to_string()));
        values.insert("exp".to_string(), Value::Integer(100000000000000000));
        values.insert("iat".to_string(),  Value::Integer(1516239022));

        let mock = Outer {
            payload: values,
            secretkey: SecretKey { value: "secretKey".to_string() }
        };

        let token = create_jwt(&mock);

        println!("Token: {:?}", token);

        let result = decode::<HashMap<String, toml::Value>>(&token.unwrap(), &DecodingKey::from_secret("secretKey".as_ref()), &Validation::default());

        assert_eq!(mock.payload, result.unwrap().claims);
    }
}
