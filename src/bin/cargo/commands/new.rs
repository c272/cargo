use crate::command_prelude::*;

use cargo::ops;

pub fn cli() -> Command {
    subcommand("new")
        .about("Create a new cargo package at <path>")
        .arg(
            Arg::new("path")
                .value_name("PATH")
                .action(ArgAction::Set)
                .required(true),
        )
        .arg_new_opts()
        .arg_registry("Registry to use")
        .arg_silent_suggestion()
        .after_help(color_print::cstr!(
            "Run `<cyan,bold>cargo help new</>` for more detailed information.\n"
        ))
}

pub fn exec(config: &mut Config, args: &ArgMatches) -> CliResult {
    let opts = args.new_options(config)?;

    ops::new(&opts, config)?;
    Ok(())
}
