struct Book {
    title: String,
    author: String,
    year_of_publication: u32,
}

fn main() {
    let mut my_book = Book {
        title: String::from("The Great Gatsby"),
        author: String::from("F. Scott Fitzgerald"),
        year_of_publication: 1925,
    };

    show_book_details(&my_book);

    update_year(&mut my_book, 1800);

    show_book_details(&my_book);

    if is_published_after(&my_book, 1900) {
        println!("The book was published after 1900.");
    } else {
        println!("The book was published before 1900.");
    }

    let first_word = first_word(&my_book.title);
    println!("First word of the title: {}", first_word);
}

fn show_book_details(book: &Book) {
    println!("Title: {}", book.title);
    println!("Author: {}", book.author);
    println!("Year: {}", book.year_of_publication);
}

fn is_published_after(book: &Book, year: u32) -> bool {
    book.year_of_publication > year
}

fn update_year(book: &mut Book, new_year: u32) {
    book.year_of_publication = new_year;
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
