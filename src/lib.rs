pub mod output;

pub fn generate_progress_bar(current: f64, max: f64, use_color: bool) -> String {
    let width = 40;
    let filled = ((current / max) * width as f64) as usize;
    let empty = width - filled;

    let mut result = String::from("[");

    if use_color {
        let color = get_color_for_percentage(current);
        result.push_str(&color); // Start color
        result.push_str(&"█".repeat(filled));
        result.push_str("\x1B[0m"); // Reset color
    } else {
        result.push_str(&"█".repeat(filled));
    }

    result.push_str(&" ".repeat(empty));
    result.push(']');

    result
}

pub fn get_color_for_percentage(percent: f64) -> String {
    if percent > 90.0 {
        "\x1B[31m".to_string() // Red for >90%
    } else if percent > 75.0 {
        "\x1B[33m".to_string() // Yellow for >75%
    } else {
        "\x1B[32m".to_string() // Green for normal
    }
}

pub fn calculate_percentage(used: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        (used as f64 / total as f64) * 100.0
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}
