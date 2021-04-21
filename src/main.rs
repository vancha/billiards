use round_robit::RoundRobin;

fn main() {
    let rr = RoundRobin::new_with_participants(vec![
        std::string::String::from("Player one"),
        std::string::String::from("Player two"),
        std::string::String::from("Player three"),
    ]);
    let schedule = rr.generate_circle_method();
    println!("{:?}",schedule);

    RoundRobin::print_schedule(schedule);
}
