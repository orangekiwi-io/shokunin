// Copyright © 2023 Shokunin (職人) Static Site Generator. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//!
//! # Shokunin (職人) Static Site Generator
//!
//! [![Shokunin (職人) Static Site Generator Logo](https://kura.pro/shokunin/images/logos/shokunin.svg)](https://shokunin.one "Shokunin - A Fast and Flexible Static Site Generator written in Rust")
//!
//! A Fast and Flexible Static Site Generator written in Rust 🦀
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org "Rust")
//! [![Crates.io](https://img.shields.io/crates/v/ssg.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/ssg "Crates.io")
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.12-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/ssg "Lib.rs")
//! [![License](https://img.shields.io/crates/l/ssg.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](https://opensource.org/license/apache-2-0/ "MIT or Apache License, Version 2.0")
//!
//! ## Overview 📖
//!
//! `Shokunin (職人) Static Site Generator` is a highly-optimized, Rust-based static site generator (ssg) that aims to provide an easy-to-use and powerful tool for building professional static websites and blogs.
//!
//! The library extracts metadata and content to generate static HTML files from Markdown, YAML, JSON, and TOML. It also supports HTML themes and custom templates to help you create high quality websites with ease.
//!
//! ## Features ✨
//!
//! - Blazing fast and flexible
//! - Easy to use
//! - Written in Rust 🦀
//! - Supports multiple content formats (Markdown, YAML, JSON, TOML)
//! - Compatible with various HTML themes and Premium templates to
//!   create accessible websites quickly and efficiently
//! - Generates minified HTML and JSON versions for optimal performance
//! - Built-in Rust development server with live reloading
//!
//! ## Getting Started 🚀
//!
//! It takes just a few minutes to get up and running with `Shokunin (職人) Static Site Generator`.
//!
//! ### Installation
//!
//! To install `Shokunin (職人) Static Site Generator`, you need to have the Rust toolchain installed on your machine. You can install the Rust toolchain by following the instructions on the [Rust website](https://www.rust-lang.org/learn/get-started).
//!
//! Once you have the Rust toolchain installed, you can install `Shokunin (職人) Static Site Generator` using the following command:
//!
//! ```shell
//! cargo install ssg
//! ```
//!
//! For simplicity, we have given `Shokunin (職人) Static Site Generator` a simple alias `ssg` which can stand for `Shokunin Site Generator` or `Static Site Generator`.
//!
//! You can then run the help command to see the available options:
//!
//! ```shell
//! ssg --help
//! ```
//!
//! ## Examples and Usage 📚
//!
//! Check out the examples folder for helpful snippets of code that demonstrate how to use the `Shokunin (職人) Static Site Generator` library. You can also check out the [documentation](https://docs.rs/ssg) for more information on how to use the library.
//!
//! ## License 📜
//!
//! The project is licensed under the terms of both the MIT license and the Apache License (Version 2.0).
//!
//! - [Apache License, Version 2.0](https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0")
//! - [MIT license](http://opensource.org/licenses/MIT "MIT license")
//!

#![forbid(unsafe_code)]
#![forbid(unreachable_pub)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![doc(
    html_favicon_url = "https://kura.pro/shokunin/images/favicon.ico",
    html_logo_url = "https://kura.pro/shokunin/images/logos/shokunin.svg",
    html_root_url = "https://docs.rs/ssg"
)]
#![crate_name = "ssg"]
#![crate_type = "lib"]

use cli::print_banner;
use compiler::compile;
use std::{error::Error, path::Path};

/// The `cli` module contains functions for the command-line interface.
pub mod cli;
/// The `compiler` module contains functions for the compilation process.
pub mod compiler;
/// The `file` module handles file reading and writing operations.
pub mod file;
/// The `frontmatter` module extracts the front matter from files.
pub mod frontmatter;
/// The `html` module generates the HTML content.
pub mod html;
/// The `json` module generates the JSON content.
pub mod json;
/// The `macros` module contains functions for generating macros.
pub mod macros;
/// The `metatags` module generates the meta tags.
pub mod metatags;
/// The `navigation` module generates the navigation menu.
pub mod navigation;
/// The `parser` module contains functions for parsing command-line
/// arguments and options.
pub mod process;
/// The `rss` module generates the RSS content.
pub mod rss;
/// The `serve` module contains functions for the development server.
pub mod serve;
/// The `template` module renders the HTML content using the pre-defined
/// template.
pub mod template;
/// The `directory` function ensures that a directory exists.
pub mod utilities;

#[allow(non_camel_case_types)]

/// ## Function: `run` - Runs the static site generator command-line tool.
///
/// This function prints a banner containing the title and description of the tool,
/// and then processes any command-line arguments passed to it. If no
/// arguments are passed, it prints a welcome message and instructions
/// on how to use the tool.
///
/// The function uses the `build` function from the `cli` module to
/// create the command-line interface for the tool. It then processes
/// any arguments passed to it using the `parser` function from the
/// `args` module.
///
/// If any errors occur during the process (e.g. an invalid argument is
/// passed), an error message is printed and returned. Otherwise,
/// `Ok(())` is returned.
pub fn run() -> Result<(), Box<dyn Error>> {
    // Print the CLI banner and welcome message
    print_banner();

    // Build the CLI and parse the arguments
    let matches = cli::build()?;
    process::args(&matches)?;

    if let Some(site_name) = matches.get_one::<String>("new") {
        // Start the server using the specified server address and site name.
        // If an error occurs, propagate it up the call stack.
        serve::start("127.0.0.1:8000", site_name)?;
    }

    // Set the build, content, site and template paths for the compile function.
    let build_path = Path::new("public");
    let content_path = Path::new("content");
    let site_path = Path::new("site");
    let template_path = Path::new("templates");

    // Call the compile function with the above parameters to compile the site.
    compile(build_path, content_path, site_path, template_path)?;

    Ok(())
}
