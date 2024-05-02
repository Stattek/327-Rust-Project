// Basic program that allows the user to look at data on video games
// and add to the list
// By David Slay
mod linked_list;
use linked_list::LinkedList;
use linked_list::{Date, VideoGame};
use std::io::stdin;

fn main() {
    let mut continue_running = true;
    let mut the_list = LinkedList::new(None);

    // while the user wants to continue running the program
    while continue_running {
        print_menu();

        let mut choice_string = String::new();

        // take user input
        stdin()
            .read_line(&mut choice_string)
            .expect("Error in reading choice.");

        // parse the choice for an i32
        let choice = choice_string.trim().parse::<i32>().unwrap();

        // match our choice to what we want to do
        match choice {
            1 => add_new_entry(&mut the_list),
            2 => find_video_game(&the_list),
            3 => find_first_game(&the_list),
            4 => find_newest_game(&the_list),
            5 => check_empty(&the_list),
            6 => print_list(&the_list),
            7 => empty_list(&mut the_list),
            8 => load_test_data(&mut the_list),
            9 => continue_running = false,
            _ => println!("Invalid choice."),
        };
    }
}

/*
Prints the menu for the program.
 */
fn print_menu() {
    println!("\n1. Add new entry\n2. Find Video Game\n3. Find First Game From Game Publisher in List\n4. Find Newest Game in the List\n5. Check if Empty\n6. Print List\n7. Empty the List\n8. Load Test Data\n9. Quit");
}
/*
Adds a new entry to the LinkedList passed.
 */
fn add_new_entry(list: &mut LinkedList) {
    let the_game = get_video_game_from_user();

    list.push_back(the_game);

    // prompt the user that it was added
    println!("\nEntry added.");
}

/*
Finds a game from the user's input.
Finds the game based off of name.

Uses a reference to a LinkedList object
 */
fn find_video_game(list: &LinkedList) {
    let game_name = get_game_name_from_user();

    // find the video game
    if let Some(index) = list.find(game_name) {
        // print new line
        println!();
        // print the game
        list.print_at(&index);
    } else {
        // fails to find game
        println!("No Game Found.")
    }
}

/*
Finds the first game in the list with the publisher name input by the user.

Uses a reference to a LinkedList
*/
fn find_first_game(list: &LinkedList) {
    let publisher_name = get_publisher_name_from_user();

    let index = list.find_publisher(publisher_name);
    if let Some(the_index) = index {
        list.print_at(&the_index);
    }
}
/*
Finds the newest game in the list and prints it.

Uses a reference to the LinkedList.
*/
fn find_newest_game(list: &LinkedList) {
    if let Some(index) = list.find_newest_game() {
        println!("Newest Game:");
        // print the list
        list.print_at(&index);
    } else {
        println!("No game found.");
    }
}

/*
Checks to see if the list is empty.
Prints that it is empty if there are no elements and prints that is it not empty otherwise.

Uses a reference to the LinkedList.
*/
fn check_empty(list: &LinkedList) {
    let result = list.is_empty();

    // match the result and print
    match result {
        true => println!("List is empty."),
        false => println!("List is not empty."),
    }
}

/*
Prints all of the contents of the list.

Uses a reference to the LinkedList.
*/
fn print_list(list: &LinkedList) {
    println!("List contents:\n");
    list.print_list();
    println!("\nEnd of list.");
}

/*
Empties the list passed by creating a new, empty list.

Uses a reference to the LinkedList.
*/

fn empty_list(list: &mut LinkedList) {
    *list = LinkedList::new(None);

    println!("List Emptied.");
}

/*
Prompts the user for a game name and returns the name the user input.
*/

fn get_game_name_from_user() -> String {
    println!("\nEnter the game name:");

    let mut game_name = String::new();
    stdin()
        .read_line(&mut game_name)
        .expect("Invalid game name input.");

    // trims the name before returning
    game_name.trim().to_string()
}

