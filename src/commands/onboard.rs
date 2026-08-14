use clap::Args as ClapArgs;

#[derive(Debug, ClapArgs)]
pub struct Args {}

pub fn run(_args: &Args) -> Result<u8, String> {
    println!(
        "Grey Owl Agent onboarding\n\n\
1. Install the Agent Skill:\n\
   growl skill <agent-skills-directory>\n\n\
2. Make the generated growl/ directory available to your Agent, together\n\
   with the Wiki directory.\n\n\
3. Ask your Agent:\n\
   \"Set up this Wiki with Grey Owl.\"\n\n\
The Agent will use the wiki-config Skill to ask about your Wiki, design\n\
growl.yml interactively, confirm the design, and validate the result.\n\n\
After setup, ask the Agent to:\n\
  - explain the current Wiki structure\n\
  - add an article from a source\n\
  - search the Wiki\n\
  - find maintenance candidates\n\
  - answer questions using the Wiki\n\n\
Useful commands:\n\
  growl init       Create a starter configuration\n\
  growl skill ...  Write the Agent Skills\n\
  growl check      Validate the Wiki"
    );

    Ok(0)
}
