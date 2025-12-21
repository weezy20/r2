use clap::Parser;
use ron::value::Value;
use serde::Serialize;
use std::borrow::Cow;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Parser)]
#[command(author, version, about = "Converts RON to JSON/YAML/TOML")]
struct Args {
    /// The input .ron file to convert
    input: PathBuf,

    /// The output path for converted file. If not specified, the input file name with appropriate extension will be used.
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Overwrite the output file if it exists
    #[arg(short, long, default_value_t = false)]
    force: bool,

    /// Output type. Default is json. Supports json, yaml, yml, toml.
    #[arg(short = 't', long = "type", default_value = "json")]
    output_type: OutputType,
}

#[derive(Debug, Clone)]
enum OutputType {
    Json,
    Yaml,
    Toml,
}

impl FromStr for OutputType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputType::Json),
            "yaml" | "yml" => Ok(OutputType::Yaml),
            "toml" => Ok(OutputType::Toml),
            _ => Err(format!(
                "Invalid output type: {}. Supported types are: json, yaml, yml, toml",
                s
            )),
        }
    }
}

impl std::fmt::Display for OutputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputType::Json => write!(f, "json"),
            OutputType::Yaml => write!(f, "yaml"),
            OutputType::Toml => write!(f, "toml"),
        }
    }
}

impl OutputType {
    fn extension(&self) -> &str {
        match self {
            OutputType::Json => "json",
            OutputType::Yaml => "yaml",
            OutputType::Toml => "toml",
        }
    }
}

fn main() {
    let args = Args::parse();

    if !args.input.try_exists().unwrap_or(false) {
        println!("Input file does not exist");
        std::process::exit(1);
    }

    // Determine output path
    let output_path = match args.output.as_ref() {
        Some(output) => {
            // Check if the provided path is a directory
            if output.is_dir() {
                // Use the input file's stem with the new extension
                let file_stem = args.input.file_stem().expect("Input file has no file stem");
                let mut new_path = output.join(file_stem);
                new_path.set_extension(args.output_type.extension());
                new_path
            } else {
                let file_stem = output.file_stem().expect("Output file has no file stem");
                let mut new_path = PathBuf::from(file_stem);
                new_path.set_extension(args.output_type.extension());
                new_path
            }
        }
        None => {
            // Use file stem from input and apply the appropriate extension
            let file_stem = args.input.file_stem().expect("Input file has no file stem");
            let parent: Cow<'_, Path> = match args.input.parent() {
                Some(p) => Cow::Borrowed(p),
                None => {
                    Cow::Owned(std::env::current_dir().expect("Failed to get current directory"))
                }
            };
            let mut path = parent.join(file_stem);
            path.set_extension(args.output_type.extension());
            path
        }
    };

    // Check if input and output are the same
    if output_path == args.input.as_path() {
        eprintln!(
            "Error: Input and output files are the same. Please specify a different output file with -o."
        );
        std::process::exit(1);
    }

    // Check if output exists (unless force is enabled)
    if !args.force && output_path.exists() {
        eprintln!(
            "Error: The file '{}' already exists. Use --force/-f to overwrite.",
            output_path.display()
        );
        std::process::exit(1);
    }

    // Read RON file
    let data = std::fs::read_to_string(&args.input).unwrap_or_else(|e| {
        eprintln!(
            "Error: Failed to read input file '{}': {}",
            args.input.display(),
            e
        );
        std::process::exit(1);
    });

    // Parse RON
    let value: Value = data.parse().unwrap_or_else(|e| {
        eprintln!("Error: Failed to parse RON: {}", e);
        std::process::exit(1);
    });

    // Open output file
    let mut output_file = if args.force {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&output_path)
    } else {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&output_path)
    }
    .unwrap_or_else(|e| {
        eprintln!(
            "Error: Failed to open output file '{}': {}",
            output_path.display(),
            e
        );
        std::process::exit(1);
    });

    // Serialize to the appropriate format
    match args.output_type {
        OutputType::Json => {
            let mut ser = serde_json::Serializer::pretty(&mut output_file);
            value.serialize(&mut ser).unwrap_or_else(|e| {
                eprintln!("Error serializing to JSON: {}", e);
                std::process::exit(1);
            });
        }
        OutputType::Yaml => match serde_yaml_bw::to_writer(&mut output_file, &value) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error serializing to YAML: {}", e);
                std::process::exit(1);
            }
        },
        OutputType::Toml => {
            let toml_string = toml::to_string_pretty(&value).unwrap_or_else(|e| {
                eprintln!("Error: Failed to serialize to TOML string: {}", e);
                std::process::exit(1);
            });
            output_file
                .write_all(toml_string.as_bytes())
                .unwrap_or_else(|e| {
                    eprintln!("Error writing TOML to file: {}", e);
                    std::process::exit(1);
                });
        }
    }

    println!("Converted to {}", output_path.display());
}
