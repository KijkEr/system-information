use sys_info::{linux_os_release, mem_info};

fn main() {
    let os_info = linux_os_release().unwrap();
    let memory_info = mem_info().unwrap();
    println!("OS Version: {}", os_info.pretty_name());
    println!("Total Memory: {}", memory_info.total);
    println!("Free Memory: {}", memory_info.avail);
}
