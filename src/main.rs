extern crate env_logger;
extern crate failure;
extern crate reqwest;

#[macro_use]
extern crate structopt;

use std::fs::File;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rsget")]
struct Opt {
    /// Output file
    #[structopt(short = "u", long = "url")]
    url: reqwest::Url,
}

fn main() -> Result<(), failure::Error> {
    let opt = Opt::from_args();

    env_logger::init();

    let mut res = reqwest::get(opt.url)?;
    let mut dest = {
        // extract target filename from URL
        let fname = res
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("Writing to: '{}'", fname);
        File::create(fname)?
    };

    let _ = std::io::copy(&mut res, &mut dest)?;

    Ok(())
}
