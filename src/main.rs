use anyhow::{bail, Context, Result};
use clap::Parser;
use digest::DynDigest;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};
use std::io::{Read, Write};

//use tar::Archive;

/// A simple SNTP (RFC 5905) and RFC 868 client written in Rust.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// File to process. If missing or set as -, read stdin.
    input: Option<String>,

    /// Save signatures to file
    #[arg(short, long)]
    output: Option<String>,

    /// Select a checksum algorithm (md5, sha1, sha256, sha384, sha512).
    #[arg(short, long, default_value = "sha256")]
    checksum: String,

    /// Output a zero byte (ASCII NUL) at the end of each line, rather than a newline. This option
    /// enables other programs to parse the output even when that output would contain data with
    /// embedded newlines. Also file name escaping is not used.
    #[arg(short, long)]
    zero: bool,
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}

/// Escape newlines and carriage returns
fn escape_filename(name: &[u8]) -> Vec<u8> {
    let mut escaped = Vec::<u8>::with_capacity(name.len());
    for x in name {
        match x {
            b'\\' => escaped.extend_from_slice(&[b'\\', b'\\']),
            b'\n' => escaped.extend_from_slice(&[b'\\', b'n']),
            b'\r' => escaped.extend_from_slice(&[b'\\', b'r']),
            _ => escaped.push(*x),
        };

    };
    escaped
}

fn should_escape(name: String) -> bool {
    name.find("\n") != None || name.find("\r") != None
}

fn hash_entry(entry: &mut tar::Entry<impl Read>, hasher: &mut Box<dyn DigestWrite>) -> Box<[u8]> {
    std::io::copy(entry, hasher).unwrap();
    hasher.finalize_reset()
}

trait DigestWrite: DynDigest + Write {}
impl<T: DynDigest + Write> DigestWrite for T {}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let (mut stdin, mut file);
    let input: &mut dyn Read = match cli.input.as_deref() {
        None | Some("-") => {
            stdin = std::io::stdin();
            &mut stdin
        }
        Some(path) => {
            file = std::fs::File::open(path)
                .with_context(|| format!("Failed to open input file {}", path))?;
            &mut file
        }
    };
    let (mut stdout, mut file);
    let output: &mut dyn Write = match cli.output.as_deref() {
        None | Some("-") => {
            stdout = std::io::stdout();
            &mut stdout
        }
        Some(path) => {
            file = std::fs::File::create(path)
                .with_context(|| format!("Failed to open output file {}", path))?;
            &mut file
        }
    };

    let mut archive = tar::Archive::new(input);

    let mut hasher: Box<dyn DigestWrite> = match cli.checksum.as_str() {
        "md5" => Box::new(Md5::default()),
        "sha1" => Box::new(Sha1::default()),
        "sha256" => Box::new(Sha256::default()),
        "sha384" => Box::new(Sha384::default()),
        "sha512" => Box::new(Sha512::default()),
        _ => bail!("Wrong checksum {}", cli.checksum),
    };

    let line_separator = if cli.zero {b"\0"}  else {b"\n"};

    for entry in archive
        .entries()
        .with_context(|| format!("Failed to parse input file"))?
    {
        let mut entry = entry.with_context(|| format!("Failed to parse input file"))?;

        // We skip all entries which are not regular files
        if !entry.header().entry_type().is_file() {
            continue;
        }

        let hash = hex::encode(hash_entry(&mut entry, &mut hasher));

        let entry_name = entry.path_bytes();
        let mut escaped = false;
        if !cli.zero && should_escape(String::from_utf8(entry_name.to_vec())?) {
            output.write(b"\\").unwrap();
            escaped = true;
        }
        output.write(hash.as_bytes()).unwrap();
        output.write(b"  ").unwrap();

        if escaped {
            output.write(&escape_filename(&entry.path_bytes())).unwrap();
        } else {
            output.write(&entry.path_bytes()).unwrap();
        }
        output.write(line_separator).unwrap();
    }

    Ok(())
}
