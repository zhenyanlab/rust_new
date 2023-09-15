#[macro_use]

extern crate log;
extern crate env_logger;


#[test]
fn env_log_test (){
    std::env::set_var("MY_LOG_LEVEL", "info");
    env_logger::init_from_env("MY_LOG_LEVEL");

    info!("hi");
}