/*
    
You are building a simple library system that keeps track of books and user actions.

Create a struct Book with:

title (a String)
author (a String)
available (a bool indicating if the book is available)
Create an enum UserAction representing actions a user can take:

Borrow(String): Borrow a book by its title.
Return(String): Return a borrowed book.
ListAvailable: List all books that are currently available.
Implement a function handle_action() that:

Takes a UserAction and a list of books.
Updates the availability of books accordingly.
Prints relevant messages based on the action.
Example Scenarios
If a user borrows a book, it should be marked as unavailable.
If a user returns a book, it should be marked as available again.
If a user requests to list available books, print only books that are not borrowed.
Bonus Challenge (Optional)
Prevent borrowing a book that’s already checked out.
Handle cases where a user tries to return a book that wasn’t borrowed.



*/
use std::io;

#[derive(Debug)]
struct Book {
    title: String,
    author : String,
    available: bool
}

enum UserAction {
    Borrow(String),
    Return(String),
    ListAvailable
}

impl UserAction {
    fn userOps(&self,book: &mut [Book]) {
        match self {
            UserAction::ListAvailable => {
                for item in book {
                    if item.available {
                        println!("{:?}", item.title);
                    }
                }
            }
            UserAction::Borrow(t) => {
                for item in book {
                    if item.title == *t && item.available {
                        item.available = false;
                        println!("Item Borrowed");
                        break;
                    } else if item.title == *t && !item.available{
                        println!("Item was already borrowed");
                        break;
                    }else {
                        println!("Book Not Found");
                        break;
                    }

                }
            }
            UserAction::Return(t) => {
                for item in book {
                    if item.title == *t && !item.available {
                        item.available = true;
                        println!("Item Returned");
                        break;
                    }else if item.title == *t && item.available{
                        println!("Item is in inventory");
                        break;
                    } else {
                        println!("Book Does Not Exist");
                        break;               
                    }
                }
            }
        }
    }
}



fn main() {
    let mut arr =[Book{title:"blah1".to_string(),author:"blah1a".to_string(),available: true},Book{title:"blah2".to_string(),author:"blah2a".to_string(),available: true},Book{title:"blah3".to_string(),author:"blah3a".to_string(),available: true}];

    let mut str = String::new();
    let mut str2 = String::new();

    loop {
        println!("1. Borrow\n2. Return\n3. List");
        io::stdin().read_line(&mut str).expect("Error Reading Message");
        if str.trim() == "end" {
            return;
        } else {
            match str.trim() {
            "1" => {
                println!("Please Input the name of the book: ");
                io::stdin().read_line(&mut str2).expect("Error Reading Message");
                UserAction::Borrow(str2.trim().to_string()).userOps(&mut arr);
            }
            "2" => {
                println!("Please Input the name of the book: ");
                io::stdin().read_line(&mut str2).expect("Error Reading Message");
                UserAction::Return(str2.trim().to_string()).userOps(&mut arr);
            }
            "3" => {
                UserAction::ListAvailable.userOps(&mut arr);
            }
            _=> println!("Please choose from 1 2 or 3")
           }
        }
        str.clear();
        str2.clear();
    }
}