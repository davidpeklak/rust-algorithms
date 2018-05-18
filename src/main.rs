extern crate algorithms;

use std::io;

use algorithms::quick_find::QuickFind;

fn main() {
    println!("Enter size:");
    let size = read_i32();

    let mut vec: Vec<u32> = QuickFind::new(size as usize);

    println!("{:?}", vec);

    loop {
        println!("Enter first:");
        let first = read_i32();
        if first < 0 {
            break;
        }

        println!("Enter second:");
        let second = read_i32();

        let connected = vec.is_connected(first as usize, second as usize);
        println!("{} and {} are {} connected", first, second, if connected {""} else {"not"});

        vec.connect(first as usize, second as usize);

        println!("{:?}", vec);
    }
}

fn read_i32() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    s.trim_right().parse::<i32>().unwrap()
}

