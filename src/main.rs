use round_robit::RoundRobin;

fn main() {
    let rr = RoundRobin::new_with_participants(vec![
        std::string::String::from("Enrique West"),
        std::string::String::from("Damien Braun"),
        std::string::String::from("Ellie Osborne"),
        std::string::String::from("Cierra Vega"),
        std::string::String::from("Alden Cantrell"),
        std::string::String::from("Kierra Gentry"),
    ]);
    let schedule = rr.generate_circle_method();
    RoundRobin::print_schedule(schedule);
}
