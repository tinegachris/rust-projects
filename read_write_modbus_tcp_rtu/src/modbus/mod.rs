pub mod tcp;
pub mod rtu;

use std::error::Error;

// Common types or constants
pub type ModbusResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, Copy)]
pub enum ModbusProtocol {
    RTU,
    TCP,
}

// Unified interface for Modbus operations
pub async fn read_data(
    protocol: ModbusProtocol,
    address: &str,
    slave: u8,
    register: u16,
) -> ModbusResult<u16> {
    match protocol {
        ModbusProtocol::RTU => rtu::read_rtu_data(address, slave, register).await,
        ModbusProtocol::TCP => {
            let (ip, port) = parse_address(address)?;
            tcp::read_tcp_data(ip, port, slave, register).await
        }
    }
}

// Utility to parse TCP address
fn parse_address(address: &str) -> ModbusResult<(&str, u16)> {
    let mut parts = address.split(':');
    let ip = parts.next().ok_or_else(|| format!("Invalid address format: {}", address))?;
    let port_str = parts.next().ok_or_else(|| format!("Invalid address format: {}", address))?;
    let port: u16 = port_str.parse()?;
    Ok((ip, port))
}
