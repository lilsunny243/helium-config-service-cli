use crate::{
    hex_field::{self, HexNetID},
    region::Region,
    DevaddrConstraint, Msg, Oui, PrettyJson, Result,
};
use anyhow::Context;
use clap::{Args, Parser, Subcommand};
use helium_crypto::PublicKey;
use std::path::PathBuf;

pub mod env;
pub mod org;
pub mod region_params;
pub mod route;
pub mod session_key_filter;

pub const ENV_CONFIG_HOST: &str = "HELIUM_CONFIG_HOST";
pub const ENV_KEYPAIR_BIN: &str = "HELIUM_KEYPAIR_BIN";
pub const ENV_NET_ID: &str = "HELIUM_NET_ID";
pub const ENV_OUI: &str = "HELIUM_OUI";
pub const ENV_MAX_COPIES: &str = "HELIUM_MAX_COPIES";

#[derive(Debug, Parser)]
#[command(name = "helium-config-cli")]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(
        global = true,
        long,
        env = ENV_CONFIG_HOST,
        default_value = "http://50.18.149.124:50051"
    )]
    pub config_host: String,

    #[arg(
        global = true,
        long,
        env = ENV_KEYPAIR_BIN,
        default_value = "./keypair.bin"
    )]
    pub keypair: PathBuf,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Environment
    Env {
        #[command(subcommand)]
        command: EnvCommands,
    },

    /// Route
    Route {
        #[command(subcommand)]
        command: RouteCommands,
    },

    /// Org
    Org {
        #[command(subcommand)]
        command: OrgCommands,
    },
    /// Session Key Filter
    #[command(alias = "skf")]
    SessionKeyFilter {
        #[command(subcommand)]
        command: SessionKeyFilterCommands,
    },
    /// Print a Subnet Mask for a given Devaddr Range
    SubnetMask(SubnetMask),
    /// Region Params
    RegionParams {
        #[command(subcommand)]
        command: RegionParamsCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum EnvCommands {
    /// Make Environment variables to ease repeated use
    Init,
    /// View information about your environment
    Info(EnvInfo),
    /// Make a new keypair
    GenerateKeypair(GenerateKeypair),
}

#[derive(Debug, Subcommand)]
pub enum RouteCommands {
    /// List all Routes for an OUI
    List(ListRoutes),
    /// Get a Route by ID
    Get(GetRoute),
    /// Create new Route
    New(NewRoute),
    /// Update Route component
    Update {
        #[command(subcommand)]
        command: RouteUpdateCommand,
    },
    /// Operate on EUIs for a Route
    Euis {
        #[command(subcommand)]
        command: EuiCommands,
    },
    /// Operate on Devaddrs for a Route
    Devaddrs {
        #[command(subcommand)]
        command: DevaddrCommands,
    },
    /// Remove Route
    Delete(DeleteRoute),
    /// Turn on routing for Route.
    ///
    /// The route field `locked` supersedes this setting.
    #[command(alias = "enable")]
    Activate(ActivateRoute),
    /// Turn off routing for a Route.
    ///
    /// the route field `locked` supersedes this setting.
    #[command(alias = "disable")]
    Deactivate(DeactivateRoute),
}

#[derive(Debug, Args)]
pub struct ListRoutes {
    #[arg(long, env = ENV_OUI)]
    pub oui: Oui,
    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct GetRoute {
    #[arg(short, long)]
    pub route_id: String,
    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
}

#[derive(Debug, Args)]
pub struct NewRoute {
    #[arg(long, env = ENV_NET_ID, default_value = "C00053")]
    pub net_id: HexNetID,
    #[arg(long, env = ENV_OUI)]
    pub oui: Oui,
    #[arg(long, env = ENV_MAX_COPIES, default_value = "5")]
    pub max_copies: u32,

    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct DeleteRoute {
    #[arg(short, long)]
    pub route_id: String,
    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct ActivateRoute {
    #[arg(short, long)]
    pub route_id: String,
    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct DeactivateRoute {
    #[arg(short, long)]
    pub route_id: String,
    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(long)]
    pub commit: bool,
}

#[derive(Debug, Subcommand)]
pub enum RouteUpdateCommand {
    /// Update max number of packets to buy.
    MaxCopies(UpdateMaxCopies),
    /// Update server destination details.
    Server(UpdateServer),
    /// Set the Route Protocol to Http
    Http(UpdateHttp),
    /// Set the Route Protocol to Gwmp (UDP)
    /// This will change the protocol to Gwmp AND add
    /// a region mapping if one was provided.
    AddGwmpRegion(AddGwmpRegion),
    /// Remove a region mapping from the Gwmp Protocol.
    /// This only works if the protocol is already gwmp.
    RemoveGwmpRegion(RemoveGwmpRegion),
    /// Set the Route Protocol to PacketRouter (GRPC)
    PacketRouter(UpdatePacketRouter),
}

#[derive(Debug, Args)]
pub struct UpdateMaxCopies {
    #[arg(short, long)]
    pub route_id: String,
    #[arg(short, long)]
    pub max_copies: u32,
    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct UpdateServer {
    #[arg(short, long)]
    pub route_id: String,
    #[arg(long)]
    pub host: String,
    #[arg(long)]
    pub port: u32,
    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct UpdateHttp {
    #[arg(short, long)]
    pub route_id: String,
    #[arg(short, long, default_value = "250")]
    pub dedupe_timeout: u32,
    /// Just the path part of the Server URL
    ///
    /// The rest will be taken from the Server {host}:{port}
    #[arg(short, long)]
    pub path: String,
    /// Authorization Header
    #[arg(short, long)]
    pub auth_header: Option<String>,

    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct UpdatePacketRouter {
    #[arg(short, long)]
    pub route_id: String,
    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct AddGwmpRegion {
    #[arg(short, long)]
    pub route_id: String,
    #[arg(value_enum)]
    pub region: Region,
    pub region_port: u32,

    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct RemoveGwmpRegion {
    #[arg(short, long)]
    pub route_id: String,
    #[arg(value_enum)]
    pub region: Region,

    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(long)]
    pub commit: bool,
}

#[derive(Debug, Subcommand)]
pub enum EuiCommands {
    /// Get all EUI pairs for a Route
    List(ListEuis),
    /// Add EUI pair to Route
    Add(AddEui),
    /// Remove EUI pair from Route
    Remove(RemoveEui),
    /// Remove ALL EUI Pairs from Route
    Clear(ClearEuis),
}

#[derive(Debug, Subcommand)]
pub enum DevaddrCommands {
    /// Get all Devaddr Ranges for a Route
    List(ListDevaddrs),
    /// Add Devaddr Range to Route
    Add(AddDevaddr),
    /// Remove Devaddr Range from Route
    Remove(RemoveDevaddr),
    /// Print subnet mask for all devaddr ranges in a Route.
    SubnetMask(RouteSubnetMask),
    /// Remove ALL Devaddr Ranges from Route
    Clear(ClearDevaddrs),
}

#[derive(Debug, Subcommand)]
pub enum OrgCommands {
    /// Get all Orgs
    List(ListOrgs),
    /// Get an Organization you own
    Get(GetOrg),
    /// Create a new Helium Organization
    CreateHelium(CreateHelium),
    /// Create a new Roaming Organization (admin only)
    CreateRoaming(CreateRoaming),
}

#[derive(Debug, Subcommand)]
pub enum SessionKeyFilterCommands {
    List(ListFilters),
    Get(GetFilters),
    Add(AddFilter),
    Remove(RemoveFilter),
}

#[derive(Debug, Args)]
pub struct ListFilters {
    #[arg(long, env = ENV_OUI)]
    pub oui: Oui,
    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
}

#[derive(Debug, Args)]
pub struct GetFilters {
    #[arg(long, env = ENV_OUI)]
    pub oui: Oui,
    #[arg(short, long, value_parser = hex_field::validate_devaddr)]
    pub devaddr: hex_field::HexDevAddr,
    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
}

#[derive(Debug, Args)]
pub struct AddFilter {
    #[arg(long, env = ENV_OUI)]
    pub oui: Oui,
    #[arg(short, long, value_parser = hex_field::validate_devaddr)]
    pub devaddr: hex_field::HexDevAddr,
    #[arg(short, long)]
    pub session_key: String,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(from_global)]
    pub keypair: PathBuf,
    /// Add EUI entry to a Route
    #[arg(short, long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct RemoveFilter {
    #[arg(long, env = ENV_OUI)]
    pub oui: Oui,
    #[arg(short, long, value_parser = hex_field::validate_devaddr)]
    pub devaddr: hex_field::HexDevAddr,
    #[arg(short, long)]
    pub session_key: String,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(from_global)]
    pub keypair: PathBuf,
    /// Add EUI entry to a Route
    #[arg(short, long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct ListEuis {
    #[arg(short, long)]
    pub route_id: String,
    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
}

#[derive(Debug, Args)]
pub struct AddEui {
    #[arg(short, long, value_parser = hex_field::validate_eui)]
    pub dev_eui: hex_field::HexEui,
    #[arg(short, long, value_parser = hex_field::validate_eui)]
    pub app_eui: hex_field::HexEui,
    #[arg(long)]
    pub route_id: String,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(from_global)]
    pub keypair: PathBuf,
    /// Add EUI entry to a Route
    #[arg(short, long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct RemoveEui {
    #[arg(short, long, value_parser = hex_field::validate_eui)]
    pub dev_eui: hex_field::HexEui,
    #[arg(short, long, value_parser = hex_field::validate_eui)]
    pub app_eui: hex_field::HexEui,
    #[arg(long)]
    pub route_id: String,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(from_global)]
    pub keypair: PathBuf,
    /// Remove EUI entry from the Route
    #[arg(short, long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct ClearEuis {
    #[arg(short, long)]
    pub route_id: String,
    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
    /// Remove ALL EUIs from a Route
    #[arg(short, long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct ListDevaddrs {
    #[arg(short, long)]
    pub route_id: String,
    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
}

#[derive(Debug, Args)]
pub struct AddDevaddr {
    #[arg(short, long, value_parser = hex_field::validate_devaddr)]
    pub start_addr: hex_field::HexDevAddr,
    #[arg(short, long, value_parser = hex_field::validate_devaddr)]
    pub end_addr: hex_field::HexDevAddr,
    #[arg(long)]
    pub route_id: String,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(from_global)]
    pub keypair: PathBuf,
    /// Add Devaddr entry to a Route
    #[arg(short, long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct RemoveDevaddr {
    #[arg(short, long, value_parser = hex_field::validate_devaddr)]
    pub start_addr: hex_field::HexDevAddr,
    #[arg(short, long, value_parser = hex_field::validate_devaddr)]
    pub end_addr: hex_field::HexDevAddr,
    #[arg(long)]
    pub route_id: String,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(from_global)]
    pub keypair: PathBuf,
    /// Remove Devaddr entry from a Route
    #[arg(short, long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct ClearDevaddrs {
    #[arg(short, long)]
    pub route_id: String,
    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
    /// Remove ALL Devaddrs from a route
    #[arg(short, long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct RouteSubnetMask {
    #[arg(short, long)]
    pub route_id: String,
    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
}

#[derive(Debug, Args)]
pub struct SubnetMask {
    #[arg(value_parser = hex_field::validate_devaddr)]
    pub start_addr: hex_field::HexDevAddr,
    #[arg(value_parser = hex_field::validate_devaddr)]
    pub end_addr: hex_field::HexDevAddr,
}

#[derive(Debug, Args)]
pub struct EnvInfo {
    #[arg(long, env = ENV_CONFIG_HOST, default_value="unset")]
    pub config_host: Option<String>,
    #[arg(long, env = ENV_KEYPAIR_BIN, default_value="unset")]
    pub keypair: Option<PathBuf>,
    #[arg(long, env = ENV_NET_ID)]
    pub net_id: Option<HexNetID>,
    #[arg(long, env = ENV_OUI)]
    pub oui: Option<Oui>,
    #[arg(long, env = ENV_MAX_COPIES)]
    pub max_copies: Option<u32>,
}

#[derive(Debug, Args)]
pub struct GenerateKeypair {
    #[arg(default_value = "./keypair.bin")]
    pub out_file: PathBuf,

    /// overwrite <out_file> if it already exists
    #[arg(long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct ListOrgs {
    #[arg(from_global)]
    pub config_host: String,
}

#[derive(Debug, Args)]
pub struct GetOrg {
    #[arg(long, env = "HELIUM_OUI")]
    pub oui: Oui,
    #[arg(from_global)]
    pub config_host: String,
}

#[derive(Debug, Args)]
pub struct CreateHelium {
    #[arg(long)]
    pub owner: PublicKey,
    #[arg(long)]
    pub payer: PublicKey,
    #[arg(long)]
    pub devaddr_count: u64,
    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(long)]
    pub commit: bool,
}

#[derive(Debug, Args)]
pub struct CreateRoaming {
    #[arg(long)]
    pub owner: PublicKey,
    #[arg(long)]
    pub payer: PublicKey,
    #[arg(long)]
    pub net_id: HexNetID,
    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(long)]
    pub commit: bool,
}

#[derive(Debug, Subcommand)]
pub enum RegionParamsCommands {
    /// Push a region params collection to the config service
    Push(PushRegionParams),
}

#[derive(Debug, Args)]
pub struct PushRegionParams {
    #[arg(value_enum)]
    pub region: Region,
    #[arg(long)]
    pub params_file: PathBuf,
    #[arg(long)]
    pub index_file: Option<PathBuf>,
    #[arg(from_global)]
    pub keypair: PathBuf,
    #[arg(from_global)]
    pub config_host: String,
    #[arg(long)]
    pub commit: bool,
}

pub fn subnet_mask(args: SubnetMask) -> Result<Msg> {
    let devaddr_range = DevaddrConstraint::new(args.start_addr, args.end_addr)?;
    Msg::ok(devaddr_range.to_subnet().pretty_json()?)
}

pub trait PathBufKeypair {
    fn to_keypair(&self) -> Result<helium_crypto::Keypair>;
}

impl PathBufKeypair for PathBuf {
    fn to_keypair(&self) -> Result<helium_crypto::Keypair> {
        let data = std::fs::read(self).context("reading keypair file")?;
        Ok(helium_crypto::Keypair::try_from(&data[..])?)
    }
}
