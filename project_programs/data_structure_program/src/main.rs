mod linked_list;
use linked_list::{Date, VideoGame};

use crate::linked_list::LinkedList;

fn main() {
    let mut my_list = get_starting_list();

    if let Some(index) = my_list.find(String::from("Pikmin")) {
        println!("{}", index);
    } else {
        println!("Game not found.");
    }

    if let Some(index) = my_list.find_publisher(String::from("not a publisher")) {
        println!("{}", index);
    } else {
        println!("Publisher not found.");
    }

    println!("{}", my_list.find_newest_game());
}

fn get_starting_list() -> LinkedList {
    let mut the_list = LinkedList::new(Some(VideoGame::new(
        String::from("Donkey Kong"),
        String::from("Nintendo"),
        Date::new(31, 7, 1981),
    )));

    the_list.push_back(VideoGame::new(
        String::from("Mario Bros."),
        String::from("Nintendo"),
        Date::new(1, 6, 1983),
    ));

    the_list.push_back(VideoGame::new(
        String::from("Super Mario Land 2: 6 Golden Coins"),
        String::from("Nintendo"),
        Date::new(25, 11, 1992),
    ));

    the_list.push_back(VideoGame::new(
        String::from("Earthbound"),
        String::from("Nintendo"),
        Date::new(5, 6, 1995),
    ));

    the_list.push_back(VideoGame::new(
        String::from("Chrono Trigger"),
        String::from("Square"),
        Date::new(11, 3, 1995),
    ));

    the_list.push_back(VideoGame::new(
        String::from("Pikmin"),
        String::from("Nintendo"),
        Date::new(3, 12, 2001),
    ));

    the_list.push_back(VideoGame::new(
        String::from("Super Mario 64"),
        String::from("Nintendo"),
        Date::new(29, 9, 1996),
    ));

    the_list.push_back(VideoGame::new(
        String::from("Aion: The Tower of Eternity"),
        String::from("NCSoft"),
        Date::new(25, 11, 2008),
    ));

    the_list.push_back(VideoGame::new(
        String::from("Logical Journey of the Zoombinis"),
        String::from("Broderbund"),
        Date::new(15, 3, 1996),
    ));

    the_list.push_back(VideoGame::new(
        String::from("Team Fortress 2"),
        String::from("Valve"),
        Date::new(10, 10, 2007),
    ));

    the_list.push_back(VideoGame::new(
        String::from("Portal"),
        String::from("Valve"),
        Date::new(10, 10, 2007),
    ));

    the_list
}
