// Copyright (c) 2024 Marc Pabst
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use crate::errors::PsychophysicsError;
use serialport;

pub trait SerialPortTrait {
    fn write(&mut self, data: &str)
        -> Result<(), PsychophysicsError>;
    fn write_bytes(
        &mut self,
        data: &[u8],
    ) -> Result<(), PsychophysicsError>;
}

/// A serial port. Wraprs a backend serial port.
pub enum SerialPort {
    RealSerialPort(Box<dyn serialport::SerialPort>),
    DummySerialPort,
}

impl SerialPort {
    /// Creates a new serial port.
    pub fn open<'a>(
        path: impl Into<String>,
        baudrate: u32,
        timeout: u32,
    ) -> Result<Self, PsychophysicsError> {
        let path = path.into();

        let backend = serialport::new(&path, baudrate)
            .timeout(std::time::Duration::from_millis(timeout as u64))
            .open()?;

        Ok(SerialPort::RealSerialPort(backend))
    }

    /// Creates a new serial port or a dummy serial port if the serial port could not be opened.
    /// This is useful for debugging on a machine that does not have a serial port.
    ///
    /// However, you need to be very careful when using this function because it will not
    /// throw an error if the serial port could not be opened!
    pub fn open_or_dummy<'a>(
        path: impl Into<String>,
        baudrate: u32,
        timeout: u32,
    ) -> Self {
        let path = path.into();

        let backend = serialport::new(&path, baudrate)
            .timeout(std::time::Duration::from_millis(timeout as u64))
            .open();

        match backend {
            Ok(backend) => {
                log::warn!("Serial port {} opened but this might silently fail on a machine without a serial port - make sure to use open() instead of open_or_dummy() in production!", path);
                SerialPort::RealSerialPort(backend)
            }
            Err(_) => {
                log::warn!("Serial port {} could not be opened - using dummy serial port instead", path);
                SerialPort::DummySerialPort
            }
        }
    }

    /// Creates a dummy serial port.
    pub fn dummy() -> Self {
        SerialPort::DummySerialPort
    }

    pub fn unwrap(self) -> Box<dyn serialport::SerialPort> {
        match self {
            SerialPort::RealSerialPort(backend) => backend,
            SerialPort::DummySerialPort => {
                panic!("unwrap() called on dummy serial port")
            }
        }
    }
}


impl SerialPortTrait for SerialPort {
    /// Writes a string to the serial port.
    fn write(
        &mut self,
        data: &str,
    ) -> Result<(), PsychophysicsError> {
        match self {
            SerialPort::RealSerialPort(backend) => {
                backend.write(data.as_bytes())?;
                Ok(())
            }
            SerialPort::DummySerialPort => Ok(()),
        }
    }
    /// Writes a slice of bytes to the serial port.
    fn write_bytes(
        &mut self,
        data: &[u8],
    ) -> Result<(), PsychophysicsError> {
        match self {
            SerialPort::RealSerialPort(backend) => {
                backend.write(data)?;
                Ok(())
            }
            SerialPort::DummySerialPort => Ok(()),
        }
    }

}
