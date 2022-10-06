use key_logger::db::get_connection_pool;

fn main() {
    println!("{:?}", get_connection_pool());
}
