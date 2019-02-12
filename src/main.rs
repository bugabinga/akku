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

    let status_icon = match status {
        Status::CHARGING => "↑↑",
        Status::DISCHARGING => "↓↓",
        Status::FULL => "ø",
        Status::UNKNOWN => "?",
    };

    println!("{}{}{}%", capacity_level, status_icon, capacity);
    Ok(())
}
