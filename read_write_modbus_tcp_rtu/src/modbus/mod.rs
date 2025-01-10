pub mod tcp;
pub mod rtu;

// Common types or constants
pub type ModbusResult<T> = Result<T, Box<dyn std::error::Error>>;

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
        ModbusProtocol::RTU => {
            rtu::read_rtu_data(address, slave, register).await
        }
        ModbusProtocol::TCP => {
            let (ip, port) = parse_address(address)?;
            tcp::read_tcp_data(ip, port, slave, register).await
        }
    }
}

// Utility to parse TCP address
fn parse_address(address: &str) -> ModbusResult<(&str, u16)> {
    let parts: Vec<&str> = address.split(':').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid address format: {}", address).into());
    }
    let ip = parts[0];
    let port: u16 = parts[1].parse()?;
    Ok((ip, port))
}
