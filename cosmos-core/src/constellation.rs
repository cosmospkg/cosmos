#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Constellation {
    pub name: String,
    pub description: Option<String>,
    pub members: Vec<String>,
}

impl Constellation {
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        let parsed: Self = toml::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(parsed)
    }

    pub fn contains(&self, star: &str) -> bool {
        self.members.contains(&star.to_string())
    }
}