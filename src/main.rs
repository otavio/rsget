extern crate env_logger;
extern crate failure;
extern crate indicatif;
extern crate reqwest;
extern crate structopt;

use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{header, Client};
use std::{fs::File, io, io::copy, io::Read};
use structopt::StructOpt;

struct DownloadProgress<R> {
    inner: R,
    progress_bar: ProgressBar,
}

impl<R: Read> Read for DownloadProgress<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf).map(|n| {
            self.progress_bar.inc(n as u64);
            n
        })
    }
}

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

    let fname = cmdline
        .url
        .path_segments()
        .and_then(|segments| segments.last())
        .unwrap_or("tmp.bin");

    let client = Client::new();
    let total_size = client
        .head(cmdline.url.clone())
        .send()?
        .headers()
        .get(header::CONTENT_LENGTH)
        .and_then(|ct_len| ct_len.to_str().ok())
        .and_then(|ct_len| ct_len.parse().ok())
        .unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
                 .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                 .progress_chars("#>-"));

    let mut res = DownloadProgress {
        progress_bar: pb,
        inner: reqwest::get(cmdline.url.clone())?,
    };

    let _ = copy(&mut res, &mut File::create(fname)?)?;
    println!("Download of '{}' completed.", fname);

    Ok(())
}
