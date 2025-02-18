use crate::common::util::*;

#[test]
fn test_uname() {
    new_ucmd!().succeeds();
}

#[test]
fn test_uname_compatible() {
    new_ucmd!().arg("-a").succeeds();
}

#[test]
fn test_uname_name() {
    new_ucmd!().arg("-n").succeeds();
}

#[test]
fn test_uname_processor() {
    let result = new_ucmd!().arg("-p").succeeds();
    assert_eq!(result.stdout_str().trim_end(), "unknown");
}

#[test]
fn test_uname_hardware_platform() {
    let result = new_ucmd!().arg("-i").succeeds();
    assert_eq!(result.stdout_str().trim_end(), "unknown");
}

#[test]
fn test_uname_machine() {
    new_ucmd!().arg("-m").succeeds();
}

#[test]
fn test_uname_kernel_version() {
    new_ucmd!().arg("-v").succeeds();
}

#[test]
fn test_uname_kernel() {
    let (_, mut ucmd) = at_and_ucmd!();

    #[cfg(target_os = "linux")]
    {
        let result = ucmd.arg("-o").succeeds();
        assert!(result.stdout_str().to_lowercase().contains("linux"));
    }

    #[cfg(not(target_os = "linux"))]
    ucmd.arg("-o").succeeds();
}

#[test]
fn test_uname_operating_system() {
    #[cfg(target_os = "android")]
    new_ucmd!()
        .arg("--operating-system")
        .succeeds()
        .stdout_is("Android\n");
    #[cfg(target_vendor = "apple")]
    new_ucmd!()
        .arg("--operating-system")
        .succeeds()
        .stdout_is("Darwin\n");
    #[cfg(target_os = "freebsd")]
    new_ucmd!()
        .arg("--operating-system")
        .succeeds()
        .stdout_is("FreeBSD\n");
    #[cfg(target_os = "fuchsia")]
    new_ucmd!()
        .arg("--operating-system")
        .succeeds()
        .stdout_is("Fuchsia\n");
    #[cfg(all(target_os = "linux", any(target_env = "gnu", target_env = "")))]
    new_ucmd!()
        .arg("--operating-system")
        .succeeds()
        .stdout_is("GNU/Linux\n");
    #[cfg(all(target_os = "linux", not(any(target_env = "gnu", target_env = ""))))]
    new_ucmd!()
        .arg("--operating-system")
        .succeeds()
        .stdout_is("Linux\n");
    #[cfg(target_os = "netbsd")]
    new_ucmd!()
        .arg("--operating-system")
        .succeeds()
        .stdout_is("NetBSD\n");
    #[cfg(target_os = "openbsd")]
    new_ucmd!()
        .arg("--operating-system")
        .succeeds()
        .stdout_is("OpenBSD\n");
    #[cfg(target_os = "redox")]
    new_ucmd!()
        .arg("--operating-system")
        .succeeds()
        .stdout_is("Redox\n");
    #[cfg(target_os = "windows")]
    new_ucmd!()
        .arg("--operating-system")
        .succeeds()
        .stdout_is("Windows NT\n");
}

#[test]
fn test_uname_help() {
    new_ucmd!()
        .arg("--help")
        .succeeds()
        .stdout_contains("system information");
}
