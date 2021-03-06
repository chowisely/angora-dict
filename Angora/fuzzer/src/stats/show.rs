use super::ChartStats;
use crate::{branches::GlobalBranches, depot::Depot, executor};
use angora_common::defs;
use std::{
    fs,
    io::Write,
    sync::{Arc, RwLock},
};

pub fn show_stats(
    log_f: &mut fs::File,
    depot: &Arc<Depot>,
    gb: &Arc<GlobalBranches>,
    stats: &Arc<RwLock<ChartStats>>,
    dlog_f: &mut fs::File,
    executor: &mut executor::Executor,
) -> usize {
    let covered_branches = &gb.get_covered_branches();
    stats
        .write()
        .expect("Could not write stats.")
        .sync_from_global(depot, gb);

    let dir = depot
        .dirs
        .inputs_dir
        .parent()
        .expect("Could not get parent directory.");
    let mut log_s = fs::File::create(dir.join(defs::CHART_STAT_FILE))
        .expect("Could not create chart stat file.");
    {
        let s = stats.read().expect("Could not read from stats.");
        println!("{}", *s);
        writeln!(log_f, "{}, {}", s.mini_log(), covered_branches).expect("Could not write minilog.");
        write!(
            log_s,
            "{}",
            serde_json::to_string(&*s).expect("Could not serialize!")
        )
        .expect("Unable to write!");
    }
    {
        let d = match executor.dictionary.read() {
            Ok(guard) => guard,
            Err(poisoned) => {
                warn!("Lock poisoned. Results can be incorrect! Continuing...");
                poisoned.into_inner()
            }
        };
        writeln!(dlog_f, "{}", d.get_len()).expect("Could not write minilog.");
    }
    *covered_branches
}
