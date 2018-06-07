extern crate openssl;
extern crate openssl_sys;
#[macro_use]
extern crate error_chain;

extern crate pnt_parser;

use error::Result;
use openssl::nid::Nid;
use openssl::ssl::{Ssl, SslContext, SslMethod, SslVerifyMode};
use pnt_parser::socket;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpStream, ToSocketAddrs};
use std::path::Path;

pub mod error {
    use openssl;
    use std::io;

    error_chain! {
        foreign_links {
            OpenSslErrorStack(openssl::error::ErrorStack);
            IoError(io::Error);
        }
        errors {
            HandshakeError(e: String) {
                description("HandshakeError")
                display("HandshakeError: {}", e)
            }
        }
    }
}

fn main() {
    fn from_addr<A: ToSocketAddrs>(addr: A) -> Result<String> {
        let context = {
            let mut context = SslContext::builder(SslMethod::tls())?;
            context.set_verify(SslVerifyMode::empty());
            context.build()
        };

        let connector = Ssl::new(&context)?;
        let stream = TcpStream::connect(addr)?;
        let stream = connector
            .connect(stream)
            .map_err(|e| error::ErrorKind::HandshakeError(e.description().to_owned()))?;
        let cert = stream
            .ssl()
            .peer_certificate()
            .ok_or("Certificate not found")?;
        let cn = cert.subject_name()
            .entries_by_nid(Nid::COMMONNAME)
            .next()
            .expect("No Common Name");

        Ok(String::from_utf8(cn.data().as_slice().to_vec()).unwrap())
    }

    let path = Path::new("/proc/net/tcp");
    let file = File::open(&path).expect("unable to open /proc/net/tcp");

    let reader = BufReader::new(file);
    let lines: Vec<_> = reader
        .lines()
        .skip(1)
        .flat_map(|l| {
            l.ok()
                .and_then(|s| socket(s.as_bytes()).ok().and_then(|(_, b)| Some(b)))
        })
        .collect();

    lines.iter().for_each(|x| {
        if x.remote_address.port() == 443 || x.remote_address.port() == 8443 {
            println!(
                "{:>width$}\t{:?}\t{:?}",
                x.remote_address,
                x.state,
                from_addr(x.remote_address),
                width = 25
            );
        }
    });
}
