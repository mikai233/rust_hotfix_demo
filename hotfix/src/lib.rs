use service::login_service::LoginService;

struct LoginServiceImpl;

impl LoginService for LoginServiceImpl {
    fn login(&self, username: &str, password: &str) -> anyhow::Result<bool> {
        println!("login with username:{username} and password:{password}2");
        Ok(false)
    }
}

#[no_mangle]
fn get_login_service() -> Box<dyn LoginService> {
    Box::new(LoginServiceImpl)
}
