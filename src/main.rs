#![windows_subsystem = "windows"]

use serde_json::json;  
use std::collections::HashMap; 
use std::fs::File;  
use std::io::{BufRead, BufReader, Write};  
use std::process::Command;  
use std::result::Result;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wiredk: Vec<&str> = vec!["gbe", "true"];
    let wirelessk: Vec<&str> = vec!["wi-fi", "true"];
    let bluetoothk: Vec<&str> = vec!["bluetooth", "true"];
    let output = Command::new("wmic")
        .args(&[
            "path",
            "win32_networkadapter",
            "get",
            "name,macaddress,physicaladapter",
        ])
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute wmic command")
        .stdout
        .unwrap();
    let reader = BufReader::new(output);
    let serial_number = get_bios_serial_number()?;
    let mut wired_mac = String::new();
    let mut wireless_mac = String::new();
    let mut bluetooth_mac = String::new();
    // 获取MAC地址
    let mut mac_found = false;
    for line in reader.lines() {
        let line = line.unwrap();
        let line_lower = line.to_lowercase();
        let contains_all = |keywords: &[&str]| keywords.iter().all(|kw| line_lower.contains(kw));
        if contains_all(&wiredk) {
            if wired_mac.is_empty() {
                wired_mac = extract_mac_address(&line);
                mac_found = true;
            }
        } else if contains_all(&wirelessk) {
            if wireless_mac.is_empty() {
                wireless_mac = extract_mac_address(&line);
                mac_found |= !mac_found;
            }
        } else if contains_all(&bluetoothk) {
            if bluetooth_mac.is_empty() {
                bluetooth_mac = extract_mac_address(&line);
                mac_found |= !mac_found;
            }
        }
    }
    check_and_assign_if_empty(&mut wired_mac);
    check_and_assign_if_empty(&mut wireless_mac);
    check_and_assign_if_empty(&mut bluetooth_mac);

    let mut data_map: HashMap<String, serde_json::Value> = match File::open("mac.json") {  
        Ok(file) => {  
            let reader = BufReader::new(file);  
            let existing_data: HashMap<String, serde_json::Value> = serde_json::from_reader(reader)?;  
            existing_data  
        }  
        Err(_) => HashMap::new(), // 文件不存在时创建一个空的HashMap  
    };  

    if mac_found {
        data_map.insert(  
            serial_number,  
            json!({  
                "wired_mac": wired_mac,  
                "wireless_mac": wireless_mac,  
                "bluetooth_mac": bluetooth_mac  
            }),  
        ); 
        let json_string = serde_json::to_string_pretty(&data_map)?;  
        let mut file = File::create("mac.json")?;  
        file.write_all(json_string.as_bytes())?;  
    }  
    Ok(())
}
//MAC变量检查
fn check_and_assign_if_empty(s: &mut String) {
    if s.is_empty() {
        *s = "未采集".to_string();
    }
}
//MAC地址处理
fn extract_mac_address(line: &str) -> String {
    line.chars().take(17).collect::<String>()
}
// 获取序列号
fn get_bios_serial_number() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("wmic")
        .arg("bios")
        .arg("get")
        .arg("serialnumber")
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let lines: Vec<&str> = stdout.split('\n').collect::<Vec<_>>();
    match lines.get(1) {
        Some(line) => {
            let serial = line.trim().split_whitespace().last().map(|s| s.to_string());
            match serial {
                Some(s) => Ok(s),
                None => Err("Failed to parse BIOS serial number from WMIC output.".into()),
            }
        }
        None => Err("WMIC output did not contain the expected lines.".into()),
    }
}
