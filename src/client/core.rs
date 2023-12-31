use crate::utils::config::Config;
use std::io::prelude::*;
use std::net::TcpStream;

pub trait TcpClient<'a> {
    fn with_config(config: &'a Config) -> Self;
    fn connect(&mut self) -> std::io::Result<()>;
    fn disconnect(&mut self) -> std::io::Result<()>;
    fn set_item(&mut self, key: String) -> std::io::Result<()>;
    fn get_item(&mut self, key: String) -> std::io::Result<()>;
    fn remove_item(&self, key: String) -> bool;
}
pub struct PoncuTcpClient<'a> {
    stream: Option<TcpStream>,
    config: &'a Config,
}

impl<'a> TcpClient<'a> for PoncuTcpClient<'a> {
    fn with_config(config: &'a Config) -> Self {
        PoncuTcpClient {
            stream: None,
            config,
        }
    }

    fn connect(&mut self) -> std::io::Result<()> {
        assert!(self.config.remote.is_some());
        let config_remote = self.config.remote.as_ref().unwrap();
        assert!(!config_remote.nodes.is_empty());
        let remote_address = config_remote.nodes[0];

        let stream = TcpStream::connect(remote_address)?;

        let local_addr = stream.local_addr().unwrap();
        log::info!("connected to {} as {}", remote_address, local_addr);
        
        stream.set_nodelay(true).expect("set_nodelay call failed");

        self.stream = Some(stream);
        Ok(())
    }

    fn disconnect(&mut self) -> std::io::Result<()> {
        let stream = self.stream.as_mut().unwrap();
        stream.flush()?;
        self.stream = None;
        Ok(())
    }

    fn set_item(&mut self, key: String) -> std::io::Result<()> {
        let stream = self.stream.as_mut().unwrap();
        stream.write_all(key.as_bytes())?;
        // stream.shutdown(std::net::Shutdown::Write).unwrap();
        stream.flush().unwrap();
        Ok(())
    }

    fn get_item(&mut self, _key: String) -> std::io::Result<()> {
        let stream = self.stream.as_mut().unwrap();
        let mut buf = [0; 128];
        stream.write_all(&buf)?;
        stream.read_exact(&mut buf)?;
        Ok(())
    }

    fn remove_item(&self, _key: String) -> bool {
        true
    }
}
