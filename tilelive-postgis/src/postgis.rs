use std::collections::HashMap;
use std::error::Error;
use std::io;

use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use url;

use tilelive_core::source::Source;
use tilelive_core::tile::Tile;
use tilelive_core::tilejson::{TileJSON, TileJSONBuilder};

pub type PostgresPool = Pool<PostgresConnectionManager>;

static DEFAULT_EXTENT: u32 = 4096;
static DEFAULT_BUFFER: u32 = 64;
static DEFAULT_CLIP_GEOM: bool = true;

#[derive(Clone, Debug)]
pub struct PostGIS {
  pool: PostgresPool,
  id: String,
  query: String,
}

impl Source for PostGIS {
  fn load(uri: &str) -> io::Result<Self> {
    let uri = url::Url::parse(uri)
      .map_err(|err| io::Error::new(io::ErrorKind::Other, err.description()))?;

    let params: HashMap<_, _> = uri.query_pairs().collect();

    let mut uri = uri.clone();
    uri.set_query(None);
    let conn_str = uri.as_str();

    let manager = PostgresConnectionManager::new(conn_str, TlsMode::None)?;

    let pool = Pool::builder()
      .build(manager)
      .map_err(|err| io::Error::new(io::ErrorKind::Other, err.description()))?;

    let schema = params.get("schema").unwrap().to_string();
    let table = params.get("table").unwrap().to_string();
    let id = format!("{}.{}", schema, table);
    let geometry_column = params.get("geometry_column").unwrap().to_string();

    let query = format!(
      include_str!("get_tile.sql"),
      id = id,
      geometry_column = geometry_column,
      extent = DEFAULT_EXTENT,
      buffer = DEFAULT_BUFFER,
      clip_geom = DEFAULT_CLIP_GEOM,
    );

    Ok(PostGIS { pool, id, query })
  }

  fn info(&self) -> io::Result<TileJSON> {
    let mut tilejson_builder = TileJSONBuilder::new();

    tilejson_builder.id(&self.id);
    tilejson_builder.scheme("tms");
    tilejson_builder.name(&self.id);
    tilejson_builder.tiles(vec![]);

    Ok(tilejson_builder.finalize())
  }

  fn get_tile(&self, z: u8, x: u32, y: u32) -> io::Result<Tile> {
    let conn = self
      .pool
      .get()
      .map_err(|err| io::Error::new(io::ErrorKind::Other, err.description()))?;

    let stmt = conn.prepare(&self.query)?;

    let tile: io::Result<Tile> = stmt
      .query(&[&(z as i32), &(x as i32), &(y as i32)])
      .map(|rows| rows.get(0).get("st_asmvt"))
      .map_err(|err| io::Error::new(io::ErrorKind::Other, err.description()));

    tile
  }
}
