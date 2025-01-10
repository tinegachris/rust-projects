use tokio_modbus::client::{rtu, tcp};
use tokio_modbus::prelude::*;
use tokio_serial::{Serial, SerialPortSettings};
use anyhow::Result;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Example for Modbus TCP
    if let Err(e) = handle_modbus_tcp("127.0.0.1:502").await {
        eprintln!("Error handling Modbus TCP: {}", e);
    }

    // Example for Modbus RTU
    if let Err(e) = handle_modbus_rtu("/dev/ttyUSB0", 0x01).await {
        eprintln!("Error handling Modbus RTU: {}", e);
    }

    Ok(())
}

// Function to handle Modbus TCP connection
async fn handle_modbus_tcp(address: &str) -> Result<()> {
    // Parse the address
    let socket_addr = address.parse()?;
    // Connect to the Modbus TCP server
    let mut ctx = tcp::connect(socket_addr).await?;

    // Perform Modbus operations
    perform_modbus_operations(&mut ctx).await
}

// Function to handle Modbus RTU connection
async fn handle_modbus_rtu(tty_path: &str, slave: u8) -> Result<()> {
    // Serial port settings
    let settings = SerialPortSettings {
        baud_rate: 9600,
        data_bits: tokio_serial::DataBits::Eight,
        flow_control: tokio_serial::FlowControl::None,
        parity: tokio_serial::Parity::None,
        stop_bits: tokio_serial::StopBits::One,
        timeout: Duration::from_secs(1),
    };
    // Open the serial port
    let port = Serial::from_path(tty_path, &settings)?;
    // Connect to the Modbus RTU slave
    let mut ctx = rtu::connect_slave(port, slave).await?;

    // Perform Modbus operations
    perform_modbus_operations(&mut ctx).await
}

// Function to perform Modbus operations
async fn perform_modbus_operations(ctx: &mut dyn Client) -> Result<()> {
    // Read holding registers starting at address 0x00, read 7 registers
    let response = ctx.read_holding_registers(0x00, 7).await?;
    println!("Response: {:?}", response);

    // Write a single register at address 0x00 with value 0x1234
    ctx.write_single_register(0x00, 0x1234).await?;

    Ok(())
}
