// Copyright © 2023 Shokunin (職人) Static Site Generator. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate regex;
use minify_html::{minify, Cfg};
use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};
use regex::Regex;
use std::{
    borrow::Cow,
    error::Error,
    fs::{self, File},
    io::{self, Cursor, Write},
    path::{Path, PathBuf},
};

/// Ensures a directory exists, creating it if necessary.
///
/// This function takes a reference to a `Path` object for a directory and a human-readable name for the directory,
/// and creates the directory if it does not already exist.
///
/// # Arguments
///
/// * `dir` - A reference to a `Path` object for the directory.
/// * `name` - A human-readable name for the directory, used in error messages.
///
/// # Returns
///
/// * `Result<(), String>` - A result indicating success or failure.
///     - `Ok(())` if the directory exists or was created successfully.
///     - `Err(String)` if the directory does not exist and could not be created.
///
/// # Example
///
/// ```
/// use ssg::utilities::directory;
/// use std::path::Path;
/// use std::fs;
///
/// // Create a "logs" directory if it doesn't exist
/// let dir = Path::new("logs");
/// directory(dir, "logs").expect("Could not create logs directory");
/// fs::remove_dir_all(dir).expect("Could not remove logs directory");
/// ```
pub fn directory(dir: &Path, name: &str) -> Result<String, String> {
    if dir.exists() {
        if !dir.is_dir() {
            return Err(format!(
                "❌ Error: {} is not a directory.",
                name
            ));
        }
    } else {
        match fs::create_dir_all(dir) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!(
                    "❌ Error: Cannot create {} directory: {}",
                    name, e
                ))
            }
        }
    }
    Ok(String::new())
}

/// Moves the output directory to the public directory.
///
/// This function takes a reference to a `Path` object for the output directory and a string for the site name,
/// and moves the output directory to the public directory.
///
/// # Arguments
///
/// * `site_name` - A string for the site name.
/// * `out_dir` - A reference to a `Path` object for the output directory.
///
/// # Returns
///
/// * `Result<(), std::io::Error>` - A result indicating success or failure.
///     - `Ok(())` if the output directory was moved successfully.
///     - `Err(std::io::Error)` if the output directory could not be moved.
///
pub fn move_output_directory(
    site_name: &str,
    out_dir: &Path,
) -> std::io::Result<()> {
    println!("❯ Moving output directory...");

    let public_dir = Path::new("public");

    if public_dir.exists() {
        fs::remove_dir_all(public_dir)?;
    }

    fs::create_dir(public_dir)?;

    let site_name = site_name.replace(' ', "_");
    let new_project_dir = public_dir.join(site_name);
    fs::create_dir_all(&new_project_dir)?;

    fs::rename(out_dir, &new_project_dir)?;

    println!("  Done.\n");

    Ok(())
}

/// Minifies HTML files in the output directory.
///
/// This function takes a reference to a `Path` object for the output directory and minifies all HTML files in the output directory.
///
/// # Arguments
///
/// * `out_dir` - A reference to a `Path` object for the output directory.
///
/// # Returns
///
/// * `Result<(), std::io::Error>` - A result indicating success or failure.
///     - `Ok(())` if all HTML files were minified successfully.
///     - `Err(std::io::Error)` if any HTML files could not be minified.
///
pub fn minify_html_files(out_dir: &Path) -> io::Result<()> {
    let html_files = find_html_files(out_dir)?;

    for file in &html_files {
        let minified_html = minify_html(file)?;
        let backup_path = backup_file(file)?;
        write_minified_html(file, &minified_html)?;
        println!(
            "Minified HTML file '{}' to '{}'",
            file.display(),
            backup_path.display()
        );
    }

    Ok(())
}

/// Finds all HTML files in a directory.
///
/// This function takes a reference to a `Path` object for a directory and returns a vector of `PathBuf` objects
/// for all HTML files in the directory and its subdirectories.
///
/// # Arguments
///
/// * `dir` - A reference to a `Path` object for the directory.
///
/// # Returns
///
/// * `Result<Vec<PathBuf>, std::io::Error>` - A result containing a vector of `PathBuf` objects for all HTML files
///   in the directory and its subdirectories.
///     - `Ok(Vec<PathBuf>)` if the directory exists and contains HTML files.
///     - `Err(std::io::Error)` if the directory does not exist or does not contain HTML files.
///
pub fn find_html_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut html_files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;

        if entry.path().is_dir() {
            let sub_html_files = find_html_files(&entry.path())?;
            html_files.extend(sub_html_files);
        } else if let Some(extension) =
            entry.path().extension().and_then(|ext| ext.to_str())
        {
            if extension.eq_ignore_ascii_case("html") {
                html_files.push(entry.path());
            }
        }
    }

    Ok(html_files)
}

