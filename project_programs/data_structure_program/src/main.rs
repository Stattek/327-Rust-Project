use std::{str::FromStr, vec};

struct VideoGame {
    name: String,
    genre: String,
}

// impl VideoGame {
//     fn get_game_name(&self) -> String {
//         String::from(self.name)
//     }
// }

struct Library {
    // keep a list of games
    games: Vec<VideoGame>,
}

// impl Library {
//     fn findGame(&self, game_name: String) -> usize {
//         let mut i = 0;

//         while self.games[i].get_game_name() != game_name {
//             i += 1;
//         }

//         return i;
//     }
// }

fn main() {
    let mut my_vec: Vec<String> = Vec::new();
    my_vec.push("hello".to_string());
    my_vec.push("world".to_string());
    my_vec.push("this".to_string());
    my_vec.push("is".to_string());
    my_vec.push("a".to_string());
    my_vec.push("test".to_string());

    for string in my_vec {
        println!("{}", string)
    }
}
