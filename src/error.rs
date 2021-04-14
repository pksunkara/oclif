use crate::term::{ERR_RED_BOLD, OUT_RED_BOLD, TERM_ERR, TERM_OUT};

use std::{fmt::Display, io, process::exit};

pub trait CliError: Display + Sized {
    fn color_err(self) -> Self {
        self
    }

    fn color_out(self) -> Self {
        self
    }

    fn print_out(self) -> io::Result<()> {
        TERM_OUT.write_line(&self.color_out().to_string())?;
        TERM_OUT.flush()
    }

    fn print_err(self) -> io::Result<()> {
        TERM_ERR.write_str(&format!("{}: ", ERR_RED_BOLD.apply_to("error").to_string()))?;
        TERM_ERR.write_line(&self.color_err().to_string())?;
        TERM_ERR.flush()
    }
}

pub fn finish<T>(result: Result<(), T>)
where
    T: CliError,
{
    let code = if let Some(e) = result.err() {
        e.print_err().unwrap();
        1
    } else {
        0
    };

    TERM_ERR.flush().unwrap();
    TERM_OUT.flush().unwrap();

    exit(code);
}
