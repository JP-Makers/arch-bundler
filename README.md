format!("{}-{}", metadata.name, metadata.version)

alpm_build(&metadata).map_err(|e| format!("{:?}", e))?;