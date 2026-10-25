//! RuneScape's experience table, levels 1 to 99, as whole numbers (IG-2).
//!
//! The published formula sums ⌊l + 300·2^(l/7)⌋ and divides by four; it needs
//! a power of two with a fractional exponent, which the core cannot compute
//! without floats (D2). The table is the formula's output, written out, and
//! the tests pin it to the values RuneScape publishes.

pub const MAX_LEVEL: u32 = 99;

/// Experience needed to reach each level; `TABLE[0]` is level 1.
#[rustfmt::skip]
pub const TABLE: [u64; 99] = [
    0, 83, 174, 276, 388, 512, 650, 801, 969,
    1_154, 1_358, 1_584, 1_833, 2_107, 2_411, 2_746, 3_115, 3_523,
    3_973, 4_470, 5_018, 5_624, 6_291, 7_028, 7_842, 8_740, 9_730,
    10_824, 12_031, 13_363, 14_833, 16_456, 18_247, 20_224, 22_406, 24_815,
    27_473, 30_408, 33_648, 37_224, 41_171, 45_529, 50_339, 55_649, 61_512,
    67_983, 75_127, 83_014, 91_721, 101_333, 111_945, 123_660, 136_594, 150_872,
    166_636, 184_040, 203_254, 224_466, 247_886, 273_742, 302_288, 333_804, 368_599,
    407_015, 449_428, 496_254, 547_953, 605_032, 668_051, 737_627, 814_445, 899_257,
    992_895, 1_096_278, 1_210_421, 1_336_443, 1_475_581, 1_629_200, 1_798_808, 1_986_068, 2_192_818,
    2_421_087, 2_673_114, 2_951_373, 3_258_594, 3_597_792, 3_972_294, 4_385_776, 4_842_295, 5_346_332,
    5_902_831, 6_517_253, 7_195_629, 7_944_614, 8_771_558, 9_684_577, 10_692_629, 11_805_606, 13_034_431,
];

/// Experience at the start of `level` (1..=99).
pub fn at(level: u32) -> u64 {
    TABLE[(level.clamp(1, MAX_LEVEL) - 1) as usize]
}

/// The level that `xp` experience gives.
pub fn level(xp: u64) -> u32 {
    // the table rises, so a binary search finds the same count as a scan
    TABLE.partition_point(|&need| need <= xp) as u32
}

/// Mastery runs on a table this many times the skill table (P6): a mastery 99
/// in one resource is a long goal of its own.
pub const MASTERY_SCALE: u64 = 2;

pub fn mastery_level(mxp: u64) -> u32 {
    TABLE.partition_point(|&need| need * MASTERY_SCALE <= mxp) as u32
}

pub fn mastery_at(level: u32) -> u64 {
    at(level) * MASTERY_SCALE
}

/// Progress through the current level in hundredths of a percent, for bars.
pub fn progress_bp(xp: u64, start: u64, next: u64) -> u64 {
    match next.checked_sub(start) {
        Some(span) if span > 0 => (xp.saturating_sub(start) * 10_000 / span).min(10_000),
        _ => 10_000,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table_matches_runescapes_published_values() {
        // From the OSRS Wiki's Experience table.
        assert_eq!(at(1), 0);
        assert_eq!(at(2), 83);
        assert_eq!(at(10), 1_154);
        assert_eq!(at(50), 101_333);
        assert_eq!(at(92), 6_517_253);
        assert_eq!(at(99), 13_034_431);
    }

    #[test]
    fn level_92_is_half_of_99() {
        assert_eq!(at(99) / 2, 6_517_215);
        assert!(at(92).abs_diff(at(99) / 2) < 100);
    }

    #[test]
    fn level_reads_the_table_at_its_edges() {
        assert_eq!(level(0), 1);
        assert_eq!(level(82), 1);
        assert_eq!(level(83), 2);
        assert_eq!(level(13_034_430), 98);
        assert_eq!(level(13_034_431), 99);
        assert_eq!(level(200_000_000), 99);
    }

    #[test]
    fn mastery_uses_twice_the_table() {
        assert_eq!(mastery_level(165), 1);
        assert_eq!(mastery_level(166), 2);
        assert_eq!(mastery_at(99), 26_068_862);
    }

    #[test]
    fn progress_is_in_hundredths_of_a_percent() {
        assert_eq!(progress_bp(0, 0, 83), 0);
        assert_eq!(progress_bp(166, 83, 249), 5_000);
        assert_eq!(progress_bp(500, 83, 174), 10_000);
        assert_eq!(progress_bp(5, 5, 5), 10_000);
    }
}
