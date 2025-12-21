# R2 (Ron2 - Json/Yaml/Toml)

A single file that lets you convert and use `ron` files with programs that don't understand `ron`. You can use `r2` to generate a `toml`, `yaml` or `json` until `ron` becomes widely adopted. 

The following crates enable transcoding to the supported target formats:
- json: [`serde_json`](https://github.com/serde-rs/json)
- yaml: [`serde_yaml_bw`](https://github.com/bourumir-wyngs/serde-yaml-bw)
- toml: [`toml-rs/toml`](https://github.com/toml-rs/toml)

### Installation

#### From crates.io

```sh
cargo install ron2json
```

### Usage:
```sh
r2 <path/to/ron/file> <?options>
# --type   | -t json|yaml|toml|yml - specify output type
# --output | -o <path> - specify output path. Path maybe directory or file name or path to file.
# --force  | -f - overwrite existing files. If -o <path> exists and is a file pass -f to overwrite. Default is disabled so overwrites fail.
```

#### Examples:

```sh
r2 test.ron                      # Creates test.json in current working directory
r2 test.ron -o foo               # Creates foo.json in current working directory otherwise if directory foo exists, creates foo/test.json (Preserving name from ron file, treating -o <arg> as directory)
r2 test.ron -t yml               # Creates test.yaml in current working directory. Can also be specified as -t yaml
r2 test.ron -o test.toml -t toml # Creates test.toml in current working directory
```
