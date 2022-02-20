use ini;

#[derive(Clone)]
pub struct Settings {
    path: String,
}

impl Settings {
    pub fn init() -> Self {
        Self {
            path: String::from("")
        }
    }

    pub fn set(path: String) -> Self {
        Self {
            path
        }
    }

    pub fn get_connected_url(&self) -> String {
        let (mut endpoint, mut ip, mut port) = (String::new(), String::new(), String::new());
        let config = ini!(self.path.as_str());
        ip = config["clients"]["ip"].clone().unwrap();
        port = config["clients"]["port"].clone().unwrap();
        endpoint = format!("{}:{}", ip, port);
        endpoint
    }

    pub fn get_connected_ip(&self) -> String {
        let mut ip = String::new();
        let config = ini!(self.path.as_str());
        ip = config["clients"]["ip"].clone().unwrap();
        ip  
    }

    pub fn get_server_port(&self) -> String {
        let mut port = String::new();
        let config = ini!(self.path.as_str());
        port = config["clients"]["port"].clone().unwrap();
        port
    }  

    pub fn get_trader_nums(&self) -> i32 {
        let mut num = String::new();
        let config = ini!(self.path.as_str());
        num = config["clients"]["trader_num"].clone().unwrap();
        num.parse::<i32>().unwrap()
    }    

    pub fn get_duration(&self) -> i32 {
        let mut duration = String::new();
        let config = ini!(self.path.as_str());
        duration = config["clients"]["duration"].clone().unwrap();
        duration.parse::<i32>().unwrap()
    }     
}