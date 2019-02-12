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
    println!("{}, today()?);
    println!("{}", battery()?);
    println!("{}", brightness()?);
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
    let capacity = fs::read_to_string(bat0.join("capacity"))?;
    let capacity = capacity.trim();

    let capacity_level = match capacity_level {
        CapacityLevel::FULL => "±",
        CapacityLevel::NORMAL => "¤",
        CapacityLevel::HIGH => "¯",
        CapacityLevel::LOW => "·",
        CapacityLevel::CRITICAL => "¡",
        CapacityLevel::UNKNOWN => "¿",
    };

    let status = match status {
        Status::CHARGING => "↑↑",
        Status::DISCHARGING => "↓↓",
        Status::FULL => "ø",
        Status::UNKNOWN => "?",
    };

    Ok(format!("{}{}{}%", capacity_level, status, capacity))
}

fn brightness() -> Result<String> {
    let intel_brightness = Path::new("/sys/class/intel_brightness/");

    let brightness = fs::read_to_string(intel_brightness.join("brightness"))?;
    let brightness = brightness.trim().parse::<u16>().unwrap();

    let max_brightness = fs::read_to_string(intel_brightness.join("max_brightness"))?;
    let max_brightness = max_brightness.trim().parse::<u16>().unwrap();

    Ok(format!("{}%", brightness / max_brightness * 100))
}

fn today() -> Result<String> {
    use chrono::prelude::*;
    let local = Local::now();

    Ok(format!("{}.{} {}:{}", local.day(), local.month(), local.hour(), local.minute()))
}

