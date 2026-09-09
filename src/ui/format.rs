//! Formatting + small UI helpers shared across tabs.

use ratatui::style::Color;

use crate::ui::palette as p;

/// Decimal-byte formatter (TB / GB / MB / KB / B).
/// Matches `fmtBytes` in `grid.jsx` — drives use base-10, not base-2.
pub fn fmt_size(b: u64) -> String {
    const TB: u64 = 1_000_000_000_000;
    const GB: u64 = 1_000_000_000;
    const MB: u64 = 1_000_000;
    const KB: u64 = 1_000;
    if b >= TB {
        format!("{:.1} TB", b as f64 / TB as f64)
    } else if b >= GB {
        format!("{:.0} GB", b as f64 / GB as f64)
    } else if b >= MB {
        format!("{:.0} MB", b as f64 / MB as f64)
    } else if b >= KB {
        format!("{:.0} KB", b as f64 / KB as f64)
    } else {
        format!("{} B", b)
    }
}

/// Bytes-per-second formatter for IO rates.
pub fn fmt_rate(bps: f64) -> String {
    if bps < 1.0 {
        return "   -- ".to_string();
    }
    if bps >= 1_000_000.0 {
        format!("{:>5.1} MB/s", bps / 1_000_000.0)
    } else if bps >= 1_000.0 {
        format!("{:>5.1} KB/s", bps / 1_000.0)
    } else {
        format!("{:>5.0}  B/s", bps)
    }
}

/// Bytes-per-second in prose, for running text rather than a column:
/// no padding, and "idle" instead of a dash. [`fmt_rate`] is the
/// fixed-width form the tables use.
pub fn fmt_rate_compact(bps: f64) -> String {
    if bps >= 1_000_000_000.0 {
        format!("{:.1} GB/s", bps / 1_000_000_000.0)
    } else if bps >= 1_000_000.0 {
        format!("{:.1} MB/s", bps / 1_000_000.0)
    } else if bps >= 1000.0 {
        format!("{:.0} kB/s", bps / 1000.0)
    } else if bps > 0.0 {
        format!("{:.0} B/s", bps)
    } else {
        "idle".to_string()
    }
}

pub fn pad_right(s: &str, n: usize) -> String {
    let len = s.chars().count();
    if len >= n {
        s.chars().take(n).collect()
    } else {
        let mut out = s.to_string();
        out.push_str(&" ".repeat(n - len));
        out
    }
}

pub fn pad_left(s: &str, n: usize) -> String {
    let len = s.chars().count();
    if len >= n {
        s.chars().take(n).collect()
    } else {
        let mut out = " ".repeat(n - len);
        out.push_str(s);
        out
    }
}

/// `green < 80, yellow 80-89, red >= 90` — matches JSX usage-bar thresholds.
pub fn usage_color(used_pct: u32) -> Color {
    if used_pct >= 90 {
        p::red()
    } else if used_pct >= 80 {
        p::yellow()
    } else {
        p::fg()
    }
}

/// Same thresholds, but returns the bar fill color (green when ok).
pub fn usage_bar_color(used_pct: u32) -> Color {
    if used_pct >= 90 {
        p::red()
    } else if used_pct >= 80 {
        p::yellow()
    } else {
        p::green()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The compact form is for prose, so it carries no padding and says
    /// "idle" where the column form would show a dash.
    #[test]
    fn the_compact_rate_reads_as_prose() {
        assert_eq!(fmt_rate_compact(0.0), "idle");
        assert_eq!(fmt_rate_compact(512.0), "512 B/s");
        assert_eq!(fmt_rate_compact(2_000.0), "2 kB/s");
        assert_eq!(fmt_rate_compact(5_400_000.0), "5.4 MB/s");
        assert_eq!(fmt_rate_compact(2_100_000_000.0), "2.1 GB/s");
        for bps in [0.0, 1.0, 999.0, 1e6, 1e12] {
            assert!(!fmt_rate_compact(bps).starts_with(' '), "{bps}");
        }
    }
}
