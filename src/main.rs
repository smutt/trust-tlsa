/*
Copyright (c) 2020, Andrew McConachie <andrew@depht.com>
All rights reserved.
 */

//////////////
// INCLUDES //
//////////////
use structopt::StructOpt;
use trust_dns_resolver::Resolver;

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
    let cli_opts: Opt = Opt::from_args();

    let dn: String = "".to_owned() + &cli_opts.domain.to_lowercase() + ".";
    println!("{} {}", "querying", dn);

    let resolver = Resolver::from_system_conf().unwrap();
    let response = resolver.mx_lookup(&dn);
    match response {
        Ok(rrs) => {
            for rdata in rrs.iter() {
                println!("{:?}", rdata);
            }
        }
        _ => println!("{}", "No records found"),
    }
}


