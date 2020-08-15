use griffin::config::Config;

fn main() {
    let c = Config::new("griffin.yaml").unwrap();
    println!("{:?}", c);
}
