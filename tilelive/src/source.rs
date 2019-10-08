use std::io;
use tilejson::TileJSON;

use crate::tile::Tile;

pub trait Source {
  fn load(uri: &str) -> io::Result<Self>
  where
    Self: std::marker::Sized;

  fn info(&self) -> io::Result<TileJSON>;

  fn get_tile(&self, z: u8, x: u32, y: u32) -> io::Result<Tile>;
}
