// Copyright (c) 2022 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0
//

use clap::{Args, Parser, Subcommand};
use thiserror::Error;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[clap(name = "kata-ctl", author, about = "Kata Containers control tool")]
pub struct KataCtlCli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Test if system can run Kata Containers
    Check(CheckArgument),

    /// Directly assign a volume to Kata Containers to manage
    DirectVolume(DirectVolumeCommand),

    /// Display settings
    Env(EnvArgument),

    /// Enter into guest VM by debug console
    Exec(ExecArguments),

    /// Manage VM factory
    Factory,

    /// Manage guest VM iptables
    Iptables(IptablesCommand),

    /// Gather metrics associated with infrastructure used to run a sandbox
    Metrics(MetricsCommand),

    /// Display version details
    Version,
}

#[derive(Debug, Args, Error)]
#[error("Argument is not valid")]
pub struct CheckArgument {
    #[clap(subcommand)]
    pub command: CheckSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum CheckSubCommand {
    /// Run all checks
    All,

    /// Run all checks but excluding network checks.
    NoNetworkChecks,

    /// Only compare the current and latest available versions
    CheckVersionOnly,

    /// List official release packages
    OnlyListReleases,

    /// List all official and pre-release packages
    IncludeAllReleases,

    /// List all available checks
    List,
}

#[derive(Debug, Args)]
pub struct EnvArgument {
    /// Format output as JSON
    #[clap(long)]//arg
    pub json: bool,
    /// File to write env output to
    #[clap(short = 'f', long = "file")]//arg
    pub file: Option<String>,
}
#[derive(Debug, Args)]
pub struct MetricsCommand {
    #[clap(subcommand)]
    pub metrics_cmd: MetricsSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum MetricsSubCommand {
    /// Arguments for metrics
    MetricsArgs,
}

#[derive(Debug, Args)]//Parser
pub struct IptablesCommand {
    #[clap(subcommand)]
    pub iptables: IpTablesArguments,
}

impl IptablesCommand {
    pub fn subcommand(&self) -> &IpTablesArguments {
        &self.iptables//command
    }
}

#[derive(Debug, Subcommand)]//Clap
pub enum IpTablesArguments {
    /// Configure iptables
    /// Getters
    #[clap(about = "Get iptables from the Kata Containers guest")]
    Get{
        #[clap(long = "sand-box", value_name = "ID", required = true, 
        takes_value = true, help = "The target sandbox for getting the iptables")]
        sandbox_id:String,

        #[clap(long = "v6", help = "Indicate we're requesting ipv6 iptables")]
        v6:bool,
    },

    //Setters
    Set{
        #[clap(long = "sand-box", value_name = "ID", required = true, 
        takes_value = true, help = "The target sandbox for setting the iptables")]
        sandbox_id:String,

        #[clap(long = "v6", help = "Indicate we're requesting ipv6 iptables")]
        v6:bool,

        #[clap(name = "FILE", required = true, takes_value = true, help = "The iptables file to set")]
        file: String,
    },
}

impl FromStr for IpTablesArguments{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s{
            "Get" => Ok(IpTablesArguments::Get {sandbox_id: String::new(), v6:false}),
            "Set" => Ok(IpTablesArguments::Set {sandbox_id: String::new(), v6:false, file: String::new()}),
            _=> Err(format!("Invalid argument: {}", s)),
        }
    }
}

#[derive(Debug, Args)]
pub struct DirectVolumeCommand {
    #[clap(subcommand)]
    pub directvol_cmd: DirectVolSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum DirectVolSubcommand {
    /// Add a direct assigned block volume device to the Kata Containers runtime
    Add(DirectVolAddArgs),

    /// Remove a direct assigned block volume device from the Kata Containers runtime
    Remove(DirectVolRemoveArgs),

    /// Get the filesystem stat of a direct assigned volume
    Stats(DirectVolStatsArgs),

    /// Resize a direct assigned block volume
    Resize(DirectVolResizeArgs),
}

#[derive(Debug, Args)]
pub struct DirectVolAddArgs {
    pub volume_path: String,
    pub mount_info: String,
}

#[derive(Debug, Args)]
pub struct DirectVolRemoveArgs {
    pub volume_path: String,
}

#[derive(Debug, Args)]
pub struct DirectVolStatsArgs {
    pub volume_path: String,
}

#[derive(Debug, Args)]
pub struct DirectVolResizeArgs {
    pub volume_path: String,
    pub resize_size: u64,
}

#[derive(Debug, Args)]
pub struct ExecArguments {
    /// pod sandbox ID.
    pub sandbox_id: String,
    #[clap(short = 'p', long = "kata-debug-port", default_value_t = 1026)]
    /// kata debug console vport same as configuration, default is 1026.
    pub vport: u32,
}
