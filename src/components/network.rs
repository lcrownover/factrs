use crate::Collector;
use anyhow::{Context, Result};
use serde::Serialize;
use serde_json::to_value;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Serialize, Debug)]
pub struct InterfaceFields {
    /// The DHCP server for the network interface.
    pub dhcp: Option<Ipv4Addr>,
    /// The IPv4 address for the network interface.
    pub ip: Option<Ipv4Addr>,
    /// The IPv6 address for the network interface.
    pub ip6: Option<Ipv6Addr>,
    /// The MAC address for the network interface.
    pub mac: String,
    /// The Maximum Transmission Unit (MTU) for the network interface.
    pub mtu: u32,
    /// The IPv4 prefix for the network interface.
    pub prefix: Option<u32>,
    /// The IPv6 prefix for the network interface.
    pub prefix6: Option<u32>,
    /// The IPv4 network for the network interface.
    pub network: Option<Ipv4Addr>,
    /// The IPv6 network for the network interface.
    pub network6: Option<Ipv6Addr>,
    /// The IPv6 scope for the network interface.
    pub scope6: String,
}

#[derive(Serialize, Debug)]
pub struct Interface {
    /// The array of IPv4 address bindings for the interface.
    pub bindings: Vec<Ipv4Addr>,
    /// The array of IPv6 address bindings for the interface.
    pub bindings6: Vec<Ipv6Addr>,

    #[serde(flatten)]
    pub interface_fields: InterfaceFields,
}

#[derive(Debug, Serialize)]
pub struct NetworkFacts {
    /// The host name of the system
    pub hostname: String,
    /// The domain name of the system
    pub domain: Option<String>,
    /// The fully-qualified domain name of the system
    pub fqdn: String,
    /// The network interfaces of the system
    pub interfaces: HashMap<String, Interface>,
    /// The name of the primary interface.
    pub primary: String,

    #[serde(flatten)]
    pub interface_fields: InterfaceFields,
}

pub struct NetworkComponent;
impl NetworkComponent {
    pub fn new() -> Self {
        Self
    }
}
impl Collector for NetworkComponent {
    fn name(&self) -> &'static str {
        "network"
    }

    fn collect(&self) -> Result<serde_json::Value> {
        let hostname = "myhostname";
        let domain = "mydomain.org";
        let mut interfaces = HashMap::new();
        interfaces.insert(
            "eth0".to_string(),
            Interface {
                bindings: vec!["10.0.0.1".parse::<Ipv4Addr>()?],
                bindings6: vec!["fe80::c468:27d4:bd86:a4f6".parse::<Ipv6Addr>()?],
                interface_fields: InterfaceFields {
                    mac: "3e:0a:ff:fd:7f:9f".to_string(),
                    mtu: 1500,
                    scope6: "scope".to_string(),
                    dhcp: Some("128.223.32.36".parse::<Ipv4Addr>()?),
                    prefix: Some(24),
                    network: Some("10.0.0.0".parse::<Ipv4Addr>()?),
                    ip: Some("10.0.0.1".parse::<Ipv4Addr>()?),
                    ip6: Some("fe80::c468:27d4:bd86:a4f6".parse::<Ipv6Addr>()?),
                    prefix6: Some(64),
                    network6: Some("fe80::c468:27d4:bd86".parse::<Ipv6Addr>()?),
                },
            },
        );
        let facts = NetworkFacts {
            hostname: hostname.to_string(),
            domain: Some(domain.to_string()),
            fqdn: format!("{hostname}.{domain}"),
            interfaces: interfaces,
            primary: "eth0".to_string(),
            interface_fields: InterfaceFields {
                mac: "3e:0a:ff:fd:7f:9f".to_string(),
                mtu: 1500,
                scope6: "scope".to_string(),
                dhcp: Some("128.223.32.36".parse::<Ipv4Addr>()?),
                prefix: Some(24),
                network: Some("10.0.0.0".parse::<Ipv4Addr>()?),
                ip: Some("10.0.0.1".parse::<Ipv4Addr>()?),
                ip6: Some("fe80::c468:27d4:bd86:a4f6".parse::<Ipv6Addr>()?),
                prefix6: Some(64),
                network6: Some("fe80::c468:27d4:bd86".parse::<Ipv6Addr>()?),
            },
        };
        let j = to_value(facts).context("serializing to json value")?;
        Ok(j)
    }
}
