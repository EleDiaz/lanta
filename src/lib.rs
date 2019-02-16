#![allow(unknown_lints)]

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

use error_chain::ChainedError;
use log::{debug, error, info, log};

mod bsplayout;
pub mod cmd;
pub mod config;
pub mod core;
pub mod errors;
mod keys;
mod utils;
mod window_set;
mod x;

use errors::*;
use keys::{KeyCombo, KeyHandlers};
use window_set::WindowSet;
use x::{Connection, Event, StrutPartial, WindowId, WindowType};

pub use keys::ModKey;

pub mod keysym {
    pub use x11::keysym::*;
}

/// Initializes a logger using the default configuration.
///
/// Outputs to stdout and `$XDG_DATA/lanta/lanta.log` by default.
/// You should feel free to initialize your own logger, instead of using this.
pub fn intiailize_logger() -> Result<()> {
    log_panics::init();

    let xdg_dirs = xdg::BaseDirectories::with_prefix("lanta")?;
    let log_path = xdg_dirs
        .place_data_file("lanta.log")
        .chain_err(|| "Could not create log file")?;

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] {}",
                time::now().rfc3339(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .chain(fern::log_file(&log_path)?)
        .apply()?;

    Ok(())
}

#[macro_export]
macro_rules! keys {
    [ $( ([$( $mod:ident ),*], $key:ident, $cmd:expr) ),+ $(,)*] => (
        vec![
            $( (vec![$( $mod ),*],  $crate::keysym::$key, $cmd) ),+
        ]
    )
}

#[macro_export]
macro_rules! groups {
    {
        $keys:ident,
        $movemodkey:ident,
        [
            $(( [$( $modkey:ident ),+], $key:ident, $name:expr, $layout:expr )),+
            $(,)*
        ]
    }  => {{
        $keys.extend(keys![
            // Switch to group:
            $(
                ([$($modkey),+], $key, $crate::cmd::lazy::switch_group($name))
            ),+,
            // Move window to group:
            $(
                ([$($modkey),+, $movemodkey], $key,  $crate::cmd::lazy::move_window_to_group($name))
            ),+
        ]);
        vec![
            $(
                 $crate::GroupBuilder::new($name, $layout)
            ),+
        ]
    }}
}

#[macro_export]
macro_rules! layouts {
    [$( $layout:expr ),+ $(,)*] => (
        vec![
            $(
                Box::new($layout) as Box<$crate::layout::Layout>
            ),+
        ]
    )
}
