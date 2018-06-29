extern crate env_logger;
extern crate failure;
extern crate reqwest;

#[macro_use]
extern crate structopt;

use std::fs;
use std::io;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rsget")]
struct Cmdline {
    /// Output file
    #[structopt(short = "u", long = "url")]
    url: reqwest::Url,
}

fn main() -> Result<(), failure::Error> {
    let cmdline = Cmdline::from_args();

    env_logger::init();

    let mut res = reqwest::get(cmdline.url)?;
    let mut dest = {
        // extract target filename from URL
        let fname = res
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or("tmp.bin");

        println!("Writing to: '{}'", fname);
        fs::File::create(fname)?
    };

    let _ = io::copy(&mut res, &mut dest)?;

    Ok(())
}
