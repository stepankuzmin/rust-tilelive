use url;

use crate::tile::Tile;

pub trait Source {
  fn load(uri: &str) -> Result<Self, url::ParseError>
  where
    Self: std::marker::Sized;

  fn get_tile(&self, z: u8, x: u32, y: u32) -> std::io::Result<Tile>;
}
