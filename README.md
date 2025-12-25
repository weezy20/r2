# R2 (Ron2 - Json/Yaml/Toml & vice-versa)

`r2` is a program inter-convert `ron` <> `[json | yaml | toml]` files. Usually languages outside rust don't have a native parser for `ron` files as `ron` is designed with Rust in mind. `r2` steps in to allow you to use `ron` files in projects that don't understand `ron`. You can also use `r2` to convert a `ron` file out of `json`, `yaml`, or `toml` files.

The following crates enable transcoding to the supported target formats:
- json: [`serde_json`](https://github.com/serde-rs/json)
- yaml: [`serde_yaml_bw`](https://github.com/bourumir-wyngs/serde-yaml-bw)
- toml: [`toml-rs/toml`](https://github.com/toml-rs/toml)
- ron: [`ron-rs/ron`](https://github.com/ron-rs/ron)

### Installation

#### From crates.io

```sh
cargo install ron2json # installs an executable named r2
```

### Usage:
```sh
r2 <path/to/file> <?options>
# --type   | -t json|yaml|toml|yml|ron - specify output type
# --output | -o <path> - specify output path. Path may be a directory or file name.
# --force  | -f - overwrite existing files. If -o <path> exists and is a file pass -f to overwrite. Default is disabled so overwrites fail.
```

#### Examples:

```sh
r2 test.ron                      # Creates test.json in current working directory
r2 test.ron -o foo               # Creates foo.json in current working directory otherwise if directory foo exists, creates foo/test.json (Preserving name from ron file, treating -o <arg> as directory)
r2 test.ron -t yml               # Creates test.yml in current working directory. Can also be specified as -t yaml
r2 test.ron -o test.toml -t toml # Creates test.toml in current working directory
r2 test.json -t ron              # Creates test.ron from test.json
```
