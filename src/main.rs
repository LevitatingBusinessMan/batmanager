use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(flatten)]
    mode: Option<Mode>,
}

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
struct Mode {
    #[arg(short, long, value_name("on/off"))]
    conservation: Option<Setting>,
    #[arg(short, long, value_name("on/off"))]
    rapid: Option<Setting>,
    #[arg(short, long, value_name("1/2/3"))]
    performance: Option<PerformanceMode>,
}

#[derive(Clone, clap::ValueEnum, Debug)]
enum Setting {
    On,
    Off
}


#[derive(Clone, clap::ValueEnum, Debug)]
enum PerformanceMode {
    #[value(alias="1", name="extreme-performance (1)")]
    ExtremePerformance,
    #[value(alias="2", name="intelligent-cooling (2)")]
    IntelligentCooling,
    #[value(alias="3", name="battery-saving (3)")]
    BatterySaving,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
