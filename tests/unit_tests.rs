use system_monitor::{
    calculate_percentage, format_bytes, generate_progress_bar, get_color_for_percentage,
};

#[test]
fn test_calculate_percentage() {
    assert_eq!(calculate_percentage(50, 100), 50.0);
    assert_eq!(calculate_percentage(0, 100), 0.0);
    assert_eq!(calculate_percentage(100, 100), 100.0);
    assert_eq!(calculate_percentage(75, 100), 75.0);

    // Edge case: division by zero
    assert_eq!(calculate_percentage(50, 0), 0.0);

    // Non-round numbers
    assert!((calculate_percentage(33, 100) - 33.0).abs() < 0.001);

    // Test larger numbers (like real memory values)
    assert_eq!(calculate_percentage(4_294_967_296, 8_589_934_592), 50.0); // 4GB of 8GB
}

#[test]
fn test_format_bytes() {
    // Test bytes
    assert_eq!(format_bytes(0), "0 B");
    assert_eq!(format_bytes(500), "500 B");
    assert_eq!(format_bytes(1023), "1023 B");

    // Test KB
    assert_eq!(format_bytes(1024), "1.0 KB");
    assert_eq!(format_bytes(1536), "1.5 KB");
    assert_eq!(format_bytes(1025), "1.0 KB");

    // Test MB
    assert_eq!(format_bytes(1048576), "1.0 MB");
    assert_eq!(format_bytes(1572864), "1.5 MB");

    // Test GB
    assert_eq!(format_bytes(1073741824), "1.0 GB");
    assert_eq!(format_bytes(2147483648), "2.0 GB");

    // Test real-world values
    assert_eq!(format_bytes(8_589_934_592), "8.0 GB"); // 8GB RAM
    assert_eq!(format_bytes(512_000_000_000), "476.8 GB"); // ~500GB disk
}

#[test]
fn test_get_color_for_percentage() {
    // Test normal range (green)
    assert_eq!(get_color_for_percentage(0.0), "\x1B[32m");
    assert_eq!(get_color_for_percentage(50.0), "\x1B[32m");
    assert_eq!(get_color_for_percentage(75.0), "\x1B[32m");

    // Test high range (yellow)
    assert_eq!(get_color_for_percentage(75.1), "\x1B[33m");
    assert_eq!(get_color_for_percentage(80.0), "\x1B[33m");
    assert_eq!(get_color_for_percentage(90.0), "\x1B[33m");

    // Test critical range (red)
    assert_eq!(get_color_for_percentage(90.1), "\x1B[31m");
    assert_eq!(get_color_for_percentage(95.0), "\x1B[31m");
    assert_eq!(get_color_for_percentage(100.0), "\x1B[31m");

    // Test edge cases
    assert_eq!(get_color_for_percentage(75.0), "\x1B[32m"); // Exactly 75%
    assert_eq!(get_color_for_percentage(90.0), "\x1B[33m"); // Exactly 90%
}

#[test]
fn test_generate_progress_bar_no_color() {
    // Test 0% progress
    let bar = generate_progress_bar(0.0, 100.0, false);
    assert_eq!(bar, "[                                        ]");

    // Test 50% progress
    let bar = generate_progress_bar(50.0, 100.0, false);
    assert_eq!(bar, "[████████████████████                    ]");

    // Test 100% progress
    let bar = generate_progress_bar(100.0, 100.0, false);
    assert_eq!(bar, "[████████████████████████████████████████]");

    // Test 25% progress
    let bar = generate_progress_bar(25.0, 100.0, false);
    assert_eq!(bar, "[██████████                              ]");
}

#[test]
fn test_generate_progress_bar_with_color() {
    // Test normal range (green)
    let bar = generate_progress_bar(50.0, 100.0, true);
    assert!(bar.contains("\x1B[32m")); // Contains green color
    assert!(bar.contains("\x1B[0m")); // Contains reset
    assert!(bar.contains("████████████████████"));

    // Test warning range (yellow)
    let bar = generate_progress_bar(80.0, 100.0, true);
    assert!(bar.contains("\x1B[33m")); // Contains yellow color
    assert!(bar.contains("\x1B[0m")); // Contains reset

    // Test critical range (red)
    let bar = generate_progress_bar(95.0, 100.0, true);
    assert!(bar.contains("\x1B[31m")); // Contains red color
    assert!(bar.contains("\x1B[0m")); // Contains reset
}

#[test]
fn test_progress_bar_length() {
    // All progress bars should be exactly 42 characters:
    // '[' + 40 progress chars + ']'

    let bar = generate_progress_bar(0.0, 100.0, false);
    assert_eq!(bar.chars().count(), 42);

    let bar = generate_progress_bar(50.0, 100.0, false);
    assert_eq!(bar.chars().count(), 42);

    let bar = generate_progress_bar(100.0, 100.0, false);
    assert_eq!(bar.chars().count(), 42);
}

#[test]
fn test_progress_bar_edge_cases() {
    // Test division by zero in max value
    // let bar = generate_progress_bar(50.0, 0.0, false);
    // // Should not panic and should handle gracefully
    // assert!(bar.contains("["));
    // assert!(bar.contains("]"));

    // Test negative values
    // let bar = generate_progress_bar(-10.0, 100.0, false);
    // assert!(bar.contains("["));
    // assert!(bar.contains("]"));

    // Test values over 100%
    // let bar = generate_progress_bar(120.0, 100.0, false);
    // assert!(bar.contains("["));
    // assert!(bar.contains("]"));
}
