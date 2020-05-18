use std::process::Command;
use std::{thread, time};
use sysinfo::{ProcessExt, System, SystemExt};
fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let hs_process = sys
        .get_processes()
        .into_iter()
        .map(|x| x.1)
        .filter(|x| x.name() == "Hearthstone.exe")
        .next();
    if let Some(hs) = hs_process {
        let path: String = hs.cmd()[0].clone();
        let cmd_delete = String::from("netsh advfirewall firewall delete rule name=\"hsskipper\"");
        let cmd_create = format!("netsh advfirewall firewall add rule name=\"hsskipper\" dir=out program={} action=block",path);
        let output_create = Command::new("cmd")
            .args(&["/C", cmd_create.as_str()])
            .output()
            .expect("failed to execute process");

        println!(
            "{:?}",
            output_create
                .stdout
                .iter()
                .map(|x| { *x as char })
                .collect::<String>()
        );
        let one_sec = time::Duration::from_millis(1000);
        thread::sleep(one_sec);
        let output_delete = Command::new("cmd")
            .args(&["/C", cmd_delete.as_str()])
            .output()
            .expect("failed to execute process");
        println!(
            "{:?}",
            output_delete
                .stdout
                .iter()
                .map(|x| { *x as char })
                .collect::<String>()
        );
    } else {
        println!("{:?}", "hearth stone not launched");
    }
}
