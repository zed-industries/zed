// Take a look at the license at the top of the repository in the LICENSE file.

#![crate_type = "bin"]
#![allow(unused_must_use, non_upper_case_globals)]
#![allow(clippy::manual_range_contains)]

use std::io::{self, BufRead, Write};
use std::str::FromStr;
use sysinfo::{Components, Disks, Groups, Networks, Pid, SUPPORTED_SIGNALS, System, Users};

fn print_help() {
    println!(
        "\
== Help menu ==

help               : shows this menu
quit               : exits the program

= Refresh commands =

refresh            : reloads all processes information
refresh [pid]      : reloads corresponding process information
refresh_components : reloads components information
refresh_cpu        : reloads CPU information
refresh_disks      : reloads disks information
refresh_networks   : reloads networks information
refresh_users      : reloads users information

= Process commands =

all                : displays all process name and pid
kill [pid] [signal]: sends [signal] to the process with this [pid]. To get the [signal] number, use the `signals` command.
show [pid | name]  : shows information of the given process corresponding to [pid | name]
signals            : shows the available signals
pid                : displays this example's PID

= CPU commands =

brand              : displays CPU brand
cpus               : displays CPUs state
frequency          : displays CPU frequency
vendor_id          : displays CPU vendor id

= Users and groups commands =

groups             : displays all groups
users              : displays all users and their groups

= System commands =

boot_time          : displays system boot time
load_avg           : displays system load average
system             : displays system information (such as name, version and hostname)
uptime             : displays system uptime

= Extra components commands =

disks              : displays disks' information
memory             : displays memory state
network            : displays network' information
temperature        : displays components' temperature"
    );
}

