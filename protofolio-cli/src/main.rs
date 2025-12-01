//! protofolio-cli - CLI tool for generating TypeScript types from AsyncAPI specifications
//!
//! This tool generates TypeScript type definitions from AsyncAPI 3.0 specifications
//! using Modelina.

use clap::{Parser, Subcommand};
use protofolio::AsyncApiSpec;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(name = "protofolio")]
#[command(about = "Generate TypeScript types from AsyncAPI specifications", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate TypeScript types from an AsyncAPI specification file
    Generate {
        /// Path to the AsyncAPI specification file (JSON or YAML)
        #[arg(short, long)]
        spec: PathBuf,

        /// Output directory for generated TypeScript types
        #[arg(short, long, default_value = "./types")]
        output: PathBuf,

        /// Format of the input spec file (auto-detected if not specified)
        #[arg(short, long)]
        format: Option<SpecFormat>,
    },
}

#[derive(Clone, Copy, clap::ValueEnum)]
enum SpecFormat {
    Json,
    Yaml,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = match cli.command {
        Commands::Generate {
            spec,
            output,
            format,
        } => generate_types(&spec, &output, format),
    } {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn generate_types(
    spec_path: &Path,
    output_dir: &Path,
    format: Option<SpecFormat>,
) -> Result<(), Error> {
    // Check if spec file exists
    if !spec_path.exists() {
        return Err(Error::SpecFileNotFound(spec_path.to_path_buf()));
    }

    // Detect format if not specified
    let detected_format = format.unwrap_or_else(|| {
        let ext = spec_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        match ext.as_str() {
            "yaml" | "yml" => SpecFormat::Yaml,
            _ => SpecFormat::Json,
        }
    });

    // Read and parse the spec
    println!(
        "Reading AsyncAPI specification from: {}",
        spec_path.display()
    );
    let spec_content = fs::read_to_string(spec_path)?;
    let spec: AsyncApiSpec = match detected_format {
        SpecFormat::Json => serde_json::from_str(&spec_content)
            .map_err(|e| Error::ParseError(format!("Failed to parse JSON: {}", e)))?,
        SpecFormat::Yaml => serde_yaml_ng::from_str(&spec_content)
            .map_err(|e| Error::ParseError(format!("Failed to parse YAML: {}", e)))?,
    };

    println!("✓ Successfully parsed AsyncAPI specification");
    println!("  Title: {}", spec.info.title);
    println!("  Version: {}", spec.info.version);

    // Create output directory if it doesn't exist
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
        println!("Created output directory: {}", output_dir.display());
    }

    // Write spec to temporary file for Node.js script
    let temp_dir = std::env::temp_dir();
    let temp_spec_file = temp_dir.join("asyncapi-spec.json");
    let spec_json = serde_json::to_string_pretty(&spec)?;
    fs::write(&temp_spec_file, spec_json)?;

    println!("Generating TypeScript types...");

    // Get the path to the Node.js script
    let script_path = get_script_path()?;

    // Run the Node.js script
    let output = Command::new("node")
        .arg(&script_path)
        .arg(&temp_spec_file)
        .arg(output_dir)
        .output()
        .map_err(|e| Error::CommandError(format!("Failed to execute Node.js: {}", e)))?;

    // Clean up temp file
    let _ = fs::remove_file(&temp_spec_file);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::GenerationError(format!(
            "TypeScript generation failed:\n{}",
            stderr
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.is_empty() {
        print!("{}", stdout);
    }

    println!(
        "✓ TypeScript types generated successfully in: {}",
        output_dir.display()
    );

    Ok(())
}

fn get_script_path() -> Result<PathBuf, Error> {
    // Strategy 1: Try relative to current working directory
    let script_path = PathBuf::from("scripts/generate-types.js");
    if script_path.exists() {
        return Ok(script_path);
    }

    // Strategy 2: Try absolute path from current directory
    if let Ok(cwd) = std::env::current_dir() {
        let script_path = cwd.join("scripts").join("generate-types.js");
        if script_path.exists() {
            return Ok(script_path);
        }
    }

    // Strategy 3: Try relative to the executable location
    // This works when the CLI is installed and run from anywhere
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            // For installed binaries, look for scripts in the workspace root
            // The structure is: workspace/scripts/generate-types.js
            // and the binary is in: workspace/target/release/protofolio
            let mut search_dir = exe_dir;
            for _ in 0..5 {
                // Look for scripts directory at this level
                let script_path = search_dir.join("scripts").join("generate-types.js");
                if script_path.exists() {
                    return Ok(script_path);
                }
                // Go up one level
                if let Some(parent) = search_dir.parent() {
                    search_dir = parent;
                } else {
                    break;
                }
            }
        }
    }

    Err(Error::ScriptNotFound)
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Specification file not found: {0}")]
    SpecFileNotFound(PathBuf),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Command execution error: {0}")]
    CommandError(String),

    #[error("TypeScript generation error: {0}")]
    GenerationError(String),

    #[error(
        "Could not find generate-types.js script. Please ensure scripts/generate-types.js exists."
    )]
    ScriptNotFound,
}
