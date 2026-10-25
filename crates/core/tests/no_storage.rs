//! Changed default (D17 → IG-6): the brief allows a save in the browser. This
//! reads every file under web/ (vendor/ aside) and fails if any file other than
//! `web/save.js` names a browser storage API, so storage has one door the way
//! the core does. The core writes and reads the save; `api.rs` tests that an
//! unreadable save starts a new game and is kept.

use std::fs;
use std::path::{Path, PathBuf};

/// The one page file allowed to name storage.
const STORAGE_DOOR: &str = "web/save.js";

const STORAGE_NAMES: &[&str] = &[
    "localStorage",
    "sessionStorage",
    "indexedDB",
    "IDBFactory",
    "document.cookie",
    "caches",
    "CacheStorage",
    "serviceWorker",
    "navigator.storage",
    "openDatabase",
];

fn storage_names_in(text: &str) -> Vec<&'static str> {
    STORAGE_NAMES
        .iter()
        .copied()
        .filter(|name| text.contains(name))
        .collect()
}

fn files_under(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(read) = fs::read_dir(&d) else {
            continue;
        };
        for entry in read {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                if path.file_name().map(|n| n != "vendor").unwrap_or(true) {
                    stack.push(path);
                }
            } else {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

#[test]
fn only_save_js_names_browser_storage() {
    let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../web");
    let mut found = Vec::new();
    for path in files_under(&web) {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        if path.ends_with(STORAGE_DOOR) {
            continue;
        }
        for name in storage_names_in(&text) {
            found.push(format!("{}: {name}", path.display()));
        }
    }
    assert!(
        found.is_empty(),
        "browser storage outside web/save.js (IG-6):\n{}",
        found.join("\n")
    );
}

#[test]
fn the_scanner_sees_storage() {
    assert_eq!(
        storage_names_in("window.localStorage.setItem('n', name)"),
        vec!["localStorage"]
    );
    assert_eq!(storage_names_in("indexedDB.open('x')"), vec!["indexedDB"]);
    assert!(storage_names_in("el(\"note\").value").is_empty());
}
