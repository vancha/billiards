//!A simple library to generate match schedules for games like billiards
//!
//!This library consists of a single struct, that can be initiated with a list of player names
//!it's hopefully easy to use and offers only basic functionality. 
//!
//!Keep note: when the number of players is uneven, a dummy player is added.
//!
//!# Example:
//!
//!To generate a match schedule for say 3 players, use the library like this:
//!
//!```
//!     use round_robit::RoundRobin;
//!     let player_names = vec!["Player one".to_string(),
//!                             "Player two".to_string(),
//!                             "Player three".to_string()
//!                             ];
//!     let round_robin = RoundRobin::new_with_participants(player_names);
//!     let schedule = round_robin.generate_circle_method();
//!```
//!
//! here `schedule` will hold a vec of rounds, where every round is a vec of single matches.
//! structurally it looks like this:
//!
//!`[[("Player one", "Dummy Player"), ("Player two", "Player three")], [("Player one", "Player three"), ("Dummy Player", "Player two")], [("Player one", "Player two"), ("Player three", "Dummy Player")]]`

///The main struct that allows you to generate schedules
///
///
///
pub struct RoundRobin {
    participants: Vec<String>,
}

impl RoundRobin {
    ///returns a new instance of the roundrobin generator, pass it a list of contestant names as
    ///Strings
    pub fn new_with_participants(participants: Vec<String>) -> Self {
        RoundRobin {
            participants: participants,
        }
    }
    ///return the list of participants provided as arguments for creating the roundrobin struct
    pub fn get_participants(&self) -> &Vec<String> {
        &self.participants //.clone()
    }

    ///print the matches, utility function
    ///
    ///takes a schedule in the form of ```Vec<Vec<(String,String)>>```
    pub fn print_schedule(schedule: Vec<Vec<(String, String)>>) {
        for round in schedule {
            for single_match in round {
                println!("({} : {})", single_match.0, single_match.1);
            }
            println!("\n");
        }
    }

    ///generates the actual matches, in the for of a list of rounds to play.
    ///rounds are a combination of two players, here structed as a tuple
    ///
    ///a schedule with two rounds, would look like this:
    ///```
    ///let example =  vec![ //the outer vec, the "collection of rounds"
    ///
    ///     vec![("a","b"),("c","d")],//a round, player a against b, and c against d
    ///     vec![("e","f"),("g","h")],//another round. 
    ///];
    ///```
    pub fn generate_circle_method(&self) -> Vec<Vec<(String, String)>> {
        //get the collection of players
        let mut tmp_element = self.participants.clone();
        //if there's an uneven number of players, add dummy player
        if tmp_element.len() % 2 != 0 {
            tmp_element.append(&mut vec![std::string::String::from("Dummy Player")]);
        }
        //a collection of all rounds
        let mut rounds: Vec<Vec<(String, String)>> = vec![];
        //loop as often as there are rounds to be played
        for _ in 0..tmp_element.len() - 1 {
            //If there is a first element to split off, do it. (this represents the fixed player)
            if let Some((first, elements)) = tmp_element.split_first_mut() {
                let mut first_row = vec![first.clone()];
                let mut nya = (&elements[0..(elements.len() / 2)]).to_vec();
                //prepend the first element again
                first_row.append(&mut nya);
                //reverse the second row
                let second_row = &elements[(elements.len() / 2)..elements.len()]
                    .iter()
                    .rev()
                    .collect::<Vec<&String>>()
                    .to_vec();
                //a vec to hold single rounds
                let mut full_round: Vec<(String, String)> = vec![];
                //generate the rounds
                for (i, s) in first_row.iter().enumerate() {
                    //a round is just a combination of the current player in the first row, being
                    //matched with the player in the same index in the second row
                    let current_match = (s.clone(), second_row[i].clone()); //println!("{}:{}",s,second_row[i]);
                    full_round.push(current_match);
                }
                //when this round is generated, add it to the list of rounds that make up the
                //entire schedule
                rounds.append(&mut vec![full_round]);
                //rotate the element list for the next iteration of round generation
                elements.rotate_right(1);
            }
        }
        rounds
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    //this test uses the example on
    //https://en.wikipedia.org/wiki/Round-robin_tournament#Circle_method, but with 6 instead of 14
    //players
    fn it_works() {
        let x = RoundRobin::new_with_participants(vec![
            std::string::String::from("1"),
            std::string::String::from("2"),
            std::string::String::from("3"),
            std::string::String::from("4"),
            std::string::String::from("5"),
            std::string::String::from("6"),
        ]);
        let matches = x.generate_circle_method();
        assert_eq!(matches,
        vec![
        vec![("1".to_string(), "6".to_string()), ("2".to_string(), "5".to_string()), ("3".to_string(), "4".to_string())], 
        vec![("1".to_string(), "5".to_string()), ("6".to_string(), "4".to_string()), ("2".to_string(), "3".to_string())], 
        vec![("1".to_string(), "4".to_string()), ("5".to_string(), "3".to_string()), ("6".to_string(), "2".to_string())], 
        vec![("1".to_string(), "3".to_string()), ("4".to_string(), "2".to_string()), ("5".to_string(), "6".to_string())], 
        vec![("1".to_string(), "2".to_string()), ("3".to_string(), "6".to_string()), ("4".to_string(), "5".to_string())]
        ]
        );
    }
}
