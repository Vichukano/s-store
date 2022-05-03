fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init_timed();
    log::debug!("s-store started!");
}
