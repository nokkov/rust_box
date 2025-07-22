use clap::{arg, Command};

fn cli() -> Command {
    Command::new("box")
        .author("Arkady Chesnokov, arkady-chesnokof@yandex.ru")
        .version("0.0.1")
        .about("A fictional docker/youki clone")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .override_usage("rust_box <SUBCOMMAND>")
        .subcommand(
            Command::new("build")
                .about("Build image from boxfile")
                .arg(
                    arg!(-f --file <BOXFILE_PATH> "Path to boxfile")
                )
        )    
}

fn main() {
    let matches = cli().get_matches();
}
