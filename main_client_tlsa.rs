/*
Copyright (c) 2020, Andrew McConachie <andrew@depht.com>
All rights reserved.
 */

//////////////
// INCLUDES //
//////////////
#[macro_use] extern crate log;
use structopt::StructOpt;
//use std::{iter, time, thread};
//use std::time::{Duration, SystemTime};
use std::str::FromStr;
//use std::net::Ipv4Addr;
//use trust_dns_proto::DnsStreamHandle;
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::udp::UdpClientConnection;
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType};

/////////////
// STRUCTS //
/////////////
#[derive(Debug, StructOpt)]
struct Opt {
    /// domain for lookup
    #[structopt(short = "d", long = "domain", default_value = "www.middlebox-dane.org")]
    domain: String,
}

///////////////
// CONSTANTS //
///////////////


/////////////////////////////
// BEGIN PROGRAM EXECUTION //
/////////////////////////////
fn main() {
    env_logger::builder().default_format_timestamp(false).init();
    debug!("Start");

    let cli_opts: Opt = Opt::from_args();

    ctrlc::set_handler(move || {
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    let dn: String = "_443._tcp.".to_owned() + &cli_opts.domain.to_lowercase() + ".";
    println!("{} {}", "querying", dn);

    let address = "8.8.8.8:53".parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);

    let qstr = Name::from_str(&dn).unwrap();
    let response: DnsResponse = client.query(&qstr, DNSClass::IN, RecordType::TLSA).unwrap();
    let answers: &[Record] = response.answers();

    if answers.len() > 0 {
        for ans in answers.iter() {
            if let RData::TLSA(rr) = ans.rdata() {
                println!("{} {:?}", "\ncert_usage:", rr.cert_usage());
                println!("{} {:?}", "selector:", rr.selector());
                println!("{} {:?}", "matching:", rr.matching());
                println!("{} {:?}", "hash:", rr.cert_data());
            } else {
                assert!(false, "unexpected result")
            }
        }
    }else{
        println!("{}", "No records found");
    }

    debug!("Finish");
}


