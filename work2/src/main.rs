fn main() {
    let book1 = Book {
        title: "Rust Cookbook Türkçe Özet".to_string(),
        author: "by Me".to_string(),
        page_count: 55,
    };    

    let magazine1 = Magazine {
        title: "Computer Science".to_string(),
        issue: 20,
        topic: "Programming web3".to_string(),
    };

     let publications = vec![Publication::Book(book1), Publication::Magazine(magazine1)];
     printer(publications)
    
}

fn printer(publications: Vec<Publication>) {
    for publication in publications {
      match publication {
        Publication::Book(book) => {
          println!("Kitap: {}, Yazar: {}, Sayfa sayısı: {}", book.title, book.author, book.page_count);
        },
        Publication::Magazine(magazine) => {
          println!("Dergi: {}, Sayı: {}, Konu: {}", magazine.title, magazine.issue, magazine.topic);
         
        },
      }
    }
  }
  

enum Publication {
    Book(Book),
    Magazine(Magazine),
}

struct Book{
    title: String,
    author: String,
    page_count: i32,
}

struct Magazine {
title: String,
issue: i32,
topic: String,
}

