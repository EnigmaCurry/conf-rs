//! See completion_example.sh for a test script for this example.
//! This requires the Cargo "completion" feature to be enabled.
#![cfg(feature = "completion")]

use conf::{
    Conf, Subcommands, ValueEnum,
    completion::{Shell, write_completion},
};
use std::{fmt, str::FromStr};

/// Top-level CLI wrapper
#[derive(Conf, Debug)]
#[conf(name = "completion_example")]
pub struct Cli {
    #[conf(subcommands)]
    pub cmd: CliCommand,
}

#[derive(Subcommands, Debug)]
pub enum CliCommand {
    /// Run the (simulated) nuclear reactor control system
    Run(ReactorConfig),

    /// Reactor control simulation commands
    Reactor(ReactorCli),

    /// Print a shell completion script to stdout
    Completion(CompletionArgs),
}

#[derive(Conf, Debug)]
pub struct CompletionArgs {
    /// Shell to generate completions for (bash|elvish|fish|powershell|zsh)
    #[conf(pos, value_enum)]
    pub shell: Shell,
}

/// Global configuration for the simulated reactor
#[derive(Conf, Debug)]
pub struct ReactorConfig {
    /// Reactor name (for logs)
    #[conf(long, default_value = "NTR-01")]
    pub name: String,

    /// Simulation step size in milliseconds
    #[conf(long, default_value = "250")]
    pub step_ms: u64,
}

/// Wrapper so `reactor ...` can have its own subcommands.
#[derive(Conf, Debug)]
pub struct ReactorCli {
    #[conf(subcommands)]
    pub cmd: ReactorCmd,
}

/// Reactor control commands
#[derive(Subcommands, Debug)]
pub enum ReactorCmd {
    /// Set the reactor operational mode
    SetMode(SetModeArgs),

    /// Adjust control rod bank
    Rods(RodsArgs),

    /// Change coolant system settings
    Coolant(CoolantArgs),

    /// Run a simulated SCRAM (rapid shutdown)
    Scram(ScramArgs),

    /// Print a quick reactor status line
    Status(StatusArgs),
}

/// Reactor operational mode (user-defined enum, derives ValueEnum)
#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ReactorMode {
    /// Low power / training mode
    Standby,
    /// Normal power operations
    Power,
    /// Maximum output (unsafe in real life)
    Peak,
    /// Maintenance / locked-out controls
    Maintenance,
}

/// Which control-rod bank to adjust (user-defined enum, derives ValueEnum)
#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum RodBank {
    /// Primary regulating rods
    Regulating,
    /// Shutdown (safety) rods
    Shutdown,
}

/// Coolant circuit selection (user-defined enum, derives ValueEnum)
#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum CoolantLoop {
    /// Primary loop (reactor core)
    Primary,
    /// Secondary loop (steam generator)
    Secondary,
}

/// How to trigger a SCRAM (user-defined enum, derives ValueEnum)
#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ScramReason {
    /// Operator initiated
    Manual,
    /// Simulated over-temperature trip
    OverTemp,
    /// Simulated loss of coolant flow
    LossOfFlow,
    /// Simulated neutron flux trip
    HighFlux,
}

/// Simple shared parse error for the example.
#[derive(Debug, Clone)]
pub struct ParseEnumError {
    input: String,
    expected: &'static str,
}
impl fmt::Display for ParseEnumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid value {:?} (expected: {})",
            self.input, self.expected
        )
    }
}
impl std::error::Error for ParseEnumError {}

fn norm(s: &str) -> String {
    s.trim().to_ascii_lowercase()
}

// Conf still needs FromStr for value types.
impl FromStr for ReactorMode {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match norm(s).as_str() {
            "standby" => Ok(Self::Standby),
            "power" => Ok(Self::Power),
            "peak" => Ok(Self::Peak),
            "maintenance" => Ok(Self::Maintenance),
            _ => Err(ParseEnumError {
                input: s.to_string(),
                expected: "standby|power|peak|maintenance",
            }),
        }
    }
}
impl fmt::Display for ReactorMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Standby => "standby",
            Self::Power => "power",
            Self::Peak => "peak",
            Self::Maintenance => "maintenance",
        })
    }
}

