//#![windows_subsystem = "windows"]   
  
mod sn;  
mod mac;  
mod json;  
  
fn main() {  
    let serial_number = sn::get_bios_serial_number().unwrap_or_else(|err| {  
        eprintln!("Error fetching serial number: {}", err);  
        "Unknown".to_string()  
    });  
    let (wired_mac, wireless_mac, bluetooth_mac) = mac::get_mac_addresses();  
    let json_status = json::write_mac_to_json(  
        &serial_number,  
        &wired_mac,  
        &wireless_mac,  
        &bluetooth_mac,  
    );  
    println!("SN: {}", serial_number);  
    println!("Wired MAC: {}", wired_mac);  
    println!("Wireless MAC: {}", wireless_mac);  
    println!("Bluetooth MAC: {}", bluetooth_mac);
    println!("JSON write status: {}", json_status);  
    println!("Press Enter to exit...");  
    std::io::stdin().read_line(&mut String::new()).unwrap();  

}