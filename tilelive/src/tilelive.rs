use std::collections::HashMap;
use std::error::Error;
use std::io;

use url;

use tilelive_core::Source;
use tilelive_postgis::postgis::PostGIS;

#[derive(Clone)]
pub struct Tilelive {
  sources: HashMap<String, Box<dyn Source>>,
}

impl Tilelive {
  pub fn new() -> Self {
    Tilelive {
      sources: HashMap::new(),
    }
  }

  pub fn load(&mut self, uri: &str) -> io::Result<()> {
    let uri = url::Url::parse(uri)
      .map_err(|err| io::Error::new(io::ErrorKind::Other, err.description()))?;

    let source = match uri.scheme() {
      "postgres" => PostGIS::load(uri.as_str()),
      _ => io::Result::Err(io::Error::new(io::ErrorKind::Other, "Unknown scheme")),
    };

    let source = source.map_err(|err| io::Error::new(io::ErrorKind::Other, err.description()))?;

    let info = source.info()?;

    let id = info
      .id
      .ok_or(io::Error::new(io::ErrorKind::Other, "Source id not found"))?;

    self.sources.insert(id, Box::new(source));

    io::Result::Ok(())
  }

  pub fn find_id(&self, source_id: &str) -> Option<&Box<dyn Source>> {
    self.sources.get(source_id)
  }
}
