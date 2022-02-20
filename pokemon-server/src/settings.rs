use ini;

pub struct Settings {
    path: String,
}

impl Settings {
    pub fn new(path: String) -> Self {
        Self {
            path
        }
    }

    pub fn get_server_url(&self) -> String {
        let (mut srv, mut ip, mut port) = (String::new(), String::new(), String::new());
        let config = ini!(self.path.as_str());
        ip = config["server"]["ip"].clone().unwrap();
        port = config["server"]["port"].clone().unwrap();
        srv = format!("{}:{}", ip, port);
        srv
    }

    pub fn get_server_ip(&self) -> String {
        let mut ip = String::new();
        let config = ini!(self.path.as_str());
        ip = config["server"]["ip"].clone().unwrap();
        ip  
    }

    pub fn get_server_port(&self) -> String {
        let mut ip = String::new();
        let config = ini!(self.path.as_str());
        ip = config["server"]["port"].clone().unwrap();
        ip     
    }

    pub fn get_database_url(&self) -> String {
        let (mut database, mut prefix, mut user, mut pwd, mut ip, mut port, mut db) = (String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new());
        let config = ini!(self.path.as_str());
        prefix = config["database"]["prefix"].clone().unwrap();
        user = config["database"]["user"].clone().unwrap();
        pwd = config["database"]["pwd"].clone().unwrap();
        ip = config["database"]["ip"].clone().unwrap();
        port = config["database"]["port"].clone().unwrap();
        db = config["database"]["db"].clone().unwrap();
        database = format!("{}://{}:{}@{}:{}/{}", prefix, user, pwd, ip, port, db);    
        database   
    }

    pub fn get_prefix(&self) -> String {
        let mut prefix = String::new();
        let config = ini!(self.path.as_str());
        prefix = config["database"]["port"].clone().unwrap();
        prefix        
    }

    pub fn get_user(&self) -> String {
        let mut user = String::new();
        let config = ini!(self.path.as_str());
        user = config["database"]["user"].clone().unwrap();
        user        
    }

    pub fn get_pwd(&self) -> String {
        let mut pwd = String::new();
        let config = ini!(self.path.as_str());
        pwd = config["database"]["pwd"].clone().unwrap();
        pwd        
    }
    
    pub fn get_database_ip(&self) -> String {
        let mut ip = String::new();
        let config = ini!(self.path.as_str());
        ip = config["database"]["ip"].clone().unwrap();
        ip        
    }

    pub fn get_database_port(&self) -> String {
        let mut port = String::new();
        let config = ini!(self.path.as_str());
        port = config["database"]["port"].clone().unwrap();
        port        
    }
    
    pub fn get_database_name(&self) -> String {
        let mut db = String::new();
        let config = ini!(self.path.as_str());
        db = config["database"]["db"].clone().unwrap();
        db        
    }     
}