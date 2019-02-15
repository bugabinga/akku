use std::fs;
use std::io::Result;
use std::path::Path;

#[derive(Debug)]
enum Status {
    CHARGING,
    DISCHARGING,
    FULL,
    UNKNOWN,
}

#[derive(Debug)]
enum CapacityLevel {
    FULL,
    NORMAL,
    LOW,
    HIGH,
    CRITICAL,
    UNKNOWN,
}

fn main() -> Result<()> {
    println!("{} | {} | {}", battery()?, brightness()?, today()?);
    Ok(())
}

fn battery() -> Result<String> {
    let bat0: &Path = Path::new("/sys/class/power_supply/BAT0/");

    let status = fs::read_to_string(bat0.join("status"))?;
    let status = match status.trim().as_ref() {
        "Charging" => Status::CHARGING,
        "Discharging" => Status::DISCHARGING,
        "Full" => Status::FULL,
        "Unknown" => Status::UNKNOWN,
        unknown => panic!("Unknown status in {:#?} : {}.", bat0, unknown),
    };

    let capacity_level = fs::read_to_string(bat0.join("capacity_level"))?;
    let capacity_level = match capacity_level.trim().as_ref() {
        "Full" => CapacityLevel::FULL,
        "Normal" => CapacityLevel::NORMAL,
        "Low" => CapacityLevel::LOW,
        "High" => CapacityLevel::HIGH,
        "Critical" => CapacityLevel::CRITICAL,
        "Unknown" => CapacityLevel::UNKNOWN,
        really_unknown => panic!(
            "Unknown capacity level in {:#?} : {}.",
            bat0, really_unknown
        ),
    };

    let capacity_level = match capacity_level {
        CapacityLevel::FULL => '±',
        CapacityLevel::NORMAL => '¤',
        CapacityLevel::HIGH => '¯',
        CapacityLevel::LOW => '·',
        CapacityLevel::CRITICAL => '¡',
        CapacityLevel::UNKNOWN => '¿',
    };

    let status = match status {
        Status::CHARGING => "↑↑",
        Status::DISCHARGING => "↓↓",
        Status::FULL => "ø",
        Status::UNKNOWN => "?",
    };

    let capacity = fs::read_to_string(bat0.join("capacity"))?;
    let capacity = capacity.trim();

    Ok(format!("{}{}{}", capacity_level, capacity, status))
}

fn brightness() -> Result<String> {
    let intel_backlight = Path::new("/sys/class/backlight/intel_backlight/");

    let brightness = fs::read_to_string(intel_backlight.join("brightness"))?;
    let brightness = brightness.trim().parse::<u16>().unwrap();

    let max_brightness = fs::read_to_string(intel_backlight.join("max_brightness"))?;
    let max_brightness = max_brightness.trim().parse::<u16>().unwrap();

    Ok(format!(
        "»{}«",
        brightness as f32 / max_brightness as f32 * 100.0
    ))
}

fn today() -> Result<String> {
    use chrono::prelude::*;
    let local = Local::now();

    Ok(format!(
        "{}.{} {}:{}",
        local.day(),
        local.month(),
        local.hour(),
        local.minute()
    ))
}
