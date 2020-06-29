use std::net::{IpAddr, UdpSocket};
use std::net::{TcpStream, SocketAddr};
use crate::protocol::NetworkProtocol;


pub struct Connection {
    addr: SocketAddr,
    protocol: NetworkProtocol,
    stream: Option<TcpStream>
}

impl Connection {
    pub fn new(addr: IpAddr, port: u16, protocol: NetworkProtocol) -> Self {
        let addr = SocketAddr::from((addr, port));

        let stream = match protocol {
            NetworkProtocol::TCP => Option::Some(
                TcpStream::connect(addr).expect("Unable to connect to TCP Address")
            ),
            NetworkProtocol::UDP => Option::None
        };

        Connection{
            addr,
            protocol,
            stream
        }
    }

    pub fn send(&mut self, buffer: &[u8]) -> Result<(), std::io::Error> {
        use std::io::prelude::*;
        match self.protocol {
            NetworkProtocol::TCP => {
                let mut stream = self.stream.take().unwrap();
                stream.write(buffer)?;

                self.stream = Option::Some(stream);

                Ok(())
            },
            NetworkProtocol::UDP => {
                let addrs = [
                    SocketAddr::from(([127, 0, 0, 1], self.addr.port())),
                    SocketAddr::from(([127, 0, 0, 1], self.addr.port() + 1)),
                    SocketAddr::from(([127, 0, 0, 1], self.addr.port() + 2)),
                ];

                let socket = UdpSocket::bind(&addrs[..]).expect("Unable to create UDP Socket");
                socket.connect(self.addr).expect("Unable to connect to UDP Socket on server");
                socket.send(buffer)?;

                Ok(())
            }
        }
    }
}


#[cfg(test)]
mod tests {
    mod udp {
        use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv4Addr, };
        use crate::connection::Connection;
        use crate::protocol::NetworkProtocol;

        #[test]
        pub fn it_should_send_data() -> Result<(), std::io::Error> {
            let data = b"Hello World!";

            let mut connection = Connection::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                3435,
                NetworkProtocol::UDP
            );

            let udp_listener = UdpSocket::bind("127.0.0.1:3435")?;

            connection.send(&data[..]);
            let mut buf: [u8; 256] = [0; 256];
            if let Ok(recieved) = udp_listener.recv(&mut buf) {
                let buf = &mut buf[..recieved];
                assert_eq!(buf, data);
                println!("{:?}", buf);
            } else {
                assert!(false, "Unable to receive data")
            }

            Ok(())
        }
    }
    mod tcp {
        use std::net::{IpAddr, Ipv4Addr, TcpStream, TcpListener};
        use super::super::Connection;
        use crate::protocol::NetworkProtocol;
        use std::io::Read;

        #[test]
        pub fn it_should_send_and_receive_data() -> std::io::Result<()> {
            let data = b"Hello World!";

            let tcp_listener = TcpListener::bind("127.0.0.1:3435")?;
            let mut connection = Connection::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                3435,
                NetworkProtocol::TCP
            );


            connection.send(&data[..])?;
            let mut buf: [u8; 256] = [0; 256];

            for stream in tcp_listener.incoming() {
                let mut stream = stream?;
                if let Ok(recieved) = stream.read(&mut buf) {
                    let buf = &mut buf[..recieved];
                    assert_eq!(buf, data);
                } else {
                    assert!(false, "Unable to receive data")
                }

                break;
            }

            drop(connection);

            Ok(())
        }
    }
}