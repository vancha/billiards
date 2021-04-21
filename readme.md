# Round robin in rust

## goal:
A simple rust library that just generates a competition schedule based on a list of names.

## usage:
`
let player_names = vec!["Player one".to_string(),
                             "Player two".to_string(),
                             "Player three".to_string()
                             ];
     let round_robin = RoundRobin::new_with_participants(player_names);
     let schedule = round_robin.generate_circle_method();
`

## more info
for any more information, I would suggest to just clone this library and check it out the rustdoc stuff
