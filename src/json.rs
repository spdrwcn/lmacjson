use serde_json::json;  
use std::io::{BufReader, Write};  
use std::fs::File;  
use std::collections::HashMap;  
use std::io::ErrorKind;  
  
pub fn write_mac_to_json(  
    serial_number: &str,  
    wired_mac: &str,  
    wireless_mac: &str,  
    bluetooth_mac: &str,  
) -> String {  
    // 创建JSON数据  
    let mut data_map: HashMap<String, serde_json::Value> = match File::open("mac.json") {  
        Ok(file) => {  
            let reader = BufReader::new(file);  
            match serde_json::from_reader(reader) {  
                Ok(data) => data,  
                Err(e) => {  
                    eprintln!("Error reading existing JSON data: {}", e);  
                    HashMap::new()  
                }  
            }  
        }  
        Err(e) => {  
            if e.kind() == ErrorKind::NotFound {  
                HashMap::new() // 文件不存在时创建一个空的HashMap  
            } else {  
                panic!("Error opening file: {}", e);  
            }  
        }  
    };  
  
    data_map.insert(  
        serial_number.to_string(),  
        json!({  
            "wired_mac": wired_mac,  
            "wireless_mac": wireless_mac,  
            "bluetooth_mac": bluetooth_mac  
        }),  
    );  
  
    // Convert HashMap to a JSON string.  
    let json_string = match serde_json::to_string_pretty(&data_map) {  
        Ok(json_str) => json_str,  
        Err(e) => {  
            eprintln!("Error converting HashMap to JSON: {}", e);  
            return "写入失败: 无法将数据转换为JSON格式".to_string();  
        }  
    };  
  
    // Write the JSON string to file.  
    let write_result = match File::create("mac.json") {  
        Ok(mut file) => {  
            match file.write_all(json_string.as_bytes()) {  
                Ok(_) => "写入成功",  
                Err(e) => {  
                    eprintln!("Error writing to file: {}", e);  
                    "写入失败: 无法将数据写入文件"  
                }  
            }  
        }  
        Err(e) => {  
            eprintln!("Error creating file: {}", e);  
            "写入失败: 无法创建文件"  
        }  
    };  
  
    write_result.to_string()  
}