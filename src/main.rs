use log::{info, debug};

fn main() {
    let config = simple_web::parse().expect("Parse Arguments Failed");
    config.init_log();
    info!("{}", serde_json::to_string_pretty(&config).expect("print config failed"));

}
