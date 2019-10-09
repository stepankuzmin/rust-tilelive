use actix_web::{web, Error, HttpResponse, Responder};
use futures::Future;
use serde::Deserialize;

use tilelive::Tilelive;

#[derive(Deserialize)]
struct InfoRequest {
  source_id: String,
}

fn get_info(tl: web::Data<Tilelive>, path: web::Path<InfoRequest>) -> impl Responder {
  let source = tl.find_id(&path.source_id).unwrap();
  let info = source.info().unwrap();
  HttpResponse::Ok().json(info)
}

#[derive(Deserialize)]
struct TileRequest {
  source_id: String,
  z: u8,
  x: u32,
  y: u32,
  #[allow(dead_code)]
  format: String,
}

fn get_tile(
  tl: web::Data<Tilelive>,
  path: web::Path<TileRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
  web::block(move || {
    let source = tl.find_id(&path.source_id).unwrap();
    let tile = source.get_tile(path.z, path.x, path.y).unwrap();
    Ok(tile) as Result<_, std::io::Error>
  })
  .then(|res| match res {
    Ok(now) => Ok(HttpResponse::Ok().body(now)),
    Err(_) => Ok(HttpResponse::InternalServerError().into()),
  })
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .route("/{source_id}.json", web::get().to(get_info))
    .route(
      "/{source_id}/{z}/{x}/{y}.{format}",
      web::get().to_async(get_tile),
    );
}