impl FromStr for RodBank {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match norm(s).as_str() {
            "regulating" => Ok(Self::Regulating),
            "shutdown" => Ok(Self::Shutdown),
            _ => Err(ParseEnumError {
                input: s.to_string(),
                expected: "regulating|shutdown",
            }),
        }
    }
}
impl fmt::Display for RodBank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Regulating => "regulating",
            Self::Shutdown => "shutdown",
        })
    }
}

impl FromStr for CoolantLoop {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match norm(s).as_str() {
            "primary" => Ok(Self::Primary),
            "secondary" => Ok(Self::Secondary),
            _ => Err(ParseEnumError {
                input: s.to_string(),
                expected: "primary|secondary",
            }),
        }
    }
}
impl fmt::Display for CoolantLoop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
        })
    }
}

impl FromStr for ScramReason {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match norm(s).as_str() {
            "manual" => Ok(Self::Manual),
            "overtemp" | "over-temp" => Ok(Self::OverTemp),
            "lossofflow" | "loss-of-flow" => Ok(Self::LossOfFlow),
            "highflux" | "high-flux" => Ok(Self::HighFlux),
            _ => Err(ParseEnumError {
                input: s.to_string(),
                expected: "manual|over-temp|loss-of-flow|high-flux",
            }),
        }
    }
}
impl fmt::Display for ScramReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Manual => "manual",
            Self::OverTemp => "over-temp",
            Self::LossOfFlow => "loss-of-flow",
            Self::HighFlux => "high-flux",
        })
    }
}

#[derive(Conf, Debug)]
pub struct SetModeArgs {
    /// New mode (standby|power|peak|maintenance)
    #[conf(pos, value_enum)]
    pub mode: ReactorMode,
}

#[derive(Conf, Debug)]
pub struct RodsArgs {
    /// Which rod bank to move (regulating|shutdown)
    #[conf(pos, value_enum)]
    pub bank: RodBank,

    /// Target insertion percentage (0-100)
    #[conf(long, default_value = "50")]
    pub insert_pct: u8,
}

#[derive(Conf, Debug)]
pub struct CoolantArgs {
    /// Which loop to adjust (primary|secondary)
    #[conf(pos, value_enum)]
    pub loop_: CoolantLoop,

    /// Pump speed percentage (0-100)
    #[conf(long, default_value = "75")]
    pub pump_pct: u8,

    /// Simulated valve opening percentage (0-100)
    #[conf(long, default_value = "60")]
    pub valve_pct: u8,
}

#[derive(Conf, Debug)]
pub struct ScramArgs {
    /// Why we are scramming (manual|over-temp|loss-of-flow|high-flux)
    #[conf(pos, value_enum)]
    pub reason: ScramReason,

    /// Optional delay before scram (simulation only)
    #[conf(long, default_value = "0")]
    pub delay_ms: u64,
}

#[derive(Conf, Debug)]
pub struct StatusArgs {
    /// Print status repeatedly for this many milliseconds (simulation only)
    #[conf(long, default_value = "0")]
    pub watch_ms: u64,
}

fn main() {
    match Cli::parse().cmd {
        CliCommand::Completion(args) => {
            write_completion::<Cli, _>(args.shell, None, &mut std::io::stdout())
                .expect("Expected to output shell script");
        }

        CliCommand::Run(cfg) => {
            eprintln!(
                "Starting reactor sim '{}' with step={}ms",
                cfg.name, cfg.step_ms
            );
        }

        CliCommand::Reactor(ReactorCli { cmd }) => match cmd {
            ReactorCmd::SetMode(args) => eprintln!("Setting reactor mode -> {}", args.mode),
            ReactorCmd::Rods(args) => eprintln!(
                "Moving {} rods to {}% insertion",
                args.bank, args.insert_pct
            ),
            ReactorCmd::Coolant(args) => eprintln!(
                "Adjusting {} loop: pump={}%, valve={}%",
                args.loop_, args.pump_pct, args.valve_pct
            ),
            ReactorCmd::Scram(args) => {
                eprintln!("SCRAM! reason={} delay={}ms", args.reason, args.delay_ms)
            }
            ReactorCmd::Status(args) => eprintln!("STATUS watch={}ms (simulated)", args.watch_ms),
        },
    }
}
