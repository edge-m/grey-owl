use std::fs;
use std::path::PathBuf;

use clap::Args as ClapArgs;

const SKILL_NAME: &str = "growl";
const SKILL_CONTENT: &str = include_str!("../../skills/growl/SKILL.md");

#[derive(Debug, ClapArgs)]
pub struct Args {
    #[arg(help = "Directory where the Skill should be written")]
    pub output_directory: PathBuf,
}

pub fn run(args: &Args) -> Result<u8, String> {
    let skill_directory = args.output_directory.join(SKILL_NAME);
    fs::create_dir_all(&skill_directory)
        .map_err(|error| format!("cannot create skill directory {}: {error}", skill_directory.display()))?;

    let skill_path = skill_directory.join("SKILL.md");
    fs::write(&skill_path, SKILL_CONTENT)
        .map_err(|error| format!("cannot write skill {}: {error}", skill_path.display()))?;
    println!("wrote {}", skill_path.display());

    Ok(0)
}
