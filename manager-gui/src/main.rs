#[macro_use]
extern crate log;

#[macro_use]
extern crate conrod_core;
extern crate conrod_glium;
#[macro_use]
extern crate conrod_winit;
extern crate find_folder;
extern crate glium;
extern crate image;

use conrod_core::{Colorable, Labelable, Positionable, Scalar, Sizeable, Ui};
use glium::Surface;
use layout::WidgetId;
use overwatch::{
    overwatch_3v3::{
        Match,
        Roster,
        CompBuilder,
        Player
    },
    BattleTag,
    Hero,
    HeroPool
};
use std::{
    collections::BTreeMap,
    io::Write,
    mem,
    path
};

mod app;
mod image_util;
mod layout;
mod state;
mod support;
mod window_mgmt;


fn main() {
    color_backtrace::install();
    setup_logger(2);
    let (mut events_loop, mut app) = window_mgmt::init_window();
    window_mgmt::main_window_loop(events_loop, app);
}

fn setup_logger(level: u64) {
    let mut builder = pretty_env_logger::formatted_timed_builder();

    let noisy_modules = &[
        "hyper",
        "mio",
        "tokio_core",
        "tokio_reactor",
        "tokio_threadpool",
        "fuse::request",
        "rusoto_core",
        "want",
    ];

    let log_level = match level {
        //0 => log::Level::Error,
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    if level > 1 && level < 4 {
        for module in noisy_modules {
            builder.filter_module(module, log::LevelFilter::Info);
        }
    }

    builder.filter_level(log_level);
    builder.default_format_timestamp(true);
    //builder.format(|buf, record| writeln!(buf, "{}", record.args()));
    builder.init();
}
