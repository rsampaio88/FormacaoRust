// ISBN = Book id

#[derive(Debug)]
struct  Book {
    isbn: String,
    title: String,
    author: String,
    keyword: Vec<String>,
    stock: u64,
    borrow: u64,
}

impl Book {

    fn new(isbn: String, title: String, author: String, keyword: Vec<String>) -> Self {
        Book {
            isbn,
            title,
            author,
            keyword,
            stock: stock += 1,
            borrow: 0
        }
    }
}

struct Library {
    books: Vec<Book>,
}




fn library() {
    let mut books: Vec<Book> = Vec::new();

    loop {
        clearscreen::clear().unwrap();
        println!(
            "Library Menu:
1) Add Book
2) Remove Book
3) Request Book
4) Return Book
5) List All Books
6) Search by Author
7) Search by Keyword
8) Exit"
        );

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter book title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");

                println!("Enter book author:");
                let mut author = String::new();
                io::stdin().read_line(&mut author).expect("Failed to read line");

                println!("Enter book ISBN:");
                let mut isbn = String::new();
                io::stdin().read_line(&mut isbn).expect("Failed to read line");

                println!("Enter keywords (comma-separated):");
                let mut keywords_input = String::new();
                io::stdin().read_line(&mut keywords_input).expect("Failed to read line");
                let keywords: Vec<String> = keywords_input
                    .trim()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();

                books.push(Book {
                    title: title.trim().to_string(),
                    author: author.trim().to_string(),
                    isbn: isbn.trim().to_string(),
                    keywords,
                    requested: false,
                });
                println!("Book added successfully!");
                press_to_continue()
            }
            2 => {
                println!("Enter book title to remove:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                let title = title.trim();

                if let Some(pos) = books.iter().position(|b| b.title == title && !b.requested) {
                    books.remove(pos);
                    println!("Book removed successfully!");
                } else {
                    println!("Book not found or it is currently requested.");
                }

                press_to_continue()
            }
            3 => {
                println!("Enter book title to request:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                let title = title.trim();

                if let Some(book) = books.iter_mut().find(|b| b.title == title) {
                    book.requested = true;
                    println!("Book requested successfully!");
                } else {
                    println!("Book not found.");
                }

                press_to_continue()
            }
            4 => {
                println!("Enter book title to return:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                let title = title.trim();

                if let Some(book) = books.iter_mut().find(|b| b.title == title) {
                    book.requested = false;
                    println!("Book returned successfully!");
                } else {
                    println!("Book not found.");
                }

                press_to_continue()
            }
            5 => {
                if books.is_empty() {
                    println!("No books in the library.");
                } else {
                    println!("Listing all books:");
                    for book in &books {
                        println!(
                            "Title: {}, Author: {}, ISBN: {}, Keywords: {:?}, Requested: {}",
                            book.title, book.author, book.isbn, book.keywords, book.requested
                        );
                    }
                }
                press_to_continue()
            }
            6 => {
                println!("Enter author name to search:");
                let mut author = String::new();
                io::stdin().read_line(&mut author).expect("Failed to read line");
                let author = author.trim();

                let results: Vec<&Book> = books.iter().filter(|b| b.author == author).collect();
                if results.is_empty() {
                    println!("No books found by author: {}", author);
                } else {
                    println!("Books by {}:", author);
                    for book in results {
                        println!(
                            "Title: {}, ISBN: {}, Keywords: {:?}, Requested: {}",
                            book.title, book.isbn, book.keywords, book.requested
                        );
                    }
                }
                press_to_continue()
            }
            7 => {
                println!("Enter keyword:");
                let mut keyword = String::new();
                io::stdin().read_line(&mut keyword).expect("Failed to read line");
                let keyword = keyword.trim().to_string();

                println!(
                    "Choose keyword search type:
1) Union (any matching keyword)
2) Intersection (all matching keywords)"
                );
                let mut search_type = String::new();
                io::stdin()
                    .read_line(&mut search_type)
                    .expect("Failed to read line");
                let search_type: u32 = search_type.trim().parse().expect("Invalid number");

                match search_type {
                    1 => {
                        let results: Vec<&Book> = books
                            .iter()
                            .filter(|b| b.keywords.contains(&keyword))
                            .collect();
                        if results.is_empty() {
                            println!("No books found with keyword: {}", keyword);
                        } else {
                            println!("Books with keyword '{}':", keyword);
                            for book in results {
                                println!(
                                    "Title: {}, Author: {}, ISBN: {}, Requested: {}",
                                    book.title, book.author, book.isbn, book.requested
                                );
                            }
                        }
                    }
                    2 => {
                        println!("Enter additional keywords (comma-separated):");
                        let mut additional_keywords = String::new();
                        io::stdin()
                            .read_line(&mut additional_keywords)
                            .expect("Failed to read line");
                        let additional_keywords: Vec<String> = additional_keywords
                            .trim()
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect();

                        let results: Vec<&Book> = books
                            .iter()
                            .filter(|b| {
                                let all_keywords: Vec<String> =
                                    std::iter::once(keyword.clone()).chain(additional_keywords.clone()).collect();
                                all_keywords.iter().all(|k| b.keywords.contains(k))
                            })
                            .collect();

                        if results.is_empty() {
                            println!("No books found matching all keywords.");
                        } else {
                            println!("Books matching all keywords:");
                            for book in results {
                                println!(
                                    "Title: {}, Author: {}, ISBN: {}, Requested: {}",
                                    book.title, book.author, book.isbn, book.requested
                                );
                            }
                        }
                    }
                    _ => println!("Invalid search type."),
                }
                press_to_continue()
            }
            8 => break,
            _ => println!("Invalid option, please try again."),
        }
    }
}


fn main() {
    todo!()
}