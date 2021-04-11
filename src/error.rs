use crate::term::{ERR_RED_BOLD, TERM_ERR, TERM_OUT};

use std::{fmt::Display, io, process::exit};

pub trait CliError: Display + Sized {
    fn color(self) -> Self {
        self
    }

    fn print(self) -> io::Result<()> {
        TERM_ERR.write_str(&format!("{}: ", ERR_RED_BOLD.apply_to("error").to_string()))?;
        TERM_ERR.write_line(&self.color().to_string())?;
        TERM_ERR.flush()
    }
}

pub fn finish<T>(result: Result<(), T>)
where
    T: CliError,
{
    let code = if let Some(e) = result.err() {
        e.print().unwrap();
        1
    } else {
        0
    };

    TERM_ERR.flush().unwrap();
    TERM_OUT.flush().unwrap();

    exit(code);
}
