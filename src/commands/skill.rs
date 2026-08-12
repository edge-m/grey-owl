use std::fs;
use std::path::PathBuf;

const SKILL_NAME: &str = "growl";
const SKILL_CONTENT: &str = include_str!("../../skills/growl/SKILL.md");

pub fn run(args: &[String]) -> Result<u8, String> {
    if args.len() != 1 || args[0] == "--help" || args[0] == "-h" {
        return Err("usage: growl skill <output-directory>".to_string());
    }

    let output_directory = PathBuf::from(&args[0]);
    let skill_directory = output_directory.join(SKILL_NAME);
    fs::create_dir_all(&skill_directory)
        .map_err(|error| format!("cannot create skill directory {}: {error}", skill_directory.display()))?;

    let skill_path = skill_directory.join("SKILL.md");
    fs::write(&skill_path, SKILL_CONTENT)
        .map_err(|error| format!("cannot write skill {}: {error}", skill_path.display()))?;
    println!("wrote {}", skill_path.display());

    Ok(0)
}
