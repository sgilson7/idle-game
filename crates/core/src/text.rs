//! Reader-visible words. They live in `data/strings.json` (D5); the core reads
//! them once, fills in the numbers a rule decides, and formats every number
//! the player sees (IG-9), because the page may not round or format.

use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SkillWords {
    pub name: String,
    pub short: String,
    pub verb: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Text {
    pub title: String,
    pub skills: BTreeMap<String, SkillWords>,
    pub actions: BTreeMap<String, String>,
    pub items: BTreeMap<String, String>,
    pub enemies: BTreeMap<String, String>,
    pub tools: BTreeMap<String, Vec<String>>,
    pub ui: BTreeMap<String, String>,
}

impl Text {
    pub fn parse(json: &str) -> Result<Text, String> {
        serde_json::from_str(json).map_err(|e| format!("The strings file could not be read: {e}"))
    }

    /// A template from `ui`, with each `{name}` filled. A missing key shows
    /// itself in brackets, and a test makes sure none is missing.
    pub fn t(&self, key: &str, fill: &[(&str, String)]) -> String {
        let mut s = self.ui.get(key).cloned().unwrap_or_else(|| format!("[{key}]"));
        for (name, value) in fill {
            s = s.replace(&format!("{{{name}}}"), value);
        }
        s
    }

    pub fn s(&self, key: &str) -> String {
        self.t(key, &[])
    }

    pub fn skill(&self, id: &str) -> &SkillWords {
        self.skills.get(id).unwrap_or_else(|| &self.skills["woodcutting"])
    }

    pub fn item(&self, id: &str) -> String {
        self.items.get(id).cloned().unwrap_or_else(|| id.to_string())
    }

    pub fn action(&self, id: &str) -> String {
        self.actions.get(id).cloned().unwrap_or_else(|| id.to_string())
    }

    pub fn enemy(&self, id: &str) -> String {
        self.enemies.get(id).cloned().unwrap_or_else(|| id.to_string())
    }
}

/// 9,730 · 12.3K · 4.38M · 1.20B
pub fn num(n: u64) -> String {
    if n >= 1_000_000_000 {
        format!("{}.{:02}B", n / 1_000_000_000, n % 1_000_000_000 / 10_000_000)
    } else if n >= 1_000_000 {
        format!("{}.{:02}M", n / 1_000_000, n % 1_000_000 / 10_000)
    } else if n >= 10_000 {
        format!("{}.{}K", n / 1_000, n % 1_000 / 100)
    } else if n >= 1_000 {
        format!("{},{:03}", n / 1_000, n % 1_000)
    } else {
        n.to_string()
    }
}

/// 45s · 6m 16s · 2h 5m
pub fn dur(ms: u64) -> String {
    let s = ms.div_ceil(1_000);
    let (h, m, r) = (s / 3_600, s % 3_600 / 60, s % 60);
    if h > 0 {
        format!("{h}h {m}m")
    } else if m > 0 && r > 0 {
        format!("{m}m {r}s")
    } else if m > 0 {
        format!("{m}m")
    } else {
        format!("{r}s")
    }
}

/// An action's length in seconds with two decimals: 3.40s.
pub fn secs(ms: u64) -> String {
    format!("{}.{:02}s", ms / 1_000, ms % 1_000 / 10)
}

/// A bar's width for CSS, from hundredths of a percent: 37.50%.
pub fn width(bp: u64) -> String {
    format!("{}.{:02}%", bp / 100, bp % 100)
}

/// A multiplier from basis points: ×1.25.
pub fn times(bp: u64) -> String {
    format!("×{}.{:02}", bp / 10_000, bp % 10_000 / 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_are_formatted_by_size() {
        assert_eq!(num(0), "0");
        assert_eq!(num(999), "999");
        assert_eq!(num(9_730), "9,730");
        assert_eq!(num(1_005), "1,005");
        assert_eq!(num(12_345), "12.3K");
        assert_eq!(num(4_385_776), "4.38M");
        assert_eq!(num(1_200_000_000), "1.20B");
    }

    #[test]
    fn durations_round_up_to_the_second() {
        assert_eq!(dur(0), "0s");
        assert_eq!(dur(1), "1s");
        assert_eq!(dur(45_000), "45s");
        assert_eq!(dur(376_000), "6m 16s");
        assert_eq!(dur(1_200_000), "20m");
        assert_eq!(dur(7_500_000), "2h 5m");
    }

    #[test]
    fn widths_seconds_and_multipliers() {
        assert_eq!(width(3_750), "37.50%");
        assert_eq!(width(10_000), "100.00%");
        assert_eq!(secs(3_400), "3.40s");
        assert_eq!(times(12_500), "×1.25");
        assert_eq!(times(10_000), "×1.00");
    }

    #[test]
    fn templates_fill_and_report_missing_keys() {
        let t = Text {
            title: "T".into(),
            skills: BTreeMap::new(),
            actions: BTreeMap::new(),
            items: BTreeMap::new(),
            enemies: BTreeMap::new(),
            tools: BTreeMap::new(),
            ui: [("lvl".to_string(), "Level {n}".to_string())]
                .into_iter()
                .collect(),
        };
        assert_eq!(t.t("lvl", &[("n", "4".into())]), "Level 4");
        assert_eq!(t.s("nope"), "[nope]");
    }
}
