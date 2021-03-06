/*
    dotnetdetect
    Small utility to detect version of .NET Framework
    installed on a Windows system.

    Copyright (c) 2017 Sam Saint-Pettersen.
    Released under the MIT License.
*/

extern crate clioptions;
extern crate winreg;
use clioptions::CliOptions;
use winreg::RegKey;
use winreg::enums::*;
use std::process::exit;

fn detect_dotnet_version(verbose: u32) -> f32 {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let netp = hklm.open_subkey_with_flags(
    "SOFTWARE\\Microsoft\\NET Framework Setup\\NDP\\v4\\Full", KEY_READ).unwrap();
    let dotnetv: u32 = netp.get_value("Release").unwrap();
    let ver: f32;
    match dotnetv {
        460805 | 460798 => ver =  4.7,
        394806 | 394802 => ver = 4.62,
        394254 | 394271 => ver = 4.61,
                 379893 => ver = 4.52,
        378675 | 378758 => ver = 4.51,
        378389 => ver = 4.5,
        _ => ver = 4.4,
    }
    if verbose > 0 {
        let msg = "Detected .NET Framework =>";
        if ver == 4.4 {
            println!("{} < 4.5", msg);
        }
        println!("{} v{}", msg, ver);
    }
    ver
}

fn is_required_dotnet(req: f32, verbose: u32) -> i32 {
    let mut ec = -1 as i32;
    let ver = detect_dotnet_version(verbose);
    if ver >= req {
        if verbose > 1 {
            println!("Meets or exceeds required version => {}", req);
        }
        ec = 0;
    } else if verbose > 1 {
        println!("Does not meet required version => {}", req);
    }
    ec
}

fn display_version() {
    println!("dotnetdetect v0.1.0");
    exit(0);
}

fn display_usage(program: &str) {
    println!("dotnetdetect");
    println!("Small utility to detect version of .NET Framework");
    println!("installed on a Windows system.");
    println!("\nCopyright 2017 Sam Saint-Pettersen.");
    println!("\nReleased under the MIT License.");
    println!("\nUsage: {} [-h | -v][-r <dotnet version>][-q | -p]", program);
    println!("\nOptions are:\n");
    println!("-r | --required: The required minimal .NET version.");
    println!("-q | --quiet: Do not output version to stdout; just exit code (0 for pass; -1 for fail).");
    println!("-p | --part-quiet: Output version to stdout, but not \"Meets...\" message.");
    println!("-h | --help: Display this usage information and exit.");
    println!("-v | --version: Display program version information and exit.");
    exit(0);
}

fn parse_unit(unit: &str) -> f32 {
    let n = unit.parse::<f32>().ok();
    let unit = match n {
        Some(unit) => unit as f32,
        None => 0 as f32,
    };
    unit
}

fn main() {
    let cli = CliOptions::new("dotnetdetect");
    let program = cli.get_program();
    let mut required = String::new();
    let mut verbose = 2;
    if cli.get_num() > 1 {
        for (i, a) in cli.get_args().iter().enumerate() {
            match a.trim() {
                "-r" | "--required" => required = cli.next_argument(i),
                "-p" | "--part-quiet" => verbose = 1,
                "-q" | "--quiet" => verbose = 0,
                "-h" | "--help" => display_usage(&program),
                "-v" | "--version" => display_version(),
                _ => continue,
            }
        }
        let ec = is_required_dotnet(parse_unit(&required), verbose);
        exit(ec);
    }
    detect_dotnet_version(verbose);
    exit(0);
}
