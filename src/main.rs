use std::path::PathBuf;

use common::{list_skeleton_vec, sk_cfg_dir};

use crate::app::App;
#[allow(dead_code)]
use crate::common::check_cfg_dir;

mod app;
mod common;
mod skeleton;

fn startup(app: App) {
    check_cfg_dir();
    app.get_items_from_dir(sk_cfg_dir()).unwrap_or_else(|err| {
        eprintln!(
            "ERROR: Error fetching items from directory (App::get_items_from_dir() {})",
            err
        )
    });
    list_skeleton_vec(&app.items);
}

fn main() {
    let app: App = App::new();
    startup(app);
    let mut path1 = PathBuf::new();
    path1.push(sk_cfg_dir());
    path1.push("test_dir");
    app0.items[0].copy_to_dir(path1).unwrap();
}
