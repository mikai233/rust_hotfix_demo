use service::login_service::{LoginService, LoginServiceImpl};
use std::collections::HashMap;

struct Services {
    libs: HashMap<String, libloading::Library>,
    login_service: Box<dyn LoginService>,
}

impl Services {
    fn new() -> Services {
        Services {
            libs: Default::default(),
            login_service: Box::new(LoginServiceImpl),
        }
    }
}

fn main() {
    let mut services = Services::new();
    let stdin = std::io::stdin();
    let path = "/mnt/f/CLionProjects/hotfix_demo/target/debug/libhotfix.so";
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "load" => {
                unsafe {
                    let lib = libloading::Library::new(path).unwrap();
                    let login_service: libloading::Symbol<fn() -> Box<dyn LoginService>> = lib.get(b"get_login_service").unwrap();
                    services.login_service = login_service();
                    services.libs.insert(path.to_string(), lib);
                    println!("{} load done", path);
                }
            }
            "unload" => {
                services.login_service = Box::new(LoginServiceImpl);
                services.libs.remove(path);
                println!("{} unload done", path);
            }
            _ => {
                services.login_service.login("admin", "admin").unwrap();
            }
        }
    }
}