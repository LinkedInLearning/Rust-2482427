// Implémenter une enum avec du pattern matching
enum Direction {
    Nord,
    Sud,
    Est,
    Ouest,
}

impl Direction {
    fn info(&self) {
        match self {
            Direction::Nord => println!("Je suis au nord"),
            Direction::Sud => println!("Je suis au sud"),
            Direction::Est => println!("Je suis à l'est"),
            Direction::Ouest => println!("Je suis à l'ouest"),
        }
    }
}

enum IpAddr {
    V4(String),
    V6(String),
}

impl IpAddr {
    fn addr(&self) -> String {
        match self {
            IpAddr::V4(ip_addr_str) | IpAddr::V6(ip_addr_str) => ip_addr_str.clone(),
        }
    }
}

fn main() {
    let direction = Direction::Nord;
    direction.info();
    let ip_addr = IpAddr::V4(String::from("127.0.0.1")).addr();
}