fn interpret_input(
    input: &str,
    sys: &mut System,
    networks: &mut Networks,
    disks: &mut Disks,
    components: &mut Components,
    users: &mut Users,
) -> bool {
    match input.trim() {
        "help" => print_help(),
        "refresh_disks" => {
            println!("Refreshing disk list...");
            disks.refresh(true);
            println!("Done.");
        }
        "refresh_users" => {
            println!("Refreshing user list...");
            users.refresh();
            println!("Done.");
        }
        "refresh_networks" => {
            println!("Refreshing network list...");
            networks.refresh(true);
            println!("Done.");
        }
        "refresh_components" => {
            println!("Refreshing component list...");
            components.refresh(true);
            println!("Done.");
        }
        "refresh_cpu" => {
            println!("Refreshing CPUs...");
            sys.refresh_cpu_all();
            println!("Done.");
        }
        "signals" => {
            for (nb, sig) in SUPPORTED_SIGNALS.iter().enumerate() {
                println!("{:2}:{sig:?}", nb + 1);
            }
        }
        "cpus" => {
            // Note: you should refresh a few times before using this, so that usage statistics
            // can be ascertained
            println!(
                "number of physical cores: {}",
                System::physical_core_count()
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "Unknown".to_owned()),
            );
            println!("total CPU usage: {}%", sys.global_cpu_usage(),);
            for cpu in sys.cpus() {
                println!("{cpu:?}");
            }
        }
        "memory" => {
            println!("total memory:     {: >10} KB", sys.total_memory() / 1_000);
            println!(
                "available memory: {: >10} KB",
                sys.available_memory() / 1_000
            );
            println!("used memory:      {: >10} KB", sys.used_memory() / 1_000);
            println!("total swap:       {: >10} KB", sys.total_swap() / 1_000);
            println!("used swap:        {: >10} KB", sys.used_swap() / 1_000);
        }
        "quit" | "exit" => return true,
        "all" => {
            for (pid, proc_) in sys.processes() {
                println!(
                    "{}:{} status={:?}",
                    pid,
                    proc_.name().to_string_lossy(),
                    proc_.status()
                );
            }
        }
        "frequency" => {
            for cpu in sys.cpus() {
                println!("[{}] {} MHz", cpu.name(), cpu.frequency(),);
            }
        }
        "vendor_id" => {
            println!("vendor ID: {}", sys.cpus()[0].vendor_id());
        }
        "brand" => {
            println!("brand: {}", sys.cpus()[0].brand());
        }
        "load_avg" => {
            let load_avg = System::load_average();
            println!("one minute     : {}%", load_avg.one);
            println!("five minutes   : {}%", load_avg.five);
            println!("fifteen minutes: {}%", load_avg.fifteen);
        }
        e if e.starts_with("show ") => {
            let tmp: Vec<&str> = e.split(' ').filter(|s| !s.is_empty()).collect();

            if tmp.len() != 2 {
                println!("show command takes a pid or a name in parameter!");
                println!("example: show 1254");
            } else if let Ok(pid) = Pid::from_str(tmp[1]) {
                match sys.process(pid) {
                    Some(p) => {
                        println!("{:?}", *p);
                        println!(
                            "Files open/limit: {:?}/{:?}",
                            p.open_files(),
                            p.open_files_limit(),
                        );
                    }
                    None => {
                        println!("pid \"{pid:?}\" not found");
                    }
                }
            } else {
                let proc_name = tmp[1];
                for proc_ in sys.processes_by_name(proc_name.as_ref()) {
                    println!("==== {} ====", proc_.name().to_string_lossy());
                    println!("{proc_:?}");
                }
            }
        }
        "temperature" => {
            for component in components.iter() {
                println!("{component:?}");
            }
        }
        "network" => {
            for (interface_name, data) in networks.iter() {
                println!(
                    "\
{interface_name}:
  ether {}
  input data  (new / total): {} / {} B
  output data (new / total): {} / {} B",
                    data.mac_address(),
                    data.received(),
                    data.total_received(),
                    data.transmitted(),
                    data.total_transmitted(),
                );
            }
        }
        "show" => {
            println!("'show' command expects a pid number or a process name");
        }
        e if e.starts_with("kill ") => {
            let tmp: Vec<&str> = e
                .split(' ')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();

            if tmp.len() != 3 {
                println!("kill command takes the pid and a signal number in parameter!");
                println!("example: kill 1254 9");
            } else {
                let Ok(pid) = Pid::from_str(tmp[1]) else {
                    eprintln!("Expected a number for the PID, found {:?}", tmp[1]);
                    return false;
                };
                let Ok(signal) = usize::from_str(tmp[2]) else {
                    eprintln!("Expected a number for the signal, found {:?}", tmp[2]);
                    return false;
                };
                let Some(signal) = SUPPORTED_SIGNALS.get(signal) else {
                    eprintln!(
                        "No signal matching {signal}. Use the `signals` command to get the \
                         list of signals.",
                    );
                    return false;
                };

                match sys.process(pid) {
                    Some(p) => {
                        if let Some(res) = p.kill_with(*signal) {
                            println!("kill: {res}");
                        } else {
                            eprintln!("kill: signal not supported on this platform");
                        }
                    }
                    None => {
                        eprintln!("pid not found");
                    }
                }
            }
        }
        "disks" => {
            for disk in disks {
                println!("{disk:?}");
            }
        }
        "users" => {
            for user in users {
                println!("{:?} => {:?}", user.name(), user.groups(),);
            }
        }
        "groups" => {
            for group in Groups::new_with_refreshed_list().list() {
                println!("{group:?}");
            }
        }
        "boot_time" => {
            println!("{} seconds", System::boot_time());
        }
        "uptime" => {
            let up = System::uptime();
            let mut uptime = up;
            let days = uptime / 86400;
            uptime -= days * 86400;
            let hours = uptime / 3600;
            uptime -= hours * 3600;
            let minutes = uptime / 60;
            println!("{days} days {hours} hours {minutes} minutes ({up} seconds in total)",);
        }
        x if x.starts_with("refresh") => {
            if x == "refresh" {
                println!("Getting processes' information...");
                sys.refresh_all();
                println!("Done.");
            } else if x.starts_with("refresh ") {
                println!("Getting process' information...");
                if let Some(pid) = x
                    .split(' ')
                    .filter_map(|pid| pid.parse().ok())
                    .take(1)
                    .next()
                {
                    if sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), true) != 0 {
                        println!("Process `{pid}` updated successfully");
                    } else {
                        println!("Process `{pid}` couldn't be updated...");
                    }
                } else {
                    println!("Invalid [pid] received...");
                }
            } else {
                println!(
                    "\"{x}\": Unknown command. Enter 'help' if you want to get the commands' \
                     list.",
                );
            }
        }
        "pid" => {
            println!(
                "PID: {}",
                sysinfo::get_current_pid().expect("failed to get PID")
            );
        }
        "system" => {
            println!(
                "System name:              {}\n\
                 System kernel version:    {}\n\
                 System OS version:        {}\n\
                 System OS (long) version: {}\n\
                 System host name:         {}\n\
                 System kernel:            {}",
                System::name().unwrap_or_else(|| "<unknown>".to_owned()),
                System::kernel_version().unwrap_or_else(|| "<unknown>".to_owned()),
                System::os_version().unwrap_or_else(|| "<unknown>".to_owned()),
                System::long_os_version().unwrap_or_else(|| "<unknown>".to_owned()),
                System::host_name().unwrap_or_else(|| "<unknown>".to_owned()),
                System::kernel_long_version(),
            );
        }
        e => {
            println!(
                "\"{e}\": Unknown command. Enter 'help' if you want to get the commands' \
                 list.",
            );
        }
    }
    false
}

fn main() {
    println!("Getting system information...");
    let mut system = System::new_all();
    let mut networks = Networks::new_with_refreshed_list();
    let mut disks = Disks::new_with_refreshed_list();
    let mut components = Components::new_with_refreshed_list();
    let mut users = Users::new_with_refreshed_list();

    println!("Done.");
    let t_stin = io::stdin();
    let mut stin = t_stin.lock();
    let mut done = false;

    println!("To get the commands' list, enter 'help'.");
    while !done {
        let mut input = String::new();
        write!(&mut io::stdout(), "> ");
        io::stdout().flush();

        stin.read_line(&mut input);
        if input.is_empty() {
            // The string is empty, meaning there is no '\n', meaning
            // that the user used CTRL+D so we can just quit!
            println!("\nLeaving, bye!");
            break;
        }
        if (&input as &str).ends_with('\n') {
            input.pop();
        }
        done = interpret_input(
            input.as_ref(),
            &mut system,
            &mut networks,
            &mut disks,
            &mut components,
            &mut users,
        );
    }
}
