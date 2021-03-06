use std::net::{Ipv4Addr, UdpSocket};

use log::*;

use dns::{Request, Response};
use super::{Transport, Error};


/// The **UDP transport**, which sends DNS wire data inside a UDP datagram.
///
/// # Examples
///
/// ```no_run
/// use dns_transport::{Transport, UdpTransport};
/// use dns::{Request, Flags, Query, Labels, QClass, qtype, record::NS};
///
/// let query = Query {
///     qname: Labels::encode("dns.lookup.dog").unwrap(),
///     qclass: QClass::IN,
///     qtype: qtype!(NS),
/// };
///
/// let request = Request {
///     transaction_id: 0xABCD,
///     flags: Flags::query(),
///     query: query,
///     additional: None,
/// };
///
/// let transport = UdpTransport::new("8.8.8.8");
/// transport.send(&request);
/// ```
pub struct UdpTransport {
    addr: String,
}

impl UdpTransport {

    /// Creates a new UDP transport that connects to the given host.
    pub fn new(sa: impl Into<String>) -> Self {
        let addr = sa.into();
        Self { addr }
    }
}


impl Transport for UdpTransport {
    fn send(&self, request: &Request) -> Result<Response, Error> {
        info!("Opening UDP socket");
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;

        if self.addr.contains(':') {
            socket.connect(&*self.addr)?;
        }
        else {
            socket.connect((&*self.addr, 53))?;
        }
        debug!("Opened");

        let bytes_to_send = request.to_bytes().expect("failed to serialise request");

        info!("Sending {} bytes of data to {} over UDP", bytes_to_send.len(), self.addr);
        let written_len = socket.send(&bytes_to_send)?;
        debug!("Wrote {} bytes", written_len);

        info!("Waiting to receive...");
        let mut buf = vec![0; 4096];
        let received_len = socket.recv(&mut buf)?;

        info!("Received {} bytes of data", received_len);
        let response = Response::from_bytes(&buf[.. received_len])?;
        Ok(response)
    }
}
