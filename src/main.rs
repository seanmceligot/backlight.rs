use std::env;
use std::fs;
use std::io::{self};
use std::path::Path;

fn show_brightness(pcnt: i32) {
    let icon = match pcnt {
        0 => '',
        1..=10 => '',
        11..=30 => '',
        31..=50 => '',
        51..=60 => '',
        61..=70 => '',
        71..=80 => '',
        81..=90 => '',
        91..=99 => '',
        100 => '',                        // Assume '' is the icon for 89% to 100%
        _ => panic!("Invalid percentage"), // This should never happen if percentages are between 0 and 100
    };
    println!("{} {}%", icon, pcnt);
}

fn to_percent(value: i32, max: i32) -> i32 {
    if max == 0 {
        eprintln!("Cannot divide by zero");
        std::process::exit(1);
    }
    (value as f64 / max as f64 * 100.0).round() as i32
}

fn from_percent(percentage: i32, max: i32) -> i32 {
    (percentage as f64 / 100.0 * max as f64).round() as i32
}

fn main() -> io::Result<()> {
    let backlight_path = if Path::new("/sys/class/backlight/amdgpu_bl0/").exists() {
        "/sys/class/backlight/amdgpu_bl0/"
    } else if Path::new("/sys/class/backlight/intel_backlight/").exists() {
        "/sys/class/backlight/intel_backlight/"
    } else {
        eprintln!("backlight not found");
        std::process::exit(1);
    };

    let brightness_path = format!("{}brightness", backlight_path);
    let max_brightness_path = format!("{}max_brightness", backlight_path);

    let current_brightness = fs::read_to_string(&brightness_path)?;
    let max_brightness = fs::read_to_string(max_brightness_path)?;

    let current = current_brightness
        .trim()
        .parse::<i32>()
        .expect("Failed to parse current brightness");
    let max = max_brightness
        .trim()
        .parse::<i32>()
        .expect("Failed to parse max brightness");

    let current_pcnt = to_percent(current, max);
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        show_brightness(current_pcnt);
    } else if args.len() == 2 {
        let increment = match args[1].as_str() {
            "up" => {
                if current_pcnt > 10 && current_pcnt < 90 {
                    5
                } else {
                    1
                }
            }
            "down" => {
                if current_pcnt > 10 && current_pcnt < 90 {
                    -5
                } else {
                    -1
                }
            }
            _ => {
                eprintln!("Invalid argument: use 'up' or 'down'");
                std::process::exit(1);
            }
        };

        let new_pcnt = calc_new_pcnt(current_pcnt, increment);
        if new_pcnt != current_pcnt {
            let new_value = from_percent(new_pcnt, max);
            //println!( "old {} %{} new {} %{}", current, current_pcnt, new_value, new_pcnt);
            fs::write(brightness_path, new_value.to_string())?;
        }
        show_brightness(new_pcnt);
    } else {
        eprintln!("usage: {} up", args[0]);
        eprintln!("usage: backlight down");
        eprintln!("usage: backlight");
        std::process::exit(1);
    }
    Ok(())
}
fn calc_new_pcnt(current_pcnt: i32, increment: i32) -> i32 {
    let new_pcnt = current_pcnt + increment;
    if new_pcnt < 0 {
        0
    } else if new_pcnt > 100 {
        100
    } else {
        new_pcnt
    }
}
#[test]
fn test() {
    assert!(calc_new_pcnt(20, 5) == 25);
    assert!(calc_new_pcnt(20, -5) == 15);
    assert!(calc_new_pcnt(0, 5) == 5);
    assert!(calc_new_pcnt(100, -5) == 95);
}
