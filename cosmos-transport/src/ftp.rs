use std::io::Read;

struct FtpUrl {
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

fn parse_url(url: &str) -> Result<FtpUrl, crate::TransportError> {
    let url = url.trim_start_matches("ftp://");
    let parts: Vec<&str> = url.split('/').collect();

    if parts.len() < 2 {
        return Err(crate::TransportError::UnsupportedUrlScheme(url.to_string()));
    }
    let mut host_data = parts[0].split(':');
    // ftp urli is ftp://[user[:password]@]host[:port]/[url-path]
    let user_part = parts[0].split('@').next().unwrap_or("");
    if user_part.is_empty() {
        host_data = parts[0].split('@').nth(1).unwrap_or("").split(':');
    }
    let host = host_data.next().unwrap_or("").to_string();
    // port is either specified or default to 21 so always parse it with a default value
    let port = host_data.next().map(|p| p.parse::<u16>().unwrap_or(21));
    let path = parts[1..].join("/");
    let username = if user_part.contains('@') {
        Some(user_part.split(':').next().unwrap_or("").to_string())
    } else {
        None
    };
    let password = if user_part.contains(':') {
        Some(user_part.split(':').nth(1).unwrap_or("").to_string())
    } else {
        None
    };
    if host.is_empty() {
        return Err(crate::TransportError::UnsupportedUrlScheme(url.to_string()));
    }
    if path.is_empty() {
        return Err(crate::TransportError::UnsupportedUrlScheme(url.to_string()));
    }
    if username.is_none() && password.is_some() {
        return Err(crate::TransportError::UnsupportedUrlScheme(url.to_string()));
    }

    Ok(FtpUrl {
        host,
        port,
        path,
        username,
        password,
    })
}

pub(crate) fn pull(url: &str) -> Result<Vec<u8>, crate::TransportError> {
    // split url
    let ftp_url = parse_url(url)?;
    let host = ftp_url.host;
    let port = ftp_url.port.unwrap_or(21);
    let path = ftp_url.path;
    let username = ftp_url.username.unwrap_or_default();
    let password = ftp_url.password.unwrap_or_default();
    // connect to ftp server
    let mut ftp_stream = ftp::FtpStream::connect((host.as_str(), port))
        .map_err(|e| crate::TransportError::DownloadFailed(format!("{}: {}", url, e)))?;
    // login to ftp server
    if !username.is_empty() {
        ftp_stream
            .login(username.as_str(), password.as_str())
            .map_err(|e| crate::TransportError::DownloadFailed(format!("{}: {}", url, e)))?;
    } else {
        ftp_stream
            .login("anonymous", "anonymous")
            .map_err(|e| crate::TransportError::DownloadFailed(format!("{}: {}", url, e)))?;
    }
    // download file
    let mut reader = ftp_stream
        .simple_retr(path.as_str())
        .map_err(|e| crate::TransportError::DownloadFailed(format!("{}: {}", url, e)))?;
    let mut bytes = Vec::new();
    reader
        .read_to_end(&mut bytes)
        .map_err(|e| crate::TransportError::DownloadFailed(format!("{}: {}", url, e)))?;
    // close ftp connection
    ftp_stream
        .quit()
        .map_err(|e| crate::TransportError::DownloadFailed(format!("{}: {}", url, e)))?;
    // return bytes
    Ok(bytes)
}