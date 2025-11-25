use clap::ValueEnum;
use clap_complete::{Shell, generate_to};
use clap_mangen::Man;
use std::env;
use std::path::PathBuf;

include!("./src/commands.rs");

fn main() {
    let git_describe = std::process::Command::new("git")
        .args(["describe", "--always", "--tags", "--dirty"])
        .output()
        .expect("Could not get version info from git.");

    let git_version =
        std::str::from_utf8(&git_describe.stdout).expect("Couldn't parse git version output.");

    let cargo_version = env!("CARGO_PKG_VERSION");

    assert!(
        git_version.contains(cargo_version),
        "=> Crate version and Git version do not match."
    );

    println!("cargo::rustc-env=GIT_VERSION={}", git_version);

    let git_revparse = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("Could not get commit hash info from git.");

    let git_commit_hash = &std::str::from_utf8(&git_revparse.stdout)
        .expect("Couldn't parse git version output.")
        .to_string()[0..7];

    println!("cargo::rustc-env=GIT_HASH={}", git_commit_hash);

    let Some(outdir) = env::var_os("OUT_DIR") else {
        return;
    };

    let mut cmd = build();
    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, "todo", &outdir)
            .expect("Could not generate completion scripts.");
    }

    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)
        .expect("Could not generate the manpage.");

    std::fs::write(PathBuf::from(&outdir).join("todo.1"), &buffer)
        .expect("Could not write todo.1 manpage.");
    std::fs::write(PathBuf::from(&outdir).join("todo-rs.1"), &buffer)
        .expect("Could not write todo-rs.1 manpage.");
}
