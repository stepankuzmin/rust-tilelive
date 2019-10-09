SELECT ST_AsMVT(tile, '{id}', {extent}, 'geom') FROM (
  SELECT
    ST_AsMVTGeom(
      ST_Transform({geometry_column}, 3857),
      ST_TileEnvelope($1, $2, $3),
      {extent},
      {buffer},
      {clip_geom}
    ) AS geom
  FROM {id}
  -- WHERE ST_Intersects({geometry_column}, ST_Transform(ST_TileEnvelope($1, $2, $3), 4326))
) AS tile WHERE geom IS NOT NULL