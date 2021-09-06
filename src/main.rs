// FalseGhost

#[macro_use]
extern crate clap;
use clap::App;

use tokio;
use durga;

mod extensions;

#[tokio::main]
async fn main() {

    durga::banner();
    let yaml = load_yaml!("../resources/cli.yaml");
    let argv = App::from_yaml(yaml).get_matches();
    
    let unresolved_target = argv.value_of("target_ip").unwrap();
    let timeout = std::time::Duration::from_secs(1);
    
    let my_target = durga::resolve_target(&unresolved_target.to_string());
    println!("[*] Scanning {} -> {}", argv.value_of("target_ip").unwrap(), durga::resolve_target(&unresolved_target.to_string()).unwrap());

    
    match argv.occurrences_of("full_scan") {
        0 => {
            durga::scan(my_target.unwrap(), false, 1024, timeout, unresolved_target)
                .await;
        },
        1 => {
            println!("[*] Running Full TCP Scan");
            durga::scan(my_target.unwrap(), true, 1024, timeout, unresolved_target)
                .await;
        }
        _ => ()
    }

    let open_ports_list = std::fs::read_to_string(format!("/tmp/{}.txt", unresolved_target)).unwrap();
 
    for line in open_ports_list.lines() {
        extensions::run_extensions(line.parse::<u16>().unwrap(), unresolved_target);
    }

    durga::run_command(format!("rm /tmp/{}.txt", unresolved_target).as_str());   
}