/// Minifies a single HTML file.
///
/// This function takes a reference to a `Path` object for an HTML file and returns a string containing the minified HTML.
///
/// # Arguments
///
/// * `file_path` - A reference to a `Path` object for the HTML file.
///
/// # Returns
///
/// * `Result<String, std::io::Error>` - A result containing a string containing the minified HTML.
///     - `Ok(String)` if the HTML file was minified successfully.
///     - `Err(std::io::Error)` if the HTML file could not be minified.
///
pub fn minify_html(file_path: &Path) -> io::Result<String> {
    let mut cfg = Cfg::new();
    cfg.do_not_minify_doctype = true;
    cfg.ensure_spec_compliant_unquoted_attribute_values = true;
    cfg.keep_closing_tags = true;
    cfg.keep_html_and_head_opening_tags = true;
    cfg.keep_spaces_between_attributes = true;
    cfg.keep_comments = false;
    cfg.minify_css = true;
    cfg.minify_js = true;
    cfg.remove_bangs = true;
    cfg.remove_processing_instructions = true;
    let file_content = fs::read(file_path)?;
    let minified_content = minify(&file_content, &cfg);

    String::from_utf8(minified_content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// Creates a backup of a file.
///
/// This function takes a reference to a `Path` object for a file and creates a backup of the file with the extension ".src.html".
///
/// # Arguments
///
/// * `file_path` - A reference to a `Path` object for the file.
///
/// # Returns
///
/// * `Result<PathBuf, std::io::Error>` - A result containing a `PathBuf` object for the backup file.
///     - `Ok(PathBuf)` if the backup file was created successfully.
///     - `Err(std::io::Error)` if the backup file could not be created.
///
pub fn backup_file(file_path: &Path) -> io::Result<PathBuf> {
    let backup_path = file_path.with_extension("src.html");
    fs::copy(file_path, &backup_path)?;
    Ok(backup_path)
}

/// Writes a minified HTML file.
///
/// This function takes a reference to a `Path` object for the file to write and a string containing the minified HTML,
/// and writes the minified HTML to the file.
///
/// # Arguments
///
/// * `file_path` - A reference to a `Path` object for the file to write.
/// * `minified_html` - A string containing the minified HTML.
///
/// # Returns
///
/// * `Result<(), std::io::Error>` - A result indicating success or failure.
///     - `Ok(())` if the minified HTML was written successfully.
///     - `Err(std::io::Error)` if the minified HTML could not be written.
///
pub fn write_minified_html(
    file_path: &Path,
    minified_html: &str,
) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(minified_html.as_bytes())?;
    Ok(())
}

/// Cleans up the directory at the given path.
///
/// If the directory does not exist, this function does nothing.
///
/// # Arguments
///
/// * `directories` - An array of references to `Path` objects representing the directories to be cleaned up.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - A result indicating success or failure.
///     - `Ok(())` if the directories were cleaned up successfully.
///     - `Err(Box<dyn Error>)` if an error occurred during the cleanup process.
///
pub fn cleanup_directory(
    directories: &[&Path],
) -> Result<(), Box<dyn Error>> {
    for directory in directories {
        if !directory.exists() {
            continue;
        }

        println!(
            "❯ Cleaning up `{}` directory...",
            directory.display()
        );

        fs::remove_dir_all(directory)?;

        println!("  Done.\n");
    }

    Ok(())
}

/// Creates a new directory at the given path.
///
/// If the directory already exists, this function does nothing.
///
/// # Arguments
///
/// * `directories` - An array of references to `Path` objects representing the directories to be created.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - A result indicating success or failure.
///     - `Ok(())` if the directories were created successfully.
///     - `Err(Box<dyn Error>)` if an error occurred during the creation process.
///
pub fn create_directory(
    directories: &[&Path],
) -> Result<(), Box<dyn Error>> {
    for directory in directories {
        if directory.exists() {
            continue;
        }

        println!("❯ Creating `{}` directory...", directory.display());

        fs::create_dir(directory)?;

        println!("  Done.\n");
    }

    Ok(())
}

/// Helper function to write XML element
pub fn write_element(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    name: &str,
    value: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !value.is_empty() {
        let element_start = BytesStart::new(name);
        writer.write_event(Event::Start(element_start.clone()))?;
        writer
            .write_event(Event::Text(BytesText::from_escaped(value)))?;

        let element_end = BytesEnd::new::<Cow<'static, str>>(
            std::str::from_utf8(
                element_start.name().local_name().as_ref(),
            )
            .unwrap()
            .to_string()
            .into(),
        );

        writer.write_event(Event::End(element_end))?;
    }
    Ok(())
}

