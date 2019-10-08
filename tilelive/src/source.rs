use std::io::Result;

use crate::tile::Tile;

pub trait Source {
  fn load(uri: &str) -> Result<Self>
  where
    Self: std::marker::Sized;

  fn get_tile(&self, z: u8, x: u32, y: u32) -> Result<Tile>;
}
