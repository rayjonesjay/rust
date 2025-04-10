/*
the Result type is an enum that looks like this:
enum Result {
    Ok(T)
    Err(E)
}

The E and T are generic type parameters.
T represents the value that will be returned when there are no errors (success).
T will be returned by the Ok variant.

E represents the type of error that will be returned when an error occurs.

*/
#[derive(Debug)]
pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::UnexpectedUrl => {
            match server {
                Ok(url) => panic!("{}", url),
                Err(err_msg) => err_msg.to_string(),
            }
        },
        _ => {
            match server {
                Ok(url) => url.to_string(),
                Err(err_msg) => {
                    match security_level {
                        Security::Unknown => server.unwrap().to_string(),
                        Security::Message => panic!("ERROR: program stops"),
                        Security::Warning => "WARNING: check the server".to_string(),
                        Security::NotFound => format!("Not found: {}", err_msg),
                        Security::UnexpectedUrl => unreachable!(), // Already handled
                    }
                }
            }
        }
    }
}