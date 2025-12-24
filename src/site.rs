use std::fs;
use std::io;
use std::path::Path;

use crate::markdown;
use crate::template;

pub fn build(content_dir: &Path, dist_dir: &Path) -> io::Result<()> {
    if !content_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Input directory '{}' not found", content_dir.display()),
        ));
    }

    if !dist_dir.exists() {
        fs::create_dir_all(dist_dir)?;
    }

    let mut md_files: Vec<_> = fs::read_dir(content_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map(|ext| ext == "md")
                .unwrap_or(false)
        })
        .collect();

    md_files.sort_by_key(|entry| entry.file_name());

    let mut count = 0;
    for entry in md_files {
        let src_path = entry.path();
        let file_stem = src_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("page");

        let md_content = fs::read_to_string(&src_path)?;
        let html_content = markdown::parse(&md_content);
        let title = extract_title(&md_content).unwrap_or_else(|| file_stem.to_string());
        let full_html = template::render(&title, &html_content);

        let dest_path = dist_dir.join(format!("{}.html", file_stem));
        fs::write(&dest_path, full_html)?;

        println!("  {} -> {}", src_path.display(), dest_path.display());
        count += 1;
    }

    println!("Generated {} files.", count);
    Ok(())
}

fn extract_title(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") && !trimmed.starts_with("## ") {
            return Some(trimmed[2..].trim().to_string());
        }
    }
    None
}
