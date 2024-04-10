use serde::Deserialize;

pub fn deserialize_config<'de, T: Deserialize<'de>>(path: &str) -> anyhow::Result<T> {
    let config = config::Config::builder()
        .add_source(config::File::from(std::path::Path::new(path)).required(false))
        .build()?
        .try_deserialize::<T>()?;
    Ok(config)
}
