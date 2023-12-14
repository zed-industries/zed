use std::fs::File;
use std::io::Write;

use strum::IntoEnumIterator;

use storybook2::story_selector::ComponentStory;

// TOOD: Ideally we actually create a more full featured CLI,
// but for the moment I just wanted a easier way to run the stories

fn main() -> std::io::Result<()> {
    let path = std::env::current_dir()?;
    let out_file = path.join("script").join("storybook");

    // the script output file
    let mut file = File::create(out_file)?;

    // generate the list of components, in `snake_case`
    let components = ComponentStory::iter()
        .map(|c| c.to_string()) // Converts enum to string in `snake_case`
        .collect::<Vec<_>>();

    // write the bash script
    writeln!(file, "#!/bin/bash")?;
    writeln!(file, "")?;
    writeln!(file, "options=(")?;
    for component in &components {
        writeln!(file, "  \"{}\"", component)?;
    }
    writeln!(file, ")")?;
    writeln!(file, "")?;

    // Check if an argument is provided and if it matches a valid option
    writeln!(file, "run_story() {{")?;
    writeln!(file, "  echo \"Running story: $1\"")?;
    writeln!(file, "  cargo run -p storybook2 -- \"$1\"")?;
    writeln!(file, "}}")?;
    writeln!(file, "")?;

    writeln!(file, "if [ \"$#\" -gt 0 ]; then")?;
    writeln!(file, "  story_arg=\"$1\"")?;
    writeln!(file, "  # Add prefix 'components/' if not present")?;
    writeln!(file, "  if [[ $story_arg != components/* ]]; then")?;
    writeln!(file, "    story_arg=\"components/$story_arg\"")?;
    writeln!(file, "  fi")?;
    writeln!(file, "  # Check if the provided story is a valid option")?;
    writeln!(file, "  for opt in \"${{options[@]}}\"; do")?;
    writeln!(
        file,
        "    if [[ \"components/$opt\" == \"$story_arg\" ]]; then"
    )?;
    writeln!(file, "      run_story \"$story_arg\"")?;
    writeln!(file, "      exit")?;
    writeln!(file, "    fi")?;
    writeln!(file, "  done")?;
    writeln!(file, "  echo \"Invalid story name: $1\"")?;
    writeln!(file, "  exit 1")?;
    writeln!(file, "fi")?;
    writeln!(file, "")?;

    // Existing selection prompt
    writeln!(file, "prompt=\"Please select a story:\"")?;
    writeln!(file, "PS3=\"$prompt \"")?;
    writeln!(file, "select story in \"${{options[@]}}\"; do")?;
    writeln!(file, "  if [[ -n $story ]]; then")?;
    writeln!(file, "    run_story \"components/$story\"")?;
    writeln!(file, "    break")?;
    writeln!(file, "  else")?;
    writeln!(file, "    echo \"Invalid option\"")?;
    writeln!(file, "  fi")?;
    writeln!(file, "done")?;

    Ok(())
}
