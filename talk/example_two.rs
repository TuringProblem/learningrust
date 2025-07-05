struct Human {
    name: String,
    age: u8,
}

impl Human {
    fn new_human(name: &str, age: u8) -> Human {
        Human {
            name: name.to_string(),
            age: age,
        }
    }

    fn get_name(human: &Human) -> &str {
        &human.name
    }
    fn get_age(human: &Human) -> u8 {
        human.age
    }
}

fn main() {
    println!("Hello, World!\n\n");
    let my_human = Human::new_human("Andrew", 12);
    println!(
        "My name is {}\nmy age is {}\n\n===Next example===\n\n",
        Human::get_name(&my_human),
        Human::get_age(&my_human)
    );

    let humans = vec![
        Human::new_human("Andrew", 42),
        Human::new_human("Bob", 30),
        Human::new_human("Charlie", 10),
    ];

    for human in humans {
        println!(
            "My name is {}\nmy age is {}",
            Human::get_name(&human),
            Human::get_age(&human)
        );
    }
}
