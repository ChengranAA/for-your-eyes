use lopdf::{
    content::{Content, Operation},
    Document,
};
use std::env;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read CLI argument
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: pdf_eye <file.pdf>");
        std::process::exit(1);
    }

    let input_path = &args[1];
    let input = Path::new(input_path);

    // Build output name: file_eye.pdf
    let stem = input.file_stem().unwrap().to_string_lossy();
    let output = format!("{}_eye.pdf", stem);

    let mut doc = Document::load(input_path)?;
    let pages = doc.get_pages();

    for (_, page_id) in pages {
        let content_data = doc.get_page_content(page_id)?;
        let mut content = Content::decode(&content_data)?;

        // Soft beige reading background
        let bg_color = vec![0.96.into(), 0.94.into(), 0.88.into()];

        let mut background = vec![
            Operation::new("q", vec![]),
            Operation::new("rg", bg_color),
            Operation::new("re", vec![0.into(), 0.into(), 2000.into(), 2000.into()]),
            Operation::new("f", vec![]),
            Operation::new("Q", vec![]),
        ];

        background.extend(content.operations);
        content.operations = background;

        let encoded = content.encode()?;
        doc.change_page_content(page_id, encoded)?;
    }

    doc.save(&output)?;

    println!("Created {}", output);

    Ok(())
}
