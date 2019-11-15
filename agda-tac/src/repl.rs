use std::io::{self, Write};

use crate::editor::CliEditor;
use agda_mode::agda::ReplState;
use agda_mode::base::InteractionPoint;
use agda_mode::cmd::{Cmd, GoalInput};
use agda_mode::resp::{DisplayInfo, GoalInfo};
use rustyline::error::ReadlineError;

type Monad<T = ()> = io::Result<T>;

pub const LAMBDA_LT: &str = "\u{03bb}> ";
pub const RICH_HELP: &str =
    "\
     You're in the normal REPL, where there's \
     completion, history command, hints and (in the future) colored output.\n\
     The rich mode is not compatible with Windows PowerShell ISE and Mintty\
     (Cygwin, MinGW and (possibly, depends on your installation) git-bash).\n\
     If you're having problems with the rich mode, you may want to switch to \
     the plain mode (restart agda-tac with `--plain` flag).\
     ";
pub const PLAIN_HELP: &str = "You're in the plain REPL (with `--plain` flag).";
pub const HELP: &str = "help";

pub async fn repl(mut agda: ReplState, plain: bool) -> Monad {
    if plain {
        let stdin = io::stdin();
        loop {
            print!("> ");
            io::stdout().flush()?;
            let mut next = String::new();
            stdin.read_line(&mut next)?;
            let trim = next.trim();
            if trim == HELP {
                println!("{}", PLAIN_HELP);
            } else if line(&mut agda, trim.to_owned()).await? {
                break Ok(());
            }
        }
    } else {
        let editor = CliEditor {};
        let mut r = editor.into_editor();
        loop {
            match r.readline(LAMBDA_LT) {
                Ok(input) => {
                    let trim = input.trim();
                    r.add_history_entry(trim);
                    if trim == HELP {
                        println!("{}", RICH_HELP);
                    } else if line(&mut agda, trim.to_owned()).await? {
                        break Ok(());
                    }
                }
                Err(ReadlineError::Interrupted) => {}
                Err(ReadlineError::Eof) => {
                    println!("Interrupted by Ctrl-d");
                    break Ok(());
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break Ok(());
                }
            }
        }
    }
}

pub async fn line(agda: &mut ReplState, line: String) -> Monad<bool> {
    reload(agda).await?;
    // TODO
    Ok(false)
}

pub async fn reload(agda: &mut ReplState) -> Monad {
    match agda.next_goals().await? {
        Ok(iis) => {
            println!("Goals:");
            if iis.is_empty() {
                println!("No goals.");
            }
            list_goals(agda, &iis).await?;
        }
        Err(err_msg) => {
            eprintln!("Errors:");
            eprintln!("{}", err_msg);
        }
    };
    finish(agda).await
}

async fn finish(agda: &mut ReplState) -> Monad {
    agda.command(Cmd::Abort).await?;
    agda.shutdown().await
}

async fn list_goals(agda: &mut ReplState, iis: &[InteractionPoint]) -> Monad {
    for &ii in iis {
        agda.command(Cmd::goal_type(GoalInput::simple(ii))).await?;
        let ty = loop {
            if let DisplayInfo::GoalSpecific {
                goal_info: GoalInfo::CurrentGoal { the_type, .. },
                ..
            } = agda.next_display_info().await?
            {
                break the_type;
            }
        };
        println!("?{:?}: {}", ii, ty);
    }
    Ok(())
}
