use anyhow::{Result, anyhow};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn get_exercise_name(exercise_path: &Path) -> String {
    exercise_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

pub fn get_exercise_language(exercise_path: &Path) -> Result<String> {
    // Extract the language from path (data/python/exercises/... => python)
    let parts: Vec<_> = exercise_path.components().collect();

    for (i, part) in parts.iter().enumerate() {
        if i > 0 && part.as_os_str() == "eval_code" {
            if i + 1 < parts.len() {
                let language = parts[i + 1].as_os_str().to_string_lossy().to_string();
                return Ok(language);
            }
        }
    }

    Err(anyhow!(
        "Could not determine language from path: {:?}",
        exercise_path
    ))
}

pub fn find_exercises(
    framework_path: &Path,
    languages: &[&str],
    max_per_language: Option<usize>,
) -> Result<Vec<PathBuf>> {
    let mut all_exercises = Vec::new();

    println!("Searching for exercises in languages: {:?}", languages);

    for language in languages {
        let language_dir = framework_path
            .join("eval_code")
            .join(language)
            .join("exercises")
            .join("practice");

        println!("Checking language directory: {:?}", language_dir);
        if !language_dir.exists() {
            println!("Warning: Language directory not found: {:?}", language_dir);
            continue;
        }

        let mut exercises = Vec::new();
        match fs::read_dir(&language_dir) {
            Ok(entries) => {
                for entry_result in entries {
                    match entry_result {
                        Ok(entry) => {
                            let path = entry.path();

                            if path.is_dir() {
                                // Special handling for "internal" directory
                                if *language == "internal" {
                                    // Check for repo_info.json to validate it's an internal exercise
                                    let repo_info_path = path.join(".meta").join("repo_info.json");
                                    let instructions_path =
                                        path.join(".docs").join("instructions.md");

                                    if repo_info_path.exists() && instructions_path.exists() {
                                        exercises.push(path);
                                    }
                                } else {
                                    // Map the language to the file extension - original code
                                    let language_extension = match *language {
                                        "python" => "py",
                                        "go" => "go",
                                        "rust" => "rs",
                                        "typescript" => "ts",
                                        "javascript" => "js",
                                        "ruby" => "rb",
                                        "php" => "php",
                                        "bash" => "sh",
                                        "multi" => "diff",
                                        _ => continue, // Skip unsupported languages
                                    };

                                    // Check if this is a valid exercise with instructions and example
                                    let instructions_path =
                                        path.join(".docs").join("instructions.md");
                                    let has_instructions = instructions_path.exists();
                                    let example_path = path
                                        .join(".meta")
                                        .join(format!("example.{}", language_extension));
                                    let has_example = example_path.exists();

                                    if has_instructions && has_example {
                                        exercises.push(path);
                                    }
                                }
                            }
                        }
                        Err(err) => println!("Error reading directory entry: {}", err),
                    }
                }
            }
            Err(err) => println!(
                "Error reading directory {}: {}",
                language_dir.display(),
                err
            ),
        }

        // Sort exercises by name for consistent selection
        exercises.sort_by(|a, b| {
            let a_name = a.file_name().unwrap_or_default().to_string_lossy();
            let b_name = b.file_name().unwrap_or_default().to_string_lossy();
            a_name.cmp(&b_name)
        });

        // Apply the limit if specified
        if let Some(limit) = max_per_language {
            if exercises.len() > limit {
                println!(
                    "Limiting {} exercises to {} for language {}",
                    exercises.len(),
                    limit,
                    language
                );
                exercises.truncate(limit);
            }
        }

        println!(
            "Found {} exercises for language {}: {:?}",
            exercises.len(),
            language,
            exercises
                .iter()
                .map(|p| p.file_name().unwrap_or_default().to_string_lossy())
                .collect::<Vec<_>>()
        );
        all_exercises.extend(exercises);
    }

    Ok(all_exercises)
}
