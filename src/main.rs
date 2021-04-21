use round_robit::RoundRobin;

fn main() {
    let x = RoundRobin::new_with_participants(vec![
        /*std::string::String::from("Enrique West"),
        std::string::String::from("Damien Braun"),
        std::string::String::from("Ellie Osborne"),
        std::string::String::from("Cierra Vega"),
        std::string::String::from("Alden Cantrell"),
        std::string::String::from("Kierra Gentry"),*/
        std::string::String::from("1"),
        std::string::String::from("2"),
        std::string::String::from("3"),
        std::string::String::from("4"),/*
        std::string::String::from("5"),
        std::string::String::from("6"),
        std::string::String::from("7"),
        std::string::String::from("8"),
        std::string::String::from("9"),
        std::string::String::from("10"),
        std::string::String::from("11"),
        std::string::String::from("12"),
        std::string::String::from("13"),
        std::string::String::from("14"),*/
    ]);
    let x = x.generate_circle_method();
    RoundRobin::print_schedule(x);
}
