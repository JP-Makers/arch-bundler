use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Metadata {
    pub name: String,
    pub version: String,
    pub release: String,
    pub description: String,
    pub arch: Vec<String>,
    pub url: String,
    pub license: String,
    pub depends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub sources: Vec<String>,
    pub md5sums: Vec<String>,
    pub sha1sums: Vec<String>,
    pub sha256sums: Vec<String>,
    pub sha512sums: Vec<String>,
    pub build_env: Vec<String>,
    pub appimage_exec: String,
    pub package_instructions: Vec<String>,
    pub appimage_icon_instructions: Vec<String>,
    pub appimage_desktop_instructions: Vec<String>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: String::new(),
            release: String::new(),
            description: String::new(),
            arch: Vec::new(),
            url: String::new(),
            license: String::new(),
            depends: Vec::new(),
            provides: Vec::new(),
            conflicts: Vec::new(),
            sources: Vec::new(),
            md5sums: Vec::new(),
            sha1sums: Vec::new(),
            sha256sums: Vec::new(),
            sha512sums: Vec::new(),
            build_env: vec![
                "!distcc".to_string(),
                "color".to_string(),
                "!ccache".to_string(),
                "check".to_string(),
                "!sign".to_string(),
            ],
            appimage_exec: String::new(),
            package_instructions: Vec::new(),
            appimage_icon_instructions: Vec::new(),
            appimage_desktop_instructions: Vec::new(),
        }
    }
}

pub fn extract_metadata(metadata_path: &str) -> Result<Metadata, Box<dyn std::error::Error>> {
    let metadata_file = File::open(metadata_path)?;
    let reader = BufReader::new(metadata_file);

    let mut metadata = Metadata::default();
    let mut lines = reader.lines();
    while let Some(line) = lines.next() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = trimmed.split_once(':') {
            let key = key.trim();
            let value = value.trim();

            if value == "{" {
                let mut block_lines = Vec::new();
                while let Some(block_line) = lines.next() {
                    let block_line = block_line?;
                    let trimmed_block = block_line.trim();
                    if trimmed_block == "}" {
                        break;
                    }
                    if !trimmed_block.is_empty() && !trimmed_block.starts_with('#') {
                        block_lines.push(trimmed_block.to_string());
                    }
                }
                match key {
                    "package" => metadata.package_instructions = block_lines,
                    "appimage_icon" => metadata.appimage_icon_instructions = block_lines,
                    "appimage_desktop" => metadata.appimage_desktop_instructions = block_lines,
                    _ => {}
                }
                continue;
            }

            match key {
                "name" => metadata.name = value.to_string(),
                "version" => metadata.version = value.to_string(),
                "release" => metadata.release = value.to_string(),
                "description" => metadata.description = value.to_string(),
                "arch" => metadata.arch = parse_array(value),
                "url" => metadata.url = value.to_string(),
                "license" => metadata.license = value.to_string(),
                "depends" => metadata.depends = parse_array(value),
                "provides" => metadata.provides = parse_array(value),
                "conflicts" => metadata.conflicts = parse_array(value),
                "sources" => metadata.sources = parse_array(value),
                "md5sums" => metadata.md5sums = parse_array(value),
                "sha1sums" => metadata.sha1sums = parse_array(value),
                "sha256sums" => metadata.sha256sums = parse_array(value),
                "sha512sums" => metadata.sha512sums = parse_array(value),
                "build_env" => metadata.build_env = parse_array(value),
                "appimage_exec" => metadata.appimage_exec = value.trim_matches('"').to_string(),
                _ => {}
            }
        }
    }

    if metadata.name.is_empty() {
        return Err("Package name not found".into());
    }

    if metadata.version.is_empty() {
        return Err("Package version not found".into());
    }

    if metadata.description.is_empty() {
        return Err("Package description not found".into());
    }

    if metadata.arch.is_empty() {
        return Err("Package arch not found".into());
    }

    if metadata.url.is_empty() {
        return Err("Package url not found".into());
    }

    if metadata.license.is_empty() {
        return Err("Package license not found".into());
    }

    if metadata.depends.is_empty() {
        return Err("Package depends not found".into());
    }

    if metadata.provides.is_empty() {
        return Err("Package provides not found".into());
    }

    if metadata.conflicts.is_empty() {
        return Err("Package conflicts not found".into());
    }

    if metadata.sources.is_empty() {
        return Err("Package sources not found".into());
    }

    if metadata.md5sums.is_empty()
        && metadata.sha1sums.is_empty()
        && metadata.sha256sums.is_empty()
        && metadata.sha512sums.is_empty()
    {
        return Err(
            "No package checksums found (md5sums, sha1sums, sha256sums, or sha512sums)".into(),
        );
    }

    Ok(metadata)
}

fn parse_array(value: &str) -> Vec<String> {
    value
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.trim().trim_matches('"').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn print_metadata(metadata_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let metadata = extract_metadata(metadata_path)?;

    println!("Package: {}", metadata.name);
    println!("Version: {}", metadata.version);
    println!("Description: {}", metadata.description);
    println!("Arch: {:?}", metadata.arch);
    println!("URL: {}", metadata.url);
    println!("License: {}", metadata.license);
    println!("Depends: {:?}", metadata.depends);
    println!("Provides: {:?}", metadata.provides);
    println!("Conflicts: {:?}", metadata.conflicts);
    println!("Sources: {:?}", metadata.sources);
    println!("SHA256SUMS: {:?}", metadata.sha256sums);
    println!("Build Env: {:?}", metadata.build_env);

    Ok(())
}
