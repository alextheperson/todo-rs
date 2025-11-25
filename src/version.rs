#![allow(dead_code)]

pub const PACKAGE_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
pub const LICENSE: &'static str = env!("CARGO_PKG_LICENSE");
pub const HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");
pub const TAG_NAME: &'static str = env!("GIT_VERSION");
pub const COMMIT_HASH: &'static str = env!("GIT_HASH");

pub const VERSION: &'static str = concat!(env!("GIT_VERSION"), " (", env!("GIT_HASH"), ")");

pub const LONG_VERSION: &'static str = concat!(
    "\n",
    env!("GIT_VERSION"),
    " / ",
    env!("GIT_HASH"),
    "\n\nAuthors: ",
    env!("CARGO_PKG_AUTHORS"),
    "\n",
    "Source: ",
    env!("CARGO_PKG_REPOSITORY"),
    "\n\n",
    env!("CARGO_PKG_LICENSE"),
);
