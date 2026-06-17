use std::path::PathBuf;

use clap::{Parser, Subcommand};
use rlb_core::RlbFile;

#[derive(Parser)]
#[command(
    name = "rlb",
    about = "Inspect and edit RLB configuration files",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Dump {
        path: PathBuf,
    },
    Tables {
        path: PathBuf,
    },
    DumpTable {
        path: PathBuf,
        #[arg(long)]
        name: String,
    },
    Validate {
        path: PathBuf,
    },
    Edit {
        path: PathBuf,
        #[arg(long)]
        table: String,
        #[arg(long)]
        entry: usize,
        #[arg(long)]
        field: String,
        #[arg(long)]
        value: i32,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

fn main() -> anyhow::Result<()> {
    match Cli::parse().command {
        Command::Dump { path } => dump(&path),
        Command::Tables { path } => tables(&path),
        Command::DumpTable { path, name } => dump_table(&path, &name),
        Command::Validate { path } => validate(&path),
        Command::Edit {
            path,
            table,
            entry,
            field,
            value,
            out,
        } => edit(&path, &table, entry, &field, value, out),
    }
}

fn dump(path: &PathBuf) -> anyhow::Result<()> {
    let file = RlbFile::load(path)?;
    println!("{}", path.display());
    println!("{} known table(s) discovered:", file.tables().count());
    for table in file.tables() {
        println!("  {:<32} {} entries", table.name, table.entries.len());
    }
    Ok(())
}

fn tables(path: &PathBuf) -> anyhow::Result<()> {
    let file = RlbFile::load(path)?;
    for table in file.tables() {
        println!("{}", table.name);
    }
    Ok(())
}

fn dump_table(path: &PathBuf, name: &str) -> anyhow::Result<()> {
    let file = RlbFile::load(path)?;
    let table = file.table(name)?;

    for (i, entry) in table.entries.iter().enumerate() {
        println!(
            "[{i}] object_id={} chapters={}..{}..{} zone={} area={} position={}",
            entry.object_id,
            entry.minimum_chapter,
            entry.medium_chapter,
            entry.maximum_chapter,
            entry.zone_id,
            entry.area_id,
            entry.position_id
        );
    }
    println!(
        "[terminator] target_script={}",
        table.terminator.target_script
    );
    Ok(())
}

fn validate(path: &PathBuf) -> anyhow::Result<()> {
    match RlbFile::load(path) {
        Ok(_) => println!("{}: valid", path.display()),
        Err(err) => println!("{}: invalid ({err})", path.display()),
    }
    Ok(())
}

fn edit(
    path: &PathBuf,
    table: &str,
    entry: usize,
    field: &str,
    value: i32,
    out: Option<PathBuf>,
) -> anyhow::Result<()> {
    let mut file = RlbFile::load(path)?;
    file.set_entry_field(table, entry, field, value)?;
    let out_path = out.unwrap_or_else(|| path.clone());
    file.save(&out_path)?;
    println!("wrote {}", out_path.display());
    Ok(())
}
