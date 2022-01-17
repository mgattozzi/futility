use color_eyre::eyre::Report;
use futility::terminate::Terminate;
use std::error::Error;

#[test]
pub fn terminate_eyre() -> Result<(), Report> {
    Terminate::new()
        .at_exit(|| {
            println!("The program is in the process exiting.");
        })
        .on_error(|err: Report| err.wrap_err("We're at the top of main"))
        .execute(|| {
            println!("I'm the actual program and can be both a closure or just a function");
            Ok(())
        })
}

#[test]
pub fn terminate_box_err() -> Result<(), Box<dyn Error>> {
    Terminate::new()
        .at_exit(|| {
            println!("The program is in the process exiting.");
        })
        .on_error(|err: Box<dyn Error>| {
            if let Some(source) = err.source() {
                eprintln!("Lower Level Source Error: {source:?}");
            }
            err
        })
        .execute(|| {
            println!("I'm the actual program and can be both a closure or just a function");
            Ok(())
        })
}

#[test]
pub fn terminate_eyre_named_fn() -> Result<(), Report> {
    Terminate::new()
        .install(install)
        .at_exit(at_exit)
        .on_error(eyre_on_error)
        .execute(execute)
}
#[test]
pub fn terminate_box_err_named_fn() -> Result<(), Box<dyn Error>> {
    Terminate::new()
        .at_exit(at_exit)
        .on_error(box_on_error)
        .execute(execute)
}

fn install() -> Result<(), Report> {
    color_eyre::install()?;
    Ok(())
}

fn eyre_on_error(err: Report) -> Report {
    err.wrap_err("We're at the top of main")
}
fn box_on_error(err: Box<dyn Error>) -> Box<dyn Error> {
    if let Some(source) = err.source() {
        eprintln!("Lower Level Source Error: {source:?}");
    }
    err
}

fn at_exit() {
    println!("The program is in the process exiting.");
}

fn execute<E>() -> Result<(), E> {
    println!("I'm the actual program and can be both a closure or just a function");
    Ok(())
}
