//! Every action, enemy, item and skill the core names has a picture in
//! web/icons.svg (T18). Before this test the regions' actions had none,
//! because the icon script read only the Vale's rows and the browser check
//! sees only what a new game shows.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use tool_core::content as c;

fn symbols() -> BTreeSet<String> {
    let svg = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../web/icons.svg")).unwrap();
    svg.split("<symbol id=\"")
        .skip(1)
        .filter_map(|s| s.split('"').next())
        .map(String::from)
        .collect()
}

#[test]
fn every_thing_the_core_names_has_an_icon() {
    let have = symbols();
    let mut missing: Vec<&str> = Vec::new();
    let names = c::ACTIONS
        .iter()
        .map(|a| a.id)
        .chain(c::ENEMIES.iter().map(|e| e.id))
        .chain(c::VALUES.iter().map(|(i, _)| *i))
        .chain(c::SKILLS.iter().map(|s| s.id))
        .chain(c::CROPS.iter().map(|cr| cr.id));
    for n in names {
        if !have.contains(n) {
            missing.push(n);
        }
    }
    missing.sort();
    missing.dedup();
    assert!(
        missing.is_empty(),
        "{} with no icon: {:?}",
        missing.len(),
        missing
    );
}
