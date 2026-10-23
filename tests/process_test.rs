use pyrust::process::{
    CommandChainedList, CommandMultiChainedList, ProcInfoChainedList, run_chained, run_cmd,
    run_cmd_async, run_multi_chained,
};

use std::time::{Duration, Instant};

// run_cmd

#[cfg(target_os = "linux")]
#[test]
fn test_run_cmd_linux() {
    let mut cmd = run_cmd("echo hello".to_string()).expect("run_cmd should build the command");
    let out = cmd.output().expect("echo should run");
    assert!(out.status.success());
    assert_eq!(out.stdout, b"hello\n");
    assert!(out.stderr.is_empty());
}

#[cfg(target_os = "windows")]
#[test]
fn test_run_cmd_windows() {
    let mut cmd =
        run_cmd("cmd /C echo hello".to_string()).expect("run_cmd should build the command");
    let out = cmd.output().expect("echo should run");
    assert!(out.status.success());
    assert_eq!(out.stdout, b"hello\r\n");
    assert!(out.stderr.is_empty());
}

#[cfg(target_os = "linux")]
#[test]
fn test_run_cmd_with_args() {
    let mut cmd = run_cmd("echo foo bar".to_string()).expect("run_cmd should build the command");
    let out = cmd.output().expect("echo should run");
    assert!(out.status.success());
    assert_eq!(out.stdout, b"foo bar\n");
}

#[cfg(target_os = "windows")]
#[test]
fn test_run_cmd_with_args() {
    let mut cmd =
        run_cmd("cmd /C echo foo bar".to_string()).expect("run_cmd should build the command");
    let out = cmd.output().expect("echo should run");
    assert!(out.status.success());
    assert_eq!(out.stdout, b"foo bar\r\n");
}

// run_cmd_async

#[cfg(target_os = "linux")]
#[test]
fn test_run_cmd_async() {
    let child = run_cmd_async("echo async".to_string()).expect("spawn should succeed");
    let out = child.wait_with_output().expect("wait should succeed");
    assert!(out.status.success());
    assert_eq!(out.stdout, b"async\n");
}

#[cfg(target_os = "windows")]
#[test]
fn test_run_cmd_async() {
    let child = run_cmd_async("cmd /C echo async".to_string()).expect("spawn should succeed");
    let out = child.wait_with_output().expect("wait should succeed");
    assert!(out.status.success());
    assert_eq!(out.stdout, b"async\r\n");
}

#[test]
fn test_run_cmd_async_missing_binary() {
    let res = run_cmd_async("definitely_not_a_binary_xyz".to_string());
    assert!(res.is_err());
}

// run_chained

#[cfg(target_os = "linux")]
#[test]
fn test_run_chained() {
    let chain: CommandChainedList = vec!["echo one".to_string(), "echo two".to_string()];
    let infos = run_chained(chain);

    assert_eq!(infos.len(), 2);
    for info in &infos {
        assert!(info.status.success());
        assert!(!info.never_run);
        assert!(info.err.is_none());
        assert!(info.out.is_some());
    }
    assert_eq!(infos[0].cmd, "echo one");
    assert_eq!(infos[0].out.as_ref().unwrap().stdout, b"one\n");
    assert_eq!(infos[1].cmd, "echo two");
    assert_eq!(infos[1].out.as_ref().unwrap().stdout, b"two\n");
}

#[cfg(target_os = "windows")]
#[test]
fn test_run_chained() {
    let chain: CommandChainedList =
        vec!["cmd /C echo one".to_string(), "cmd /C echo two".to_string()];
    let infos = run_chained(chain);

    assert_eq!(infos.len(), 2);
    for info in &infos {
        assert!(info.status.success());
        assert!(!info.never_run);
        assert!(info.err.is_none());
        assert!(info.out.is_some());
    }
    assert_eq!(infos[0].cmd, "cmd /C echo one");
    assert_eq!(infos[0].out.as_ref().unwrap().stdout, b"one\r\n");
    assert_eq!(infos[1].cmd, "cmd /C echo two");
    assert_eq!(infos[1].out.as_ref().unwrap().stdout, b"two\r\n");
}

#[cfg(target_os = "linux")]
#[test]
fn test_run_chained_records_failure() {
    let chain: CommandChainedList = vec!["echo start".to_string(), "false".to_string()];
    let infos = run_chained(chain);

    assert_eq!(infos.len(), 2);
    assert!(infos[0].status.success());
    assert!(!infos[1].status.success());
    assert!(!infos[1].never_run);
}

