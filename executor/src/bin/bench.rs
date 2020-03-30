// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long, default_value = "1000")]
    num_accounts: usize,

    #[structopt(long, default_value = "100000000000")]
    init_account_balance: u64,

    #[structopt(long, default_value = "1000")]
    block_size: usize,

    #[structopt(long, default_value = "10")]
    num_transfer_blocks: usize,

    #[structopt(long, default_value = "0")]
    tx_type: usize,

    #[structopt(long, parse(from_os_str))]
    db_dir: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    let _logger =
        libra_logger::set_global_logger(true /* async_drain */, None /* chan_size */);

    rayon::ThreadPoolBuilder::new()
        .thread_name(|index| format!("rayon-global-{}", index))
        .build_global()
        .expect("Failed to build rayon global thread pool.");

    executor::benchmark::run_benchmark(
        opt.num_accounts,
        opt.init_account_balance,
        opt.block_size,
        opt.num_transfer_blocks,
        opt.tx_type,
        opt.db_dir,
    );
}
