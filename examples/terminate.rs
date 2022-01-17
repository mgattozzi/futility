use color_eyre::eyre::{Report, Result};
use futility::terminate::Terminate;
use std::fs;

fn main() -> Result<()> {
    Terminate::new()
        .install(|| {
            color_eyre::install()?;
            Ok(())
        })
        .panic_with(|_| {
            println!("Oh no a panic!");
        })
        .at_exit(|| {
            println!("The program is in the process exiting.");
        })
        .on_error(|err: Report| {
            eprintln!("Oh no we had an error");
            err.wrap_err("We're at the top of main")
        })
        .execute(run)
}

fn run() -> Result<()> {
    fs::read("non/existant/file")?;
    Ok(())
}
