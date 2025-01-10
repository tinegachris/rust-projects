use modbus::{read_data, ModbusProtocol};

#[tokio::main]
async fn main() {
    // Example for RTU Communication
    match read_data(ModbusProtocol::RTU, "/dev/ttyUSB0", 1, 100).await {
        Ok(value) => println!("RTU: Register value is {}", value),
        Err(err) => eprintln!("RTU Error: {}", err),
    }

    // Example for TCP Communication
    match read_data(ModbusProtocol::TCP, "192.168.1.100:502", 1, 100).await {
        Ok(value) => println!("TCP: Register value is {}", value),
        Err(err) => eprintln!("TCP Error: {}", err),
    }
}
