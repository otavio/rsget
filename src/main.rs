// Copyright (C) 2018, 2019 O.S. Systems Sofware LTDA
//
// SPDX-License-Identifier: Apache-2.0

use std::{
    fs,
    io::{self, copy, Read},
    path::Path,
};

use exitfailure::ExitFailure;
use failure;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{header, Client};
use structopt::StructOpt;

struct DownloadProgress<R> {
    inner: R,
    progress_bar: ProgressBar,
}

impl<R> Read for DownloadProgress<R>
where
    R: Read,
{
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
    /// URL to download
    #[structopt(short = "u", long = "url")]
    url: reqwest::Url,
}

fn main() -> Result<(), ExitFailure> {
    let cmdline = Cmdline::from_args();
    let client = Client::new();

    let total_size = {
        let resp = client.head(cmdline.url.as_str()).send()?;
        if resp.status().is_success() {
            resp.headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|ct_len| ct_len.to_str().ok())
                .and_then(|ct_len| ct_len.parse().ok())
                .unwrap_or(0)
        } else {
            return Err(failure::err_msg(format!(
                "Couldn't download URL: {}. Error: {:?}",
                cmdline.url,
                resp.status(),
            ))
            .into());
        }
    };

    let mut request = client.get(cmdline.url.as_str());
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
                 .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                 .progress_chars("#>-"));

    let file = Path::new(
        cmdline
            .url
            .path_segments()
            .and_then(std::iter::Iterator::last)
            .unwrap_or("tmp.bin"),
    );

    if file.exists() {
        let size = file.metadata()?.len().checked_sub(1).unwrap_or(0);
        request = request.header(header::RANGE, format!("bytes={}-", size));
        pb.inc(size);
    }

    let mut source = DownloadProgress {
        progress_bar: pb,
        inner: request.send()?,
    };

    let mut dest = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file)?;

    let _ = copy(&mut source, &mut dest)?;

    println!(
        "Download of '{}' has been completed.",
        file.to_str().unwrap()
    );

    Ok(())
}