#[cfg(target_os = "windows")]
#[test]
fn test_run_chained_records_failure() {
    let chain: CommandChainedList =
        vec!["cmd /C echo start".to_string(), "cmd /C exit 1".to_string()];
    let infos = run_chained(chain);

    assert_eq!(infos.len(), 2);
    assert!(infos[0].status.success());
    assert!(!infos[1].status.success());
    assert!(!infos[1].never_run);
}

#[cfg(target_os = "linux")]
#[test]
fn test_run_chained_captures_stderr() {
    let chain: CommandChainedList = vec!["ls /definitely/not/here".to_string()];
    let infos = run_chained(chain);

    assert_eq!(infos.len(), 1);
    assert!(!infos[0].status.success());
    let err = infos[0].err.as_ref().expect("stderr should be captured");
    assert!(!err.is_empty());
}

#[cfg(target_os = "windows")]
#[test]
fn test_run_chained_captures_stderr() {
    let chain: CommandChainedList = vec![
        r#"cmd /C "echo error 1>&2 & exit /b 1""#.to_string(),
    ];

    let infos = run_chained(chain);

    assert_eq!(infos.len(), 1);
    assert!(!infos[0].status.success());

    let err = infos[0]
        .err
        .as_ref()
        .expect("stderr should be captured");

    assert!(
        String::from_utf8_lossy(err.as_bytes()).contains("error"),
        "expected stderr to contain 'error', got: {:?}",
        String::from_utf8_lossy(err.as_bytes())
    );
}

#[test]
fn test_run_chained_missing_binary() {
    let chain: CommandChainedList = vec!["no_such_binary_xyz".to_string()];
    let infos = run_chained(chain);

    assert_eq!(infos.len(), 1);
    assert!(infos[0].never_run);
    assert!(infos[0].err.is_some());
    assert!(infos[0].out.is_none());
}

// run_multi_chained

#[cfg(target_os = "linux")]
#[test]
fn test_run_multi_chained_sequential() {
    let multi: CommandMultiChainedList = vec![
        vec!["echo one".to_string(), "echo two".to_string()],
        vec!["echo three".to_string()],
    ];
    let res: ProcInfoChainedList = run_multi_chained(multi, false);

    assert_eq!(res.len(), 2);
    assert_eq!(res[0].len(), 2);
    assert_eq!(res[1].len(), 1);
    assert_eq!(res[0][0].out.as_ref().unwrap().stdout, b"one\n");
    assert_eq!(res[0][1].out.as_ref().unwrap().stdout, b"two\n");
    assert_eq!(res[1][0].out.as_ref().unwrap().stdout, b"three\n");
}

#[cfg(target_os = "windows")]
#[test]
fn test_run_multi_chained_sequential() {
    let multi: CommandMultiChainedList = vec![
        vec!["cmd /C echo one".to_string(), "cmd /C echo two".to_string()],
        vec!["cmd /C echo three".to_string()],
    ];
    let res: ProcInfoChainedList = run_multi_chained(multi, false);

    assert_eq!(res.len(), 2);
    assert_eq!(res[0].len(), 2);
    assert_eq!(res[1].len(), 1);
    assert_eq!(res[0][0].out.as_ref().unwrap().stdout, b"one\r\n");
    assert_eq!(res[0][1].out.as_ref().unwrap().stdout, b"two\r\n");
    assert_eq!(res[1][0].out.as_ref().unwrap().stdout, b"three\r\n");
}

#[cfg(target_os = "linux")]
#[test]
fn test_run_multi_chained_separate_proc_runs_concurrently() {
    let multi: CommandMultiChainedList = vec![
        vec!["sleep 0.4".to_string(), "echo a".to_string()],
        vec!["sleep 0.4".to_string(), "echo b".to_string()],
        vec!["sleep 0.4".to_string()],
    ];
    let start = Instant::now();
    let res: ProcInfoChainedList = run_multi_chained(multi, true);
    let elapsed = start.elapsed();

    assert_eq!(res.len(), 3);
    for chain in &res {
        for info in chain {
            assert!(info.status.success());
            assert!(!info.never_run);
        }
    }
    // sequential would take >= 1.2s, concurrent ~0.4s
    assert!(elapsed < Duration::from_millis(1100), "took {:?}", elapsed);
}

#[test]
fn test_run_multi_chained_empty() {
    let res = run_multi_chained(CommandMultiChainedList::new(), true);
    assert!(res.is_empty());

    let res_seq = run_multi_chained(CommandMultiChainedList::new(), false);
    assert!(res_seq.is_empty());
}
