use std::collections::HashMap;
use std::error::Error;

use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use url;

use tilelive::source::Source;
use tilelive::tile::Tile;

pub type PostgresPool = Pool<PostgresConnectionManager>;

static DEFAULT_EXTENT: u32 = 4096;
static DEFAULT_BUFFER: u32 = 64;
static DEFAULT_CLIP_GEOM: bool = true;

#[derive(Debug)]
pub struct PostGIS {
  pool: PostgresPool,
  query: String,
}

impl Source for PostGIS {
  fn load(uri: &str) -> Result<Self, url::ParseError> {
    let uri = url::Url::parse(uri)?;
    let params: HashMap<_, _> = uri.query_pairs().collect();

    let mut uri = uri.clone();
    uri.set_query(None);
    let conn_str = uri.as_str();

    let manager = PostgresConnectionManager::new(conn_str, TlsMode::None).unwrap();
    let pool = Pool::builder().build(manager).unwrap();

    let schema = params.get("schema").unwrap().to_string();
    let table = params.get("table").unwrap().to_string();
    let geometry_column = params.get("geometry_column").unwrap().to_string();
    let id = format!("{}.{}", schema, table);

    let query = format!(
      include_str!("get_tile.sql"),
      id = id,
      geometry_column = geometry_column,
      extent = DEFAULT_EXTENT,
      buffer = DEFAULT_BUFFER,
      clip_geom = DEFAULT_CLIP_GEOM,
    );

    Ok(PostGIS { pool, query })
  }

  fn get_tile(&self, z: u8, x: u32, y: u32) -> std::io::Result<Tile> {
    let conn = self
      .pool
      .get()
      .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.description()))?;

    let stmt = conn.prepare(&self.query)?;

    let tile: Result<Tile, _> = stmt
      .query(&[&(z as i32), &(x as i32), &(y as i32)])
      .map(|rows| rows.get(0).get("st_asmvt"))
      .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.description()));

    tile
  }
}
