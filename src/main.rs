use std::{fs::{File, OpenOptions}, io::{Write, Read}};
use anyhow::{anyhow, Context};
use clap::Parser;
use colored::Colorize;
use anyhow::Result;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None, about="Tool for configuring Lenovo battery settings")]
#[command(propagate_version = true)]
struct Args {
    #[arg(short, long, value_name("on/off"), help="get or set the conservation mode")]
    conservation: Option<Option<Setting>>,
    
    #[arg(short, long, value_name("on/off"), help="get or set the rapid charing mode")]
    rapid: Option<Option<Setting>>,
    
    #[arg(short, long, value_name("1/2/3"), help="get or set the performance mode")]
    performance: Option<Option<PerformanceMode>>,
}

#[derive(Clone, clap::ValueEnum, Debug)]
pub enum Setting {
    #[value(alias="1")]
    On,
    #[value(alias="0")]
    Off
}

impl std::fmt::Display for Setting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Setting::On => "on".green(),
            Setting::Off => "off".red(),
        })
    }
}

#[derive(Clone, clap::ValueEnum, Debug)]
pub enum PerformanceMode {
    #[value(alias="1", name="intelligent-cooling (1)")]
    IntelligentCooling,
    #[value(alias="2", name="extreme-performance (2)")]
    ExtremePerformance,
    #[value(alias="3", name="battery-saving (3)")]
    BatterySaving,
}

impl std::fmt::Display for PerformanceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PerformanceMode::IntelligentCooling => "Intelligent Cooling".blue(),
            PerformanceMode::BatterySaving => "Battery Saving".green(),
            PerformanceMode::ExtremePerformance => "Extreme Performance".red(),
        })
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    sudo::escalate_if_needed().expect("Failed to escalate priveleges");
    
    if args.conservation.is_none() && args.rapid.is_none() && args.performance.is_none() {
        println!("{}", "Battery configuration status".bold());
        println!("* Conservation Mode: {}", get_conservation()?);
        println!("* Rapid Charge: {}", get_rapid()?);
        println!("* Performance Mode: {}", get_performance()?);
        return Ok(());
    }
    
    if let Some(conservation) = args.conservation {
        if let Some(setting) = conservation {
            set_conservation(setting)?;
        }
        println!("Conservation Mode: {}", get_conservation()?);
    }

    if let Some(rapid) = args.rapid {
        if let Some(setting) = rapid {
            set_rapid(setting)?;
        }
        println!("Rapid Charge: {}", get_rapid()?);
    }

    if let Some(performance) = args.performance {
        if let Some(setting) = performance {
            set_performance(setting)?;
        }
        println!("Performance Mode: {}", get_performance()?);
    }

    Ok(())
}

pub fn set_conservation(setting: Setting) -> Result<()> {
    let mut file = open_file()?;
    let value = match setting {
        Setting::On => "0x03",
        Setting::Off => "0x05",
    };
    file.write(("\\_SB.PCI0.LPC0.EC0.VPC0.SBMC ".to_owned() + value).as_bytes()).expect("Failed writing to acpi_call");
    Ok(())
}

pub fn set_rapid(setting: Setting) -> Result<()> {
    let mut file = open_file()?;
    let value = match setting {
        Setting::On => "0x07",
        Setting::Off => "0x08",
    };
    file.write(("\\_SB.PCI0.LPC0.EC0.VPC0.SBMC ".to_owned() + value).as_bytes()).expect("Failed writing to acpi_call");
    Ok(())
}

pub fn set_performance(setting: PerformanceMode) -> Result<()> {
    let mut file = open_file()?;
    let value = match setting {
        PerformanceMode::IntelligentCooling => "0x000FB001",
        PerformanceMode::ExtremePerformance => "0x0012B001",
        PerformanceMode::BatterySaving => "0x0013B001",
    };
    file.write(("\\_SB.PCI0.LPC0.EC0.VPC0.DYTC ".to_owned() + value).as_bytes()).expect("Failed writing to acpi_call");
    Ok(())
}
pub fn get_conservation() -> Result<Setting>{
    let mut file = open_file()?;
    file.write(b"\\_SB.PCI0.LPC0.EC0.BTSM").expect("Failed writing to acpi_call");
    match get_acpi_response(file)? {
        0 => Ok(Setting::Off),
        1 => Ok(Setting::On),
        _ => Err(anyhow!("Invalid byte read from acpi_call"))
    }
}

pub fn get_rapid() -> Result<Setting> {
    let mut file = open_file()?;
    file.write(b"\\_SB.PCI0.LPC0.EC0.QCHO").expect("Failed writing to acpi_call");
    match get_acpi_response(file)? {
        0 => Ok(Setting::Off),
        1 => Ok(Setting::On),
        _ => Err(anyhow!("Invalid byte read from acpi_call"))
    }
}

pub fn get_performance() -> Result<PerformanceMode> {
    let mut file = open_file()?;
    file.write(b"\\_SB.PCI0.LPC0.EC0.SPMO").expect("Failed writing to acpi_call");
    match get_acpi_response(file)? {
        0 => Ok(PerformanceMode::IntelligentCooling),
        1 => Ok(PerformanceMode::ExtremePerformance),
        2 => Ok(PerformanceMode::BatterySaving),
        _ => Err(anyhow!("Invalid byte read from acpi_call"))
    }
}

fn get_acpi_response(mut file: File) -> Result<u8> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let buf = buf.strip_prefix("0x").ok_or(anyhow!("Invalid data returned from /proc/acpi/call"))?;
    let buf = buf.strip_suffix("\0").ok_or(anyhow!("Invalid data returned from /proc/acpi/call"))?;
    return Ok((&buf).parse()?);
}

// At first I was opening it once and storing it in a OnceLock
// But acpi_call only works when you repoen it per command
fn open_file() -> Result<File, anyhow::Error> {
    Ok(
        OpenOptions::new().read(true).write(true).open("/proc/acpi/call")
        .context("Failed to open /proc/acpi/call.\nIs is the 'acpi_call' module loaded?")?
    )
}
