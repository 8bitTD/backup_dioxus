use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use super::define::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowInfo{
    pub pos_x: i32,
    pub pos_y: i32,
    pub width: u32,
    pub height: u32,
}
impl Default for WindowInfo{
    fn default() -> WindowInfo{
        WindowInfo { pos_x: 600, pos_y: 200, width: 600, height: 600 }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Json{
    pub backup_path: String,
    pub backup_text: String,
    pub wi: WindowInfo,
}
impl Default for Json{
    fn default() -> Json{
        Json {
            backup_path: String::new(),
            backup_text: String::new(),
            wi: WindowInfo::default(),
        }
    }
}
impl Json{
    pub fn new() -> Json{
        let mut jsn = Json::default();
        jsn.load();
        return jsn;
    }

    pub fn load(&mut self){
        let mut json_path: String = dirs::home_dir().unwrap().as_os_str().to_str().unwrap().to_string();
        let rust_path = format!("{}{}",&json_path, "\\Documents\\script\\Rust\\");
        if !std::path::Path::new(&rust_path).is_dir(){Some(std::fs::create_dir_all(&rust_path));}
        json_path.push_str(format!("{}{}{}","\\Documents\\script\\Rust\\",common::TOOLNAME,".json").replace("\\","/").as_str());
        let contents = match std::fs::read_to_string(&json_path) {                                                
            Ok(contents) => contents,                                                  
            Err(_error) => {return;},                                                                 
        }; 
        let jsn:Result<Json,_> = serde_json::from_str(&contents);
        if jsn.is_err(){return;}
        let mut jsn = jsn.unwrap();
        if jsn.wi.pos_x == -32000{ jsn.wi.pos_x = 0; }
        if jsn.wi.pos_y == -32000{ jsn.wi.pos_y = 0; }
        if jsn.wi.width == 0 {jsn.wi.width = 600;}
        if jsn.wi.height == 0 {jsn.wi.height = 600;}
        *self = jsn;
    }

    pub fn save(&self){
        let content = serde_json::to_string_pretty(&self).unwrap();
        let mut json_path: String = dirs::home_dir().unwrap().as_os_str().to_str().unwrap().to_string();
        let path = std::path::Path::new(&json_path);
        if !path.is_dir(){Some(std::fs::create_dir_all(path));}
        json_path.push_str(format!("{}{}{}","\\Documents\\script\\Rust\\",common::TOOLNAME,".json").as_str());
        let mut file = std::fs::File::create(&json_path).expect("create failed");
        file.write_all(content.as_bytes()).unwrap();
    }
}