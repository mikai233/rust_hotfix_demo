pub trait LoginService: Send + Sync {
    fn login(&self, username: &str, password: &str) -> anyhow::Result<bool>;
}

#[derive(Debug)]
pub struct LoginServiceImpl;

impl LoginService for LoginServiceImpl {
    fn login(&self, username: &str, password: &str) -> anyhow::Result<bool> {
        println!("default login");
        if username == "admin" && password == "admin" {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}