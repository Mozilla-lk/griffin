use griffin::config::Config;

fn main() {
    let c = Config::new_from_file("griffin.yaml").unwrap();
    println!("{:?}", c);
}
