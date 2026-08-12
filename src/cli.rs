use crate::commands::{check, init, skill};

pub fn run(args: Vec<String>) -> Result<u8, String> {
    if args.is_empty() || args[0] == "--help" || args[0] == "-h" {
        print_help();
        return Ok(0);
    }

    match args[0].as_str() {
        "check" => check::run(&args[1..]),
        "init" => init::run(&args[1..]),
        "skill" => skill::run(&args[1..]),
        command => Err(format!("unknown command '{command}'; try 'growl --help'")),
    }
}

fn print_help() {
    println!(
        "growl — Grey Owl wiki validator\n\n\
Usage:\n  growl init\n  growl check [<wiki-path>] [--config <file>] [--format human|json]\n  growl skill <output-directory>\n\n\
Options:\n  --config <file>       YAML configuration file\n  --format <format>     human (default) or json\n  -h, --help            Show this help"
    );
}
