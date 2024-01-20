use std::{collections::BTreeMap, io::Error};

use crate::backend::get_string_from_file;

#[derive(Debug, Default)]
pub struct Proc {
    pub name: String,
    pub id: u64,
}

pub const PROC: &str = "/proc/";

#[derive(Debug, Default)]
pub struct ProcList {
    pub processes: Vec<Proc>,
}

impl ProcList {
    pub fn get() -> Result<Self, Error> {
        let mut ps = ProcList::default();

        if let Ok(dir) = std::fs::read_dir(PROC) {
            for entry in dir.flatten() {
                let mut path = entry.path().clone();

                if let Some(id) = path
                    .file_name()
                    .and_then(|p| p.to_str())
                    .and_then(|s| s.parse().ok())
                {
                    // let name = entry.path().to_string_lossy().to_string();
                    path.push("cmdline");
                    if let Ok(s) = get_string_from_file(path) {
                        let cmdline: Vec<&str> = s.splitn(2, '\0').collect();
                        if let Some(name) = cmdline.get(0) {
                            let proc = Proc {
                                id,
                                name: name.to_string(),
                            };

                            ps.processes.push(proc);
                        }
                    }
                }
            }
        }

        Ok(ps)
    }
}

#[test]
fn get_proc_list_test() {
    let ps = ProcList::get().unwrap();
    dbg!(ps);
}
