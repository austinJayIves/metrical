use std::env::var;
use crate::metric::Namespace;
use crate::protocol::{NetworkProtocol, Protocol, Compression};
use crate::MetricalError;
use std::net::IpAddr;
use crate::configuration::{ConfigurationBuilder, Configuration};
use crate::collector::BufferedCollector;
use crate::shared::set_boxed_collector;

static FLUSH_INTERVAL_ENV: &str = "METRICAL_FLUSH_INTERVAL";
static FLUSH_AMOUNT_ENV: &str = "METRICAL_FLUSH_AMOUNT";
static NAMESPACE_ENV: &str = "METRICAL_NAMESPACE";
static NETWORK_PROTOCOL_ENV: &str = "METRICAL_NETWORK_PROTOCOL";
static SEND_METHOD_ENV: &str = "METRICAL_SEND_METHOD";
static NETWORK_DESTINATION_ENV: &str = "METRICAL_NETWORK_DESTINATION";

pub fn from_env(prefix: Option<&str>) -> Result<(), MetricalError>{
    let prefix = match prefix {
        Some("") => {
            "".to_owned()
        }
        Some(prefix) => {
            format!("{}_", prefix)
        },
        None => {
            "".to_owned()
        }
    };

    let flush_interval: Option<u64> = match var(format!("{}{}", prefix, FLUSH_INTERVAL_ENV)) {
        Ok(value) => match value.parse::<u64>() {
            Ok(v) => Some(v),
            Err(_) => None
        },
        Err(_) => None
    };

    let flush_amount: Option<usize> = match var(format!("{}{}", prefix, FLUSH_AMOUNT_ENV)) {
        Ok(value) => match value.parse::<usize>() {
            Ok(v) => Some(v),
            Err(_) => None
        },
        Err(_) => None
    };

    let namespace: Option<Namespace> = match var(format!("{}{}", prefix, NAMESPACE_ENV)) {
        Ok(value) => Some(Namespace(value)),
        Err(_) => None
    };

    let network_protocol: NetworkProtocol = match var(
        format!("{}{}", prefix, NETWORK_PROTOCOL_ENV)
    ) {
        Ok(value) => match value.to_lowercase().as_ref() {
            "udp" => NetworkProtocol::UDP,
            "tcp" => NetworkProtocol::TCP,
            _ => return Err(MetricalError::ConfigurationInvalid("Invalid value for network protocol environment variable"))
        },
        Err(_) => return Err(MetricalError::ConfigurationInvalid("No value for network protocol environment variable"))
    };

    let (ip_addr, port) = match var(
        format!("{}{}", prefix, NETWORK_DESTINATION_ENV)
    ) {
        Ok(value) => {
            match value.find(":") {
                Some(idx) => {
                    let ip_addr: IpAddr = match value[..idx].parse() {
                        Ok(v) => v,
                        Err(_) => return Err(
                            MetricalError::ConfigurationInvalid("Unable to parse IP Address")
                        )
                    };

                    let port: u16 = match value[idx + 1..].parse() {
                        Ok(port) => port,
                        Err(_) => return Err(
                            MetricalError::ConfigurationInvalid("Unable to parse port")
                        )
                    };

                    (ip_addr, port)
                },
                None => {
                    let ip_addr: IpAddr = match value.parse() {
                        Ok(v) => v,
                        Err(_) => return Err(
                            MetricalError::ConfigurationInvalid("Unable to parse IP Address")
                        )
                    };

                    let port = 2003;

                    (ip_addr, port)
                }
            }
        },
        Err(_) => return Err(
            MetricalError::ConfigurationInvalid("Network destination unspecified")
        )
    };

    let send_method: Protocol = match var(format!("{}{}", prefix, SEND_METHOD_ENV)) {
        Ok(val) => match val.to_lowercase().as_ref() {
            "statsd" => Protocol::StatsD,
            "graphite" => Protocol::Graphite(Compression::Uncompressed),
            "graphite_pickle" => Protocol::Graphite(Compression::Pickled),
            _ => return Err(
                MetricalError::ConfigurationInvalid("Unable to parse Send method.")
            )
        },
        Err(_) => return Err(
            MetricalError::ConfigurationInvalid("Send Method unspecified [STATSD|GRAPHITE|GRAPHITE_PICKLE]")
        )
    };

    let configuration = ConfigurationBuilder::new()
        .ip_addr(ip_addr)
        .port(port)
        .namespace(namespace)
        .protocol(send_method)
        .network_protocol(network_protocol)
        .flush_after_interval(flush_interval)
        .flush_after_amount(flush_amount)
        .build()?;

    from_config(configuration)
}

pub fn from_config(configuration: Configuration) -> Result<(), MetricalError> {
    let collector = Box::new(BufferedCollector::new(configuration));

    set_boxed_collector(collector);

    Ok(())
}