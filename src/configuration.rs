use crate::connection::Connection;
use crate::protocol::{Protocol, NetworkProtocol};
use crate::metric::{MetricData, Namespace};
use std::net::IpAddr;
use crate::MetricalError;

#[derive(Clone)]
pub struct FlushConfigurationOptions {
    pub flush_after_amount: Option<usize>,
    pub flush_after_interval: Option<u64>

}

pub struct ConfigurationOptions {
    pub namespace: Option<Namespace>,
    pub flush: FlushConfigurationOptions
}

pub struct Configuration {
    connection: Connection,
    pub protocol: Protocol,
    pub options: ConfigurationOptions,
}

pub struct ConfigurationBuilder {
    ip_addr: Option<IpAddr>,
    port: Option<u16>,
    network_protocol: Option<NetworkProtocol>,
    namespace: Option<Namespace>,
    protocol: Option<Protocol>,
    flush_after_amount: Option<usize>,
    flush_after_interval: Option<u64>
}

impl Default for ConfigurationBuilder {
    fn default() -> Self {
        ConfigurationBuilder {
            ip_addr: None,
            port: None,
            network_protocol: None,
            protocol: None,
            namespace: None,
            flush_after_amount: None,
            flush_after_interval: None
        }
    }
}

impl ConfigurationBuilder {
    pub fn new() -> Self {
        ConfigurationBuilder::default()
    }

    pub fn ip_addr(mut self, ip_addr: IpAddr) -> Self {
        self.ip_addr = Some(ip_addr);
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn flush_after_amount(mut self, amount: Option<usize>) -> Self {
        self.flush_after_amount = amount;
        self
    }

    pub fn flush_after_interval(mut self, interval: Option<u64>) -> Self {
        self.flush_after_interval = interval;
        self
    }

    pub fn protocol(mut self, protocol: Protocol) -> Self {
        self.protocol = Some(protocol);
        self
    }

    pub fn network_protocol(mut self, protocol: NetworkProtocol) -> Self {
        self.network_protocol = Some(protocol);
        self
    }

    pub fn namespace(mut self, namespace: Option<Namespace>) -> Self {
        self.namespace = namespace;
        self
    }

    pub fn build(self) -> Result<Configuration, MetricalError>  {
        let protocol = if self.protocol.is_none() {
            return Err(MetricalError::ConfigurationInvalid("Protocol Unspecified"));
        } else {
            self.protocol.unwrap()
        };

        let network_protocol = if self.network_protocol.is_none() {
            return Err(MetricalError::ConfigurationInvalid("Network Protocol unspecified"))
        } else { self.network_protocol.unwrap() };

        let ip_addr = if self.ip_addr.is_some() { self.ip_addr.unwrap() } else {
            return Err(MetricalError::ConfigurationInvalid("IP Address unspecified"))
        };

        let port = if self.port.is_some() { self.port.unwrap() } else {
            return Err(MetricalError::ConfigurationInvalid("Port unspecified"))
        };

        let configuration_options = ConfigurationOptions{
            namespace: self.namespace,
            flush: FlushConfigurationOptions {
                flush_after_interval: self.flush_after_interval,
                flush_after_amount: self.flush_after_amount
            }
        };

        let connection = Connection::new(ip_addr, port, network_protocol);
        Ok(Configuration::new(connection, protocol, configuration_options))
    }
}

impl Configuration {
    pub fn new(connection: Connection, protocol: Protocol, options: ConfigurationOptions)
        -> Configuration
    {
        Configuration{
            connection,
            protocol,
            options,
        }
    }
}

impl Configuration {
    pub fn send(&mut self,  data: Vec<MetricData>) {
        let data = self.protocol.serialize(data);

        for packet_body in data {
            match self.connection.send(packet_body.as_ref()) {
                Ok(_) => {},
                Err(_) => {}
            };
        }
    }
}

