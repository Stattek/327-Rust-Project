// Simple linked list implementation
// By David Slay

/*
Simple struct to hold date information
 */
pub struct Date {
    day: u8,
    month: u8,
    year: u16,
}

impl Date {
    // constructor to create a new Date object
    pub fn new(day: u8, month: u8, year: u16) -> Self {
        match day {
            1..=31 => day,
            _ => panic!("Invalid date given to Date object"),
        };

        match month {
            1..=12 => month,
            _ => panic!("Invalid month given to Date object"),
        };

        Self { day, month, year }
    }

    // checks if the rhs is newer than this instance
    pub fn newer_than(&self, rhs: &Date) -> bool {
        let mut result = false;

        if self.year > rhs.year {
            // if this year is later than rhs, then this is false
            result = true;
        } else if self.month > rhs.month && self.year == rhs.year {
            // if this month is later than rhs, then this is false
            result = true;
        } else if self.day > rhs.day && self.year == rhs.year && self.month == rhs.month {
            // if this day is later than rhs, then this is false
            result = true;
        }

        result
    }
}

/*
Stores information about a video game
 */
pub struct VideoGame {
    name: String,
    // a non-negative rating system
    publisher: String,
    release_date: Date,
}

impl VideoGame {
    //constructor to make a new video game
    pub fn new(name: String, publisher: String, date: Date) -> Self {
        Self {
            name,
            publisher,
            release_date: date,
        }
    }

    // prints the video game to console
    pub fn print_self(&self) {
        println!(
            "Video Game Name: {}\n\tPublisher: {}\n\tRelease date: {}/{}/{}",
            self.name,
            self.publisher,
            self.release_date.month,
            self.release_date.day,
            self.release_date.year
        );
    }
}

/*
Node for the LinkedList class.
Holds a VideoGame.
 */
pub struct Node {
    // stores its next node on the heap
    // the next node can exist or not exist, which is why it is an Option
    next: Option<Box<Node>>,
    pub data: VideoGame,
}

impl Node {
    // constructor for a node
    pub fn new(data: VideoGame) -> Self {
        Self {
            //gives next a default value of None
            next: None,
            // makes data a box
            data: data,
        }
    }
}

/*
Struct that holds a singly-linked list.
*/
pub struct LinkedList {
    head: Option<Box<Node>>,
    size: i32,
}

impl LinkedList {
    // create a new LinkedList with an optional first piece of data
    pub fn new(data: Option<VideoGame>) -> Self {
        // create a linked list with one node
        match data {
            Some(data) => Self {
                head: Some(Box::new(Node::new(data))),
                size: 1,
            },
            None => Self {
                head: None,
                size: 0,
            },
        }
    }

    // pushes a new VideoGame to the end of the list. Note: takes ownership of the VideoGame
    pub fn push_back(&mut self, data: VideoGame) {
        // get the current node in the list, starting at the head

        //get a mutable reference to the head
        let mut cur_node = self.head.as_mut();

        // find the node we want to add
        for _i in 0..self.size - 1 {
            // increment the head
            if let Some(next_node) = cur_node {
                cur_node = next_node.next.as_mut();
            }
        }

        if let Some(node) = cur_node {
            // add the element to the end of the list
            node.next = Some(Box::new(Node::new(data)));
        } else {
            self.head = Some(Box::new(Node::new(data)));
        }

        self.size += 1;
    }

    // returns a reference to the node on the heap at this index
    pub fn at(&self, index: &i32) -> Option<&Box<Node>> {
        if *index < self.size {
            //get a mutable reference to the head
            let mut cur_node = self.head.as_ref();

            // find the node we want to add
            for _i in 0..*index {
                // increment the head
                if let Some(next_node) = cur_node {
                    cur_node = next_node.next.as_ref();
                }
            }

            return cur_node;
        } else {
            return None;
        }
    }

    pub fn print_at(&self, index: &i32) {
        if let Some(the_node) = self.at(index) {
            the_node.data.print_self();
        }
    }

    /*
    Finds a video game in the list.
    If it exists, then returns the index of the item
    If it does not exist, then it returns None.
    */
    pub fn find(&self, video_game_name: String) -> Option<i32> {
        // start at the head of the list
        let mut cur_node = self.head.as_ref();
        let mut index = 0;

        if let Some(mut the_node) = cur_node {
            while the_node.data.name != video_game_name && index < self.size {
                cur_node = the_node.next.as_ref();
                index += 1;

                // move the node up
                if cur_node.is_some() {
                    the_node = cur_node.unwrap();
                }
            }

            if the_node.data.name == video_game_name {
                return Some(index);
            }
        }

        None
    }

    /*
    Finds the first Node that contains
    a game with a publisher of the same name as the one passed.
     */
    pub fn find_publisher(&self, publisher_name: String) -> Option<i32> {
        // start at the head of the list
        let mut cur_node = self.head.as_ref();
        let mut index = 0;

        if let Some(mut the_node) = cur_node {
            while the_node.data.publisher != publisher_name && index < self.size {
                cur_node = the_node.next.as_ref();
                index += 1;

                // move the node up
                if cur_node.is_some() {
                    the_node = cur_node.unwrap();
                }
            }

            if the_node.data.publisher == publisher_name {
                return Some(index);
            }
        }

        None
    }

    // finds and returns the newest game in the list
    pub fn find_newest_game(&self) -> Option<i32> {
        // start at the head of the list
        if self.head.is_some() {
            let mut index = 0;
            let mut min_index = 0;

            // start with the head node as the minimum
            let mut min_node = self.head.as_ref();
            let mut cur_node = self.head.as_ref();

            while cur_node.is_some() && cur_node.unwrap().next.is_some() {
                cur_node = cur_node.unwrap().next.as_ref();
                index += 1;

                if cur_node
                    .unwrap()
                    .data
                    .release_date
                    .newer_than(&min_node.unwrap().data.release_date)
                {
                    min_node = cur_node;
                    min_index = index;
                }
            }

            // return the index of the minimum element
            return Some(min_index);
        }

        None
    }

    // return true if the list is empty, false otherwise
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // prints the list's contents
    pub fn print_list(&self) {
        let mut cur_node = self.head.as_ref();

        // go through the linked list
        for _ in 0..self.size {
            // if we have some value, then print it
            if let Some(the_node) = &cur_node {
                the_node.data.print_self();

                // move the current node forward
                cur_node = the_node.next.as_ref();
            }
        }
    }
}
