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

    // Split into host segment and path segment (requires at least one '/').
    let (host_segment, path) = match url.split_once('/') {
        Some((h, p)) if !h.is_empty() && !p.is_empty() => (h, p),
        _ => {
            return Err(crate::TransportError::UnsupportedUrlScheme(url.to_string()));
        }
    };

    // Separate optional user info from host and port.
    let (userinfo, host_port) = if let Some((u, h)) = host_segment.rsplit_once('@') {
        (Some(u), h)
    } else {
        (None, host_segment)
    };

    // Parse username and password from userinfo, if present.
    let (username, password) = if let Some(info) = userinfo {
        let mut parts = info.splitn(2, ':');
        let user = parts.next().unwrap_or("");
        let pass = parts.next();

        if user.is_empty() {
            return Err(crate::TransportError::UnsupportedUrlScheme(url.to_string()));
        }

        (Some(user.to_string()), pass.map(|p| p.to_string()))
    } else {
        (None, None)
    };

    if username.is_none() && password.is_some() {
        return Err(crate::TransportError::UnsupportedUrlScheme(url.to_string()));
    }

    // Parse host and optional port.
    let mut host_parts = host_port.splitn(2, ':');
    let host = host_parts.next().unwrap_or("");
    if host.is_empty() {
        return Err(crate::TransportError::UnsupportedUrlScheme(url.to_string()));
    }
    let port = host_parts.next().map(|p| p.parse::<u16>().unwrap_or(21));

    Ok(FtpUrl {
        host: host.to_string(),
        port,
        path: path.to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_url() {
        let url = parse_url("ftp://example.com/file.txt").unwrap();
        assert_eq!(url.host, "example.com");
        assert_eq!(url.port, None);
        assert_eq!(url.path, "file.txt");
        assert!(url.username.is_none());
        assert!(url.password.is_none());
    }

    #[test]
    fn parse_url_with_auth_and_port() {
        let url = parse_url("ftp://user:pass@example.com:2121/path/file.tar").unwrap();
        assert_eq!(url.host, "example.com");
        assert_eq!(url.port, Some(2121));
        assert_eq!(url.path, "path/file.tar");
        assert_eq!(url.username.as_deref(), Some("user"));
        assert_eq!(url.password.as_deref(), Some("pass"));
    }
}