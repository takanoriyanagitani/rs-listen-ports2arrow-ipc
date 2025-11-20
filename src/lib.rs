use core::net::SocketAddr;
use std::collections::HashSet;
use std::error::Error;

use arrow::datatypes::{DataType, Field, Schema, SchemaRef};
use std::sync::Arc;

use listeners::Listener;
use listeners::Process;
use listeners::Protocol;

pub fn get_listeners() -> Result<HashSet<Listener>, Box<dyn Error>> {
    listeners::get_all()
}

pub struct ListenerInfo(pub Listener);

impl ListenerInfo {
    pub fn process(&self) -> &Process {
        &self.0.process
    }

    pub fn socket(&self) -> &SocketAddr {
        &self.0.socket
    }

    pub fn protocol(&self) -> &Protocol {
        &self.0.protocol
    }
}

pub struct ProcInfo(pub Process);

impl ProcInfo {
    pub fn pid(&self) -> u32 {
        self.0.pid
    }

    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn path(&self) -> &str {
        &self.0.path
    }
}

pub struct ProtoInfo(pub Protocol);

impl ProtoInfo {
    pub fn as_str(&self) -> &str {
        match self.0 {
            Protocol::TCP => "tcp",
            Protocol::UDP => "udp",
        }
    }
}

pub fn listener_schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        Field::new("pid", DataType::UInt32, false),
        Field::new("process_name", DataType::Utf8, false),
        Field::new("process_path", DataType::Utf8, false),
        Field::new("local_ip", DataType::Utf8, false),
        Field::new("local_port", DataType::UInt16, false),
        Field::new("protocol", DataType::Utf8, false),
        Field::new("ip_version", DataType::Utf8, false),
    ]))
}
