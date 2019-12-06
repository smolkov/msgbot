use sysinfo::{NetworkExt, System, SystemExt};




pub async fn systeminfo(msg: &str) -> String {

    let mut sys = System::new();

    // We display the disks:
    println!("=> disk list:");
    for disk in sys.get_disks() {
        println!("{:?}", disk);
    }

    // Network data:
    println!("input data : {} B", sys.get_network().get_income());
    println!("output data: {} B", sys.get_network().get_outcome());

    // Components temperature:
    for component in sys.get_components_list() {
        println!("{:?}", component);
    }

    // Memory information:
    println!("total memory: {} kB", sys.get_total_memory());
    println!("used memory : {} kB", sys.get_used_memory());
    println!("total swap  : {} kB", sys.get_total_swap());
    println!("used swap   : {} kB", sys.get_used_swap());

    // Number of processors
    println!("NB processors: {}", sys.get_processor_list().len());
    String::from("")
}
