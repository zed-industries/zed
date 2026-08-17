// Take a look at the license at the top of the repository in the LICENSE file.

#![crate_type = "bin"]
#![allow(unused_must_use, non_upper_case_globals)]
#![allow(clippy::manual_range_contains)]

use std::io::{self, BufRead, Write};
use std::str::FromStr;
use sysinfo::{Components, Disks, Networks, Pid, Signal, System, Users};

const signals: &[Signal] = &[
    Signal::Hangup,
    Signal::Interrupt,
    Signal::Quit,
    Signal::Illegal,
    Signal::Trap,
    Signal::Abort,
    Signal::Bus,
    Signal::FloatingPointException,
    Signal::Kill,
    Signal::User1,
    Signal::Segv,
    Signal::User2,
    Signal::Pipe,
    Signal::Alarm,
    Signal::Term,
    Signal::Child,
    Signal::Continue,
    Signal::Stop,
    Signal::TSTP,
    Signal::TTIN,
    Signal::TTOU,
    Signal::Urgent,
    Signal::XCPU,
    Signal::XFSZ,
    Signal::VirtualAlarm,
    Signal::Profiling,
    Signal::Winch,
    Signal::IO,
    Signal::Power,
    Signal::Sys,
];

fn print_help() {
    writeln!(&mut io::stdout(), "== Help menu ==");
    writeln!(&mut io::stdout(), "help               : show this menu");
    writeln!(
        &mut io::stdout(),
        "signals            : show the available signals"
    );
    writeln!(
        &mut io::stdout(),
        "refresh            : reloads all processes information"
    );
    writeln!(
        &mut io::stdout(),
        "refresh [pid]      : reloads corresponding process information"
    );
    writeln!(
        &mut io::stdout(),
        "refresh_components : reloads components information"
    );
    writeln!(
        &mut io::stdout(),
        "refresh_cpu        : reloads CPU information"
    );
    writeln!(
        &mut io::stdout(),
        "refresh_disks      : reloads disks information"
    );
    writeln!(
        &mut io::stdout(),
        "refresh_users      : reloads users information"
    );
    writeln!(
        &mut io::stdout(),
        "refresh_networks   : reloads networks information"
    );
    writeln!(
        &mut io::stdout(),
        "show [pid | name]  : show information of the given process \
         corresponding to [pid | name]"
    );
    writeln!(
        &mut io::stdout(),
        "kill [pid] [signal]: send [signal] to the process with this \
         [pid]. 0 < [signal] < 32"
    );
    writeln!(
        &mut io::stdout(),
        "cpus               : Displays CPUs state"
    );
    writeln!(
        &mut io::stdout(),
        "memory             : Displays memory state"
    );
    writeln!(
        &mut io::stdout(),
        "temperature        : Displays components' temperature"
    );
    writeln!(
        &mut io::stdout(),
        "disks              : Displays disks' information"
    );
    writeln!(
        &mut io::stdout(),
        "network            : Displays network' information"
    );
    writeln!(
        &mut io::stdout(),
        "all                : Displays all process name and pid"
    );
    writeln!(
        &mut io::stdout(),
        "uptime             : Displays system uptime"
    );
    writeln!(
        &mut io::stdout(),
        "boot_time          : Displays system boot time"
    );
    writeln!(
        &mut io::stdout(),
        "vendor_id          : Displays CPU vendor id"
    );
    writeln!(&mut io::stdout(), "brand              : Displays CPU brand");
    writeln!(
        &mut io::stdout(),
        "load_avg           : Displays system load average"
    );
    writeln!(
        &mut io::stdout(),
        "frequency          : Displays CPU frequency"
    );
    writeln!(
        &mut io::stdout(),
        "users              : Displays all users and their groups"
    );
    writeln!(
        &mut io::stdout(),
        "system             : Displays system information (such as name, version and hostname)"
    );
    writeln!(
        &mut io::stdout(),
        "pid                : Display this example's PID"
    );
    writeln!(&mut io::stdout(), "quit               : Exit the program");
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
            writeln!(&mut io::stdout(), "Refreshing disk list...");
            disks.refresh_list();
            writeln!(&mut io::stdout(), "Done.");
        }
        "refresh_users" => {
            writeln!(&mut io::stdout(), "Refreshing user list...");
            users.refresh_list();
            writeln!(&mut io::stdout(), "Done.");
        }
        "refresh_networks" => {
            writeln!(&mut io::stdout(), "Refreshing network list...");
            networks.refresh_list();
            writeln!(&mut io::stdout(), "Done.");
        }
        "refresh_components" => {
            writeln!(&mut io::stdout(), "Refreshing component list...");
            components.refresh_list();
            writeln!(&mut io::stdout(), "Done.");
        }
        "refresh_cpu" => {
            writeln!(&mut io::stdout(), "Refreshing CPUs...");
            sys.refresh_cpu_all();
            writeln!(&mut io::stdout(), "Done.");
        }
        "signals" => {
            let mut nb = 1i32;

            for sig in signals {
                writeln!(&mut io::stdout(), "{nb:2}:{sig:?}");
                nb += 1;
            }
        }
        "cpus" => {
            // Note: you should refresh a few times before using this, so that usage statistics
            // can be ascertained
            writeln!(
                &mut io::stdout(),
                "number of physical cores: {}",
                sys.physical_core_count()
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "Unknown".to_owned()),
            );
            writeln!(
                &mut io::stdout(),
                "total CPU usage: {}%",
                sys.global_cpu_usage(),
            );
            for cpu in sys.cpus() {
                writeln!(&mut io::stdout(), "{cpu:?}");
            }
        }
        "memory" => {
            writeln!(
                &mut io::stdout(),
                "total memory:     {: >10} KB",
                sys.total_memory() / 1_000
            );
            writeln!(
                &mut io::stdout(),
                "available memory: {: >10} KB",
                sys.available_memory() / 1_000
            );
            writeln!(
                &mut io::stdout(),
                "used memory:      {: >10} KB",
                sys.used_memory() / 1_000
            );
            writeln!(
                &mut io::stdout(),
                "total swap:       {: >10} KB",
                sys.total_swap() / 1_000
            );
            writeln!(
                &mut io::stdout(),
                "used swap:        {: >10} KB",
                sys.used_swap() / 1_000
            );
        }
        "quit" | "exit" => return true,
        "all" => {
            for (pid, proc_) in sys.processes() {
                writeln!(
                    &mut io::stdout(),
                    "{}:{} status={:?}",
                    pid,
                    proc_.name().to_string_lossy(),
                    proc_.status()
                );
            }
        }
        "frequency" => {
            for cpu in sys.cpus() {
                writeln!(
                    &mut io::stdout(),
                    "[{}] {} MHz",
                    cpu.name(),
                    cpu.frequency(),
                );
            }
        }
        "vendor_id" => {
            writeln!(
                &mut io::stdout(),
                "vendor ID: {}",
                sys.cpus()[0].vendor_id()
            );
        }
        "brand" => {
            writeln!(&mut io::stdout(), "brand: {}", sys.cpus()[0].brand());
        }
        "load_avg" => {
            let load_avg = System::load_average();
            writeln!(&mut io::stdout(), "one minute     : {}%", load_avg.one);
            writeln!(&mut io::stdout(), "five minutes   : {}%", load_avg.five);
            writeln!(&mut io::stdout(), "fifteen minutes: {}%", load_avg.fifteen);
        }
        e if e.starts_with("show ") => {
            let tmp: Vec<&str> = e.split(' ').collect();

            if tmp.len() != 2 {
                writeln!(
                    &mut io::stdout(),
                    "show command takes a pid or a name in parameter!"
                );
                writeln!(&mut io::stdout(), "example: show 1254");
            } else if let Ok(pid) = Pid::from_str(tmp[1]) {
                match sys.process(pid) {
                    Some(p) => writeln!(&mut io::stdout(), "{:?}", *p),
                    None => writeln!(&mut io::stdout(), "pid \"{pid:?}\" not found"),
                };
            } else {
                let proc_name = tmp[1];
                for proc_ in sys.processes_by_name(proc_name.as_ref()) {
                    writeln!(
                        &mut io::stdout(),
                        "==== {} ====",
                        proc_.name().to_string_lossy()
                    );
                    writeln!(&mut io::stdout(), "{proc_:?}");
                }
            }
        }
        "temperature" => {
            for component in components.iter() {
                writeln!(&mut io::stdout(), "{component:?}");
            }
        }
        "network" => {
            for (interface_name, data) in networks.iter() {
                writeln!(
                    &mut io::stdout(),
                    "{}:\n  ether {}\n  input data  (new / total): {} / {} B\n  output data (new / total): {} / {} B",
                    interface_name,
                    data.mac_address(),
                    data.received(),
                    data.total_received(),
                    data.transmitted(),
                    data.total_transmitted(),
                );
            }
        }
        "show" => {
            writeln!(
                &mut io::stdout(),
                "'show' command expects a pid number or a process name"
            );
        }
        e if e.starts_with("kill ") => {
            let tmp: Vec<&str> = e.split(' ').collect();

            if tmp.len() != 3 {
                writeln!(
                    &mut io::stdout(),
                    "kill command takes the pid and a signal number in parameter!"
                );
                writeln!(&mut io::stdout(), "example: kill 1254 9");
            } else {
                let pid = Pid::from_str(tmp[1]).unwrap();
                let signal = i32::from_str(tmp[2]).unwrap();

                if signal < 1 || signal > 31 {
                    writeln!(
                        &mut io::stdout(),
                        "Signal must be between 0 and 32 ! See the signals list with the \
                         signals command"
                    );
                } else {
                    match sys.process(pid) {
                        Some(p) => {
                            if let Some(res) =
                                p.kill_with(*signals.get(signal as usize - 1).unwrap())
                            {
                                writeln!(&mut io::stdout(), "kill: {res}");
                            } else {
                                writeln!(
                                    &mut io::stdout(),
                                    "kill: signal not supported on this platform"
                                );
                            }
                        }
                        None => {
                            writeln!(&mut io::stdout(), "pid not found");
                        }
                    };
                }
            }
        }
        "disks" => {
            for disk in disks {
                writeln!(&mut io::stdout(), "{disk:?}");
            }
        }
        "users" => {
            for user in users {
                writeln!(
                    &mut io::stdout(),
                    "{:?} => {:?}",
                    user.name(),
                    user.groups()
                );
            }
        }
        "boot_time" => {
            writeln!(&mut io::stdout(), "{} seconds", System::boot_time());
        }
        "uptime" => {
            let up = System::uptime();
            let mut uptime = up;
            let days = uptime / 86400;
            uptime -= days * 86400;
            let hours = uptime / 3600;
            uptime -= hours * 3600;
            let minutes = uptime / 60;
            writeln!(
                &mut io::stdout(),
                "{days} days {hours} hours {minutes} minutes ({up} seconds in total)",
            );
        }
        x if x.starts_with("refresh") => {
            if x == "refresh" {
                writeln!(&mut io::stdout(), "Getting processes' information...");
                sys.refresh_all();
                writeln!(&mut io::stdout(), "Done.");
            } else if x.starts_with("refresh ") {
                writeln!(&mut io::stdout(), "Getting process' information...");
                if let Some(pid) = x
                    .split(' ')
                    .filter_map(|pid| pid.parse().ok())
                    .take(1)
                    .next()
                {
                    if sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid])) != 0 {
                        writeln!(&mut io::stdout(), "Process `{pid}` updated successfully");
                    } else {
                        writeln!(&mut io::stdout(), "Process `{pid}` couldn't be updated...");
                    }
                } else {
                    writeln!(&mut io::stdout(), "Invalid [pid] received...");
                }
            } else {
                writeln!(
                    &mut io::stdout(),
                    "\"{x}\": Unknown command. Enter 'help' if you want to get the commands' \
                     list.",
                );
            }
        }
        "pid" => {
            writeln!(
                &mut io::stdout(),
                "PID: {}",
                sysinfo::get_current_pid().expect("failed to get PID")
            );
        }
        "system" => {
            writeln!(
                &mut io::stdout(),
                "System name:              {}\n\
                 System kernel version:    {}\n\
                 System OS version:        {}\n\
                 System OS (long) version: {}\n\
                 System host name:         {}",
                System::name().unwrap_or_else(|| "<unknown>".to_owned()),
                System::kernel_version().unwrap_or_else(|| "<unknown>".to_owned()),
                System::os_version().unwrap_or_else(|| "<unknown>".to_owned()),
                System::long_os_version().unwrap_or_else(|| "<unknown>".to_owned()),
                System::host_name().unwrap_or_else(|| "<unknown>".to_owned()),
            );
        }
        e => {
            writeln!(
                &mut io::stdout(),
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
