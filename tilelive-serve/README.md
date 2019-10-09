# tilelive-serve

Simple tilelive server

## Usage

```shell
tilelive-serve "postgres://postgres@localhost/db?schema=public&table=points&geometry_column=geom"
```

## Development

```shell
systemfd --no-pid -s http::3000 -- cargo watch -x 'run -p tilelive-serve <uri>...'
```
