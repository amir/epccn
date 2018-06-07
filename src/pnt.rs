#[macro_use]
extern crate nom;

use nom::{anychar, is_hex_digit};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::{str, u16, u32, u8};

#[derive(Debug, PartialEq)]
pub enum State {
    Established,
    SynSent,
    SynRecv,
    FinWait1,
    FinWait2,
    TimeWait,
    Close,
    CloseWait,
    LastAck,
    Listen,
    Closing,
    NewSynRecv,

    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct Socket {
    pub local_address: SocketAddr,
    pub remote_address: SocketAddr,
    pub state: State,
}

named!(hex_vec<&[u8], &[u8]>,
    take_while_m_n!(2, 2, is_hex_digit)
);

named!(
    ipv4<Ipv4Addr>,
    map!(count_fixed!(&[u8], hex_vec, 4), |mut a| {
        a.reverse();
        let hex = a.iter()
            .fold("".to_string(), |i, j| i + str::from_utf8(j).unwrap());
        Ipv4Addr::from(u32::from_str_radix(&hex, 16).unwrap())
    })
);

named!(
    port<u16>,
    map!(count_fixed!(&[u8], hex_vec, 2), |a| {
        let hex = a.iter()
            .fold("".to_string(), |i, j| i + str::from_utf8(j).unwrap());
        u16::from_str_radix(&hex, 16).unwrap()
    })
);

named!(
    state<State>,
    map!(count_fixed!(&[u8], hex_vec, 1), |a| {
        let hex = a.iter()
            .fold("".to_string(), |i, j| i + str::from_utf8(j).unwrap());
        let ns = u8::from_str_radix(&hex, 16).unwrap();
        match ns {
            1 => State::Established,
            2 => State::SynSent,
            3 => State::SynRecv,
            4 => State::FinWait1,
            5 => State::FinWait2,
            6 => State::TimeWait,
            7 => State::Close,
            8 => State::CloseWait,
            9 => State::LastAck,
            10 => State::Listen,
            11 => State::Closing,
            12 => State::NewSynRecv,
            _ => State::Unknown,
        }
    })
);

named!(pub socket<&[u8], Socket>,
    do_parse!(
       take!(6)  >>
       la: ipv4  >>
       tag!(":") >> 
       lp: port  >>
       tag!(" ") >>
       ra: ipv4  >>
       tag!(":") >> 
       rp: port  >>
       tag!(" ") >>
       s: state  >>
       opt!(complete!(many0!(anychar))) >>
       ({
           let ls = SocketAddr::new(IpAddr::V4(la), lp);
           let rs = SocketAddr::new(IpAddr::V4(ra), rp);
           Socket {
               local_address: ls,
               remote_address: rs,
               state: s
           }
       })
    )
);

#[test]
fn parse_ip() {
    let input = b"   0: 017AA8C0:0035 00000000:0000 0A";
    let res = Socket {
        local_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 122, 1)), 53),
        remote_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0),
        state: State::Listen,
    };
    assert_eq!(socket(&input[..]), Ok((&b""[..], res)));
}
