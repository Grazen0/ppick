use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use clap::CommandFactory;
use clap_complete::Shell;
use cli::Args;

#[path = "src/cli.rs"]
mod cli;

fn generate_man_pages(out_dir: &Path, bin: &str) -> io::Result<()> {
    let out_dir = out_dir.join("man");

    let cmd = Args::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buf = vec![];
    man.render(&mut buf)?;

    fs::create_dir_all(&out_dir)?;
    fs::write(out_dir.join(format!("{bin}.1")), buf)?;
    Ok(())
}

fn generate_completions(out_dir: &Path, bin: &str) -> io::Result<()> {
    let out_dir = out_dir.join("completions");

    const SHELLS: [Shell; 5] = [
        Shell::Bash,
        Shell::Fish,
        Shell::Zsh,
        Shell::Elvish,
        Shell::PowerShell,
    ];

    let mut cmd = Args::command();

    fs::create_dir_all(&out_dir)?;
    for sh in SHELLS {
        clap_complete::generate_to(sh, &mut cmd, bin, &out_dir)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    println!("cargo::rerun-if-env-changed=PPICK_GEN_COMPLETIONS");
    println!("cargo::rerun-if-env-changed=PPICK_GEN_MAN_PAGES");
    println!("cargo::rerun-if-changed=src");

    let bin = env!("CARGO_PKG_NAME");
    let out_dir = PathBuf::from("out");

    if env::var_os("PPICK_GEN_MAN_PAGES").is_some() {
        generate_man_pages(&out_dir, bin)?;
    }

    if env::var_os("PPICK_GEN_COMPLETIONS").is_some() {
        generate_completions(&out_dir, bin)?;
    }

    Ok(())
}