/// Converts a string to title case.
pub fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(c).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Formats a header string with an ID and class attribute.
pub fn format_header_with_id_class(
    header_str: &str,
    id_regex: &Regex,
) -> String {
    let mut formatted_header_str = String::new();
    let mut in_header_tag = false;
    let mut id_attribute_added = false;
    let mut class_attribute_added = false;

    for c in header_str.chars() {
        if !in_header_tag {
            formatted_header_str.push(c);
            if c == '<' {
                in_header_tag = true;
            }
        } else {
            if !id_attribute_added && c == ' ' {
                formatted_header_str.push_str(&format!(
                    " id=\"{}\"",
                    id_regex
                        .replace_all(&header_str.to_lowercase(), "-")
                        .trim_matches('-')
                        .trim_end_matches("-h1")
                        .trim_end_matches("-h2")
                        .trim_end_matches("-h3")
                        .trim_end_matches("-h4")
                        .trim_end_matches("-h5")
                        .trim_end_matches("-h6")
                ));
                id_attribute_added = true;
            }
            if !class_attribute_added && c == '>' {
                formatted_header_str.push_str(&format!(
                    " class=\"{}\"",
                    id_regex
                        .replace_all(&header_str.to_lowercase(), "-")
                        .trim_matches('-')
                        .trim_end_matches("-h1")
                        .trim_end_matches("-h2")
                        .trim_end_matches("-h3")
                        .trim_end_matches("-h4")
                        .trim_end_matches("-h5")
                        .trim_end_matches("-h6")
                ));
                class_attribute_added = true;
            }
            formatted_header_str.push(c);
            if c == '>' {
                in_header_tag = false;
            }
        }
    }
    formatted_header_str
}

/// Extracts the front matter from the given content.
pub fn extract_front_matter(content: &str) -> &str {
    if content.starts_with("---\n") {
        if let Some(end_pos) = content.find("\n---\n") {
            &content[end_pos + 5..] // Skip the "---\n\n" that follows the front matter
        } else {
            ""
        }
    } else if content.starts_with("+++\n") {
        if let Some(end_pos) = content.find("\n+++\n") {
            &content[end_pos + 5..] // Skip the "+++\n\n" that follows the front matter
        } else {
            ""
        }
    } else if content.starts_with("{\n") {
        if let Some(end_pos) = content.find("\n}\n") {
            &content[end_pos + 2..]
        } else {
            ""
        }
    } else {
        content
    }
}

/// Creates the default Comrak options.
pub fn create_comrak_options() -> comrak::ComrakOptions {
    let mut options = comrak::ComrakOptions::default();

    // Enable non-standard Markdown features:
    options.extension.autolink = true; // Detects URLs and email addresses and makes them clickable.
    options.extension.description_lists = true; // Allows you to create description lists.
    options.extension.footnotes = true; // Allows you to create footnotes.
    options.extension.front_matter_delimiter = Some("---".to_owned()); // Ignore front-mater starting with '---'
    options.extension.header_ids = Some("".to_string()); // Adds an ID to each header.
    options.extension.strikethrough = true; // Allows you to create strikethrough text.
    options.extension.superscript = true; // Allows you to create superscript text.
    options.extension.table = true; // Allows you to create tables.
    options.extension.tagfilter = true; // Allows you to filter HTML tags.
    options.extension.tasklist = true; // Allows you to create task lists.
    options.parse.smart = true; // Enables smart punctuation.
    options.render.github_pre_lang = true; // Renders GitHub-style fenced code blocks.
    options.render.hardbreaks = false; // Renders hard line breaks as <br> tags.
    options.render.unsafe_ = true; // Allows raw HTML to be rendered.

    options
}

/// Updates the class attributes in a line of content
pub fn update_class_attributes(
    line: &str,
    class_regex: &Regex,
    img_regex: &Regex,
) -> String {
    if line.contains(".class=&quot;") {
        let captures = class_regex.captures(line).unwrap();
        let class_value = captures.get(1).unwrap().as_str();
        let updated_line = class_regex.replace(line, "");
        let updated_line_with_class = img_regex.replace(
            &updated_line,
            &format!("$1 class=\"{}\"$2", class_value),
        );
        return updated_line_with_class.into_owned();
    }
    line.to_owned()
}
