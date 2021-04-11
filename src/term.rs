use console::{Style, Term};
use once_cell::sync::Lazy;
use paste::paste;

pub static TERM_ERR: Lazy<Term> = Lazy::new(|| Term::stderr());
pub static TERM_OUT: Lazy<Term> = Lazy::new(|| Term::stdout());

macro_rules! color {
    ($color:ident) => {
        paste! {
            pub static [<ERR_ $color:upper>]: Lazy<Style> = Lazy::new(|| Style::new().for_stderr().$color());
            pub static [<ERR_ $color:upper _BOLD>]: Lazy<Style> = Lazy::new(|| Style::new().for_stderr().$color().bold());
            pub static [<OUT_ $color:upper>]: Lazy<Style> = Lazy::new(|| Style::new().$color());
            pub static [<OUT_ $color:upper _BOLD>]: Lazy<Style> = Lazy::new(|| Style::new().$color().bold());
        }
    };
}

color!(black);
color!(red);
color!(green);
color!(yellow);
color!(blue);
color!(magenta);
color!(cyan);
color!(white);
