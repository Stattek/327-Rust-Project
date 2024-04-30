mod linked_list;
use linked_list::LinkedList;
use linked_list::{Date, VideoGame};
use std::io::stdin;

fn main() {
    let mut continue_running = true;
    let mut the_list = LinkedList::new(None);

    while continue_running {
        print_menu();

        let mut choice_string = String::new();
        stdin()
            .read_line(&mut choice_string)
            .expect("Error in reading choice.");
        // parse the choice
        let choice = choice_string.trim().parse::<i32>().unwrap();

        match choice {
            1 => add_new_entry(&mut the_list),
            2 => find_video_game(&the_list),
            3 => find_first_game(&the_list),
            4 => check_empty(&the_list),
            5 => print_list(&the_list),
            6 => continue_running = false,
            _ => println!("Invalid choice."),
        };
    }
}

fn print_menu() {
    println!("\n1. Add new entry\n2. Find Video Game\n3. Find First Game From Game Publisher in List\n4. Check if Empty\n5. Print List\n6. Quit");
}

fn add_new_entry(list: &mut LinkedList) {
    let the_game = get_video_game_from_user();

    list.push_back(the_game);

    println!("\n\nEntry added.");
}

fn find_video_game(list: &LinkedList) {
    let game_name = get_game_name_from_user();

    let index = list.find(game_name);
    if let Some(the_index) = index {
        // print new line
        println!();
        // print the game
        let the_game = list.print_at(&the_index);
    }
}

fn find_first_game(list: &LinkedList) {
    let publisher_name = get_publisher_name_from_user();

    let index = list.find_publisher(publisher_name);
    if let Some(the_index) = index {
        let the_game = list.print_at(&the_index);
    }
}

fn check_empty(list: &LinkedList) {
    let result = list.is_empty();

    if result {
        println!("List is empty.");
    } else {
        println!("List is not empty.");
    }
}

fn print_list(list: &LinkedList) {
    println!("List contents:\n");
    list.print_list();
    println!("\nEnd of list.");
}

fn get_game_name_from_user() -> String {
    println!("\nEnter the game name:");

    let mut game_name = String::new();
    stdin()
        .read_line(&mut game_name)
        .expect("Invalid game name input.");

    game_name
}

fn get_publisher_name_from_user() -> String {
    println!("\nEnter the game's publisher:");

    let mut publisher_name = String::new();
    stdin()
        .read_line(&mut publisher_name)
        .expect("Invalid publisher name input.");

    publisher_name
}

fn get_date_from_user() -> Date {
    println!("\nEnter the day:");

    let mut day_string = String::new();
    stdin().read_line(&mut day_string);

    let mut day: u8 = day_string.trim().parse().unwrap();

    while day < 1 || day > 31 {
        println!("Invalid day input");

        println!("\nEnter the day");

        day_string = String::new();
        stdin().read_line(&mut day_string);

        day = day_string.trim().parse().unwrap();
    }

    println!("\nEnter the month:");

    let mut month_string = String::new();
    stdin().read_line(&mut month_string);

    let mut month: u8 = month_string.trim().parse().unwrap();

    while month < 1 || month > 12 {
        println!("Invalid month input");

        println!("\nEnter the month");

        month_string = String::new();
        stdin().read_line(&mut month_string);

        month = month_string.trim().parse().unwrap();
    }

    println!("\nEnter the year:");

    let mut year_string = String::new();
    stdin().read_line(&mut year_string);

    let mut year: u16 = year_string.trim().parse().unwrap();

    Date::new(day, month, year)
}

fn get_video_game_from_user() -> VideoGame {
    let game_name = get_game_name_from_user();
    let publisher_name = get_publisher_name_from_user();
    let date = get_date_from_user();

    VideoGame::new(game_name, publisher_name, date)
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