/*
Prompts the user for and returns the publisher name.
*/
fn get_publisher_name_from_user() -> String {
    println!("\nEnter the game's publisher:");

    let mut publisher_name = String::new();
    stdin()
        .read_line(&mut publisher_name)
        .expect("Invalid publisher name input.");

    // trims the name before returning
    publisher_name.trim().to_string()
}

/*
Prompts the user for a date and returns a Date object with the information passed.
*/
fn get_date_from_user() -> Date {
    println!("\nEnter the day:");

    let mut day_string = String::new();
    stdin()
        .read_line(&mut day_string)
        .expect("Invalid day input.");

    let mut day: u8 = day_string.trim().parse().unwrap();

    while day < 1 || day > 31 {
        println!("Invalid day input");

        println!("\nEnter the day");

        day_string = String::new();
        stdin()
            .read_line(&mut day_string)
            .expect("Invalid day input.");

        day = day_string.trim().parse().unwrap();
    }

    println!("\nEnter the month:");

    let mut month_string = String::new();
    stdin()
        .read_line(&mut month_string)
        .expect("Invalid month input.");

    let mut month: u8 = month_string.trim().parse().unwrap();

    while month < 1 || month > 12 {
        println!("Invalid month input");

        println!("\nEnter the month");

        month_string = String::new();
        stdin()
            .read_line(&mut month_string)
            .expect("Invalid month input.");

        month = month_string.trim().parse().unwrap();
    }

    println!("\nEnter the year:");

    let mut year_string = String::new();
    stdin()
        .read_line(&mut year_string)
        .expect("Invalid year input.");

    let year: u16 = year_string.trim().parse().unwrap();

    Date::new(day, month, year)
}

/*
Prompts the user for information to create a VideoGame object and returns that object
with that data.
 */
fn get_video_game_from_user() -> VideoGame {
    let game_name = get_game_name_from_user();
    let publisher_name = get_publisher_name_from_user();
    let date = get_date_from_user();

    VideoGame::new(game_name, publisher_name, date)
}

/*
Creates a new list and loads in test data for the user to
use.
 */
fn load_test_data(list: &mut LinkedList) {
    // make this list be a new linked list
    *list = LinkedList::new(Some(VideoGame::new(
        String::from("Donkey Kong"),
        String::from("Nintendo"),
        Date::new(31, 7, 1981),
    )));

    list.push_back(VideoGame::new(
        String::from("Mario Bros."),
        String::from("Nintendo"),
        Date::new(1, 6, 1983),
    ));

    list.push_back(VideoGame::new(
        String::from("Super Mario Land 2: 6 Golden Coins"),
        String::from("Nintendo"),
        Date::new(25, 11, 1992),
    ));

    list.push_back(VideoGame::new(
        String::from("Earthbound"),
        String::from("Nintendo"),
        Date::new(5, 6, 1995),
    ));

    list.push_back(VideoGame::new(
        String::from("Chrono Trigger"),
        String::from("Square"),
        Date::new(11, 3, 1995),
    ));

    list.push_back(VideoGame::new(
        String::from("Pikmin"),
        String::from("Nintendo"),
        Date::new(3, 12, 2001),
    ));

    list.push_back(VideoGame::new(
        String::from("Super Mario 64"),
        String::from("Nintendo"),
        Date::new(29, 9, 1996),
    ));

    list.push_back(VideoGame::new(
        String::from("Aion: The Tower of Eternity"),
        String::from("NCSoft"),
        Date::new(25, 11, 2008),
    ));

    list.push_back(VideoGame::new(
        String::from("Logical Journey of the Zoombinis"),
        String::from("Broderbund"),
        Date::new(15, 3, 1996),
    ));

    list.push_back(VideoGame::new(
        String::from("Team Fortress 2"),
        String::from("Valve"),
        Date::new(10, 10, 2007),
    ));

    list.push_back(VideoGame::new(
        String::from("Portal"),
        String::from("Valve"),
        Date::new(10, 10, 2007),
    ));

    println!("Test data loaded.");
}
