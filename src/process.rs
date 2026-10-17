/// process managment


use std::process::{
    Command,
    Child,
    ExitStatus,
    Output,
    Stdio
};

use std::io::Result;
use std::thread;

///
/// ProcInfo is struct that holds the informations of the process that is already ran and finished
///
#[derive(Debug, PartialEq)]
pub struct ProcInfo {
    pub cmd : String,
    pub status : ExitStatus,
    pub err : Option<String>,
    pub out : Option<Output>,
    pub never_run: bool,
}

impl ProcInfo {
    fn new(cmd: String, status: ExitStatus, err: Option<String>, out: Option<Output>, never_run: bool) -> Self {
        Self {
            cmd,
            status,
            err,
            out,
            never_run,
        }
    }
}

/// CommandMultiChainedList is type that represent list of chained deps list of commands
/// {
///     {
///         "gcc foo.c -c -o foo.o",
///         "gcc main.c -o main foo.o"
///     },
///     {
///         "gcc bar.c -c -o bar.o",
///         "gcc another_main.c -o another_main bar.o"
///     },
/// }
///
///
pub type CommandChainedList = Vec<String>;
pub type CommandMultiChainedList = Vec<CommandChainedList>;
pub type ProcInfoChainedList = Vec<Vec<ProcInfo>>;

pub fn run_cmd(cmd: String) -> Result<Command> {
    let splitted: Vec<&str> = cmd.split(' ').collect();
    let mut command = Command::new(splitted[0]);
    for i in 1..splitted.len() {
        command.arg(splitted[i]);
    }
    Ok(command)
}

pub fn run_cmd_async(cmd: String) -> Result<Child>{
    let splitted: Vec<&str> = cmd.split(' ').collect();
    let mut command = Command::new(splitted[0]);
    for i in 1..splitted.len() {
        command.arg(splitted[i]);
    }
    command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
}

///run_chained must be a single proc because it's chained and each command has a dep
pub fn run_chained(chained_commands: CommandChainedList) -> Vec<ProcInfo> {
    let mut list_of_chained_commands: Vec<ProcInfo> = Vec::new();

    for cmd_str in &chained_commands {
        let cmd_str_owned = cmd_str.clone();
        match run_cmd(cmd_str_owned.clone()) {
            Ok(mut command) => {
                match command.output() {
                    Ok(output) => {
                        let status = output.status;
                        let err = if output.stderr.is_empty() {
                            None
                        } else {
                            Some(String::from_utf8_lossy(&output.stderr).to_string())
                        };
                        let out = Some(output);
                        let info = ProcInfo::new(cmd_str_owned, status, err, out, false);
                        list_of_chained_commands.push(info);
                    }
                    Err(e) => {
                        let info = ProcInfo::new(
                            cmd_str_owned,
                            ExitStatus::default(),
                            Some(e.to_string()),
                            None,
                            true,
                        );
                        list_of_chained_commands.push(info);
                    }
                }
            }
            Err(e) => {
                let info = ProcInfo::new(
                    cmd_str_owned,
                    ExitStatus::default(),
                    Some(e.to_string()),
                    None,
                    true,
                );
                list_of_chained_commands.push(info);
            }
        }
    }
    list_of_chained_commands
}

/// run_multi_chained runs every inner chain (deps are sequential inside a chain).
/// `separate_proc: false` -> if fasle chains run one after another in this proc otherwise every chain gets its own proc (thread) so all chains run concurrently
pub fn run_multi_chained(
    multi_chained: CommandMultiChainedList,
    separate_proc: bool,
) -> ProcInfoChainedList {
    if !separate_proc {
        let mut ret: ProcInfoChainedList = Vec::new();
        for chain in &multi_chained {
            let info_chain = run_chained(chain.clone());
            ret.push(info_chain);
        }
        return ret;
    }

    let mut handles = Vec::with_capacity(multi_chained.len());
    for chain in multi_chained {
        handles.push(thread::spawn(move || run_chained(chain)));
    }

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap_or_default())
        .collect()
}
