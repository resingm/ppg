use std::io;
use clap::{arg, value_parser, Command};


fn start(ifname: &str) -> io::Result<()> {
    let nic = tun_tap::Iface::new(ifname, tun_tap::Mode::Tun)?;
    
    // A packet will have the following structure:
    //      flags: 2 bytes
    //      proto: 2 bytes
    //      payload: 0 .. MTU
    let mut buf = [0u8; 1504];

    loop {

        let nbytes = nic.recv(&mut buf[..])?;

        if nbytes < 4 {
            // Received an invalid packet.
            eprintln!("Received invalid ethernet packet: {:x?}", &buf[..nbytes]);
            continue;
        }

        // Network endianess is big-endian
        // from_be_bytes = from_big_endian
        // let flags: u16 = u16::from_be_bytes(&buf[..2].);
        let _eth_flags: u16 = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto: u16 = u16::from_be_bytes([buf[2], buf[3]]);

        // Overview of proto versions:
        // https://en.wikipedia.org/wiki/EtherType#Values
        //   0x0800 | IPv4
        //   0x86dd | IPv6

        if eth_proto != 0x0800 {
            // eprintln!("received proto: {:x}", proto);
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(p) => {
                let src = p.source_addr();
                let dst = p.destination_addr();
                let proto = p.protocol();
                let len = p.payload_len();
                let ttl = p.ttl();

                eprintln!("{:?} -> {:?} ; Bytes {} ; Proto {} ; TTL {}", src, dst, len, proto, ttl);
            },
            Err(e) => {
                eprintln!("ignoring non-parseable packet {:?}", e);
            }
        };

        // eprintln!("received ipv4 : {} bytes", nbytes - 4);
        // eprintln!("       flags  : {:x}", flags);
        // eprintln!("       proto  : {:x}", proto);
        // eprintln!("     payload  : {:x?}", &buf[4..nbytes]);
        // eprintln!("read {} bytes (): {:x?}", nbytes - 4, &buf[4..nbytes]);
    }

    // TODO: To send something, use nic.send();

    // Ok(())
}

fn main() -> io::Result<()> {
    let matches = Command::new("ppg")
        .version("0.1.0")
        .about("Packet playground to work with raw sockets on a Unix system")
        .arg(
            arg!(-i --interface <IFNAME> "Name of the interface that will be established")
                .required(true)
                .value_parser(value_parser!(String)),
        )
        .get_matches();

    let ifname = matches.get_one::<String>("interface")
        .expect("Requires an interface name, e.g. tun0");

    start(&ifname)
}