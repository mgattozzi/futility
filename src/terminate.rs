//! Types and functions associated with exiting a program

use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    panic::{self, PanicInfo},
};

/// The `Terminate` type is used to setup the execution of program from start to
/// finish and what to do when the program errors, what panic hooks to use, what
/// to install at the beginning, and any clean up that needs to occur when
/// exiting the program.
pub struct Terminate<E>
where
    E: Display + Debug,
{
    at_exit: Option<fn()>,
    on_error: Option<fn(E) -> E>,
    install: Option<fn() -> Result<(), E>>,
    error: PhantomData<E>,
}

impl<E> Terminate<E>
where
    E: Display + Debug,
{
    /// Create a new Terminate
    pub fn new() -> Self {
        Self {
            on_error: None,
            at_exit: None,
            install: None,
            error: PhantomData,
        }
    }

    /// Install anything that needs to be installed before program execution
    /// like `tracing`
    pub fn install(mut self, install: fn() -> Result<(), E>) -> Self {
        self.install = Some(install);
        self
    }

    /// Set a panic for the program that replaces the original panic hook
    pub fn replace_panic(self, panic: impl Fn(&PanicInfo<'_>) + Send + Sync + 'static) -> Self {
        panic::set_hook(Box::new(panic));
        self
    }

    /// Set a panic for the program that is invoked first followed by the
    /// original panic hook
    pub fn panic_with(self, panic: fn(&PanicInfo<'_>)) -> Self {
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            panic(&panic_info);
            original_hook(&panic_info);
        }));
        self
    }

    /// When there is an error in the main program set what should happen
    pub fn on_error(mut self, on_error: fn(E) -> E) -> Self {
        self.on_error = Some(on_error);
        self
    }

    /// When the program is going to exit, regardless of if there is an error or
    /// not, set what should be done
    pub fn at_exit(mut self, at_exit: fn()) -> Self {
        self.at_exit = Some(at_exit);
        self
    }

    /// Execute your program with the given function. This will:
    ///
    /// 1. Call the provided `install` function.
    /// 2. If there was an error it will call the `on_error` function if it exists
    /// 3. If there was an error then the `at_exit` function is called if it
    ///    exists
    /// 4. Call the provided the function to `execute`
    /// 5. If there was an error it will call the `on_error` function if it exists
    /// 6. If there was an error then the `at_exit` function is called if it
    ///    exists
    pub fn execute(self, main: fn() -> Result<(), E>) -> Result<(), E> {
        if let Some(install) = self.install {
            let mut res = install();
            res = match (self.on_error, res) {
                (Some(on_error), Err(err)) => Err(on_error(err)),
                (_, res) => res,
            };
            if let Some(at_exit) = self.at_exit {
                at_exit()
            }
            res?;
        }

        let mut res = main();
        res = match (self.on_error, res) {
            (Some(on_error), Err(err)) => Err(on_error(err)),
            (_, res) => res,
        };
        if let Some(at_exit) = self.at_exit {
            at_exit()
        }

        res
    }
}
