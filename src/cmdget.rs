use std;
use std::io::BufRead;

pub fn execute(file_name: &str) -> Result<String, Box<std::error::Error>> {
    let (protocol, host) = parse_stdin()?;

    if protocol.is_empty() || host.is_empty() {
        return Ok(String::new());
    }
    Ok(find_in_store(&protocol, &host, file_name)?)
}

fn parse_stdin() -> Result<(String, String), Box<std::error::Error>> {
    let stdin = std::io::stdin();
    let mut protocol = String::new();
    let mut host = String::new();

    for line in stdin.lock().lines() {
        let linestr = line.unwrap_or(String::new());

        if linestr.is_empty() {
            break;
        }

        if linestr.starts_with("protocol=") {
            protocol = linestr.split_at(9).1.to_string() + &"://";
        } else if linestr.starts_with("host=") {
            host = linestr.split_at(5).1.to_string();
        }
    }

    Ok((protocol, host))
}

fn find_in_store(
    protocol: &str,
    host: &str,
    file_name: &str,
) -> Result<String, Box<std::error::Error>> {
    let f = std::fs::File::open(file_name)?;
    let mut result: String = String::new();

    for line in std::io::BufReader::new(f).lines() {
        let linestr = line.unwrap_or(String::new());

        if linestr.starts_with(protocol) && linestr.ends_with(host) {
            result = String::from("url=") + linestr.as_str();
        }
    }

    Ok(result)
}
