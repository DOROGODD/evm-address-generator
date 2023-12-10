use std::{
    fs::{File, OpenOptions},
    io::Write,
    time::Duration,
};

use dialoguer::{theme::ColorfulTheme, Input};
use ethers_signers::{LocalWallet, Signer};
use indicatif::{ProgressBar, ProgressStyle};
use threadpool::ThreadPool;
use utils::validate_string;

mod macros;
mod utils;

fn generate_addresses(
    pb: ProgressBar,
    file: &mut File,
    prefix: Option<&str>,
    postfix: Option<&str>,
) {
    loop {
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        let address = format!("{:x?}", wallet.address());
        let private_key = hex::encode(wallet.signer().to_bytes());

        if let Some(prefix) = prefix {
            if !address.trim_start_matches("0x").starts_with(prefix) {
                continue;
            }
        }

        if let Some(postfix) = postfix {
            if !address.trim_start_matches("0x").ends_with(postfix) {
                continue;
            }
        }

        pb.set_message(format!("Address: {address}"));

        file.write_all(format!("{address}:{private_key}\n").as_bytes())
            .unwrap();
    }
}

fn main() {
    let threads: i16 = input!("Threads:");

    let prefix = input!(
        Option(String),
        "Prefix (to skip press Enter):",
        validate_string
    );

    let postfix = input!(
        Option(String),
        "Postfix (to skip press Enter):",
        validate_string
    );

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );

    pb.set_message("Attaching founded.txt file...");

    let pool = ThreadPool::new(threads as usize);

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("founded.txt")
        .unwrap();

    pb.set_message("Generating addresses...");

    for _ in 0..threads {
        let pb = pb.clone();

        let prefix = prefix.clone();
        let postfix = postfix.clone();
        let mut file = file.try_clone().unwrap();

        pool.execute(move || {
            generate_addresses(pb, &mut file, prefix.as_deref(), postfix.as_deref())
        });
    }

    pool.join();
}
