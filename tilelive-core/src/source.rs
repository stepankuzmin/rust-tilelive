use std::io;
use tilejson::TileJSON;

use crate::tile::Tile;

pub trait Source: SourceClone + Send + Sync {
  fn load(uri: &str) -> io::Result<Self>
  where
    Self: Sized;

  fn info(&self) -> io::Result<TileJSON>;

  fn get_tile(&self, z: u8, x: u32, y: u32) -> io::Result<Tile>;
}

pub trait SourceClone {
  fn clone_box(&self) -> Box<dyn Source>;
}

impl<T> SourceClone for T
where
  T: 'static + Source + Clone,
{
  fn clone_box(&self) -> Box<dyn Source> {
    Box::new(self.clone())
  }
}

impl Clone for Box<dyn Source> {
  fn clone(&self) -> Box<dyn Source> {
    self.clone_box()
  }
}
