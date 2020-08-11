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

    println!("{}",cli_opts.domain.to_uppercase());

    debug!("Finish");
}


