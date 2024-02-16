// Yazar struct tanımı
struct Author {
    name: String,
    surname: String,
}

// Publication enum tanımı
enum Publication {
    Book {
        title: String,
        author: Author,
        page_count: u32,
        price: f32,
        publisher: String,
        isbn: String,
    },
    Magazine {
        title: String,
        issue: u32,
        topic: String,
        price: f32,
        publisher: String,
        isbn: String,
    },
}

// Publication enum için özel bir yazdırma fonksiyonu
impl Publication {
    fn display(&self) {
        match self {
            Publication::Book {
                title,
                author,
                page_count,
                price,
                publisher,
                isbn,
            } => {
                println!("Kitap: {} - Yazar: {} {} - Sayfa Sayısı: {} - Fiyat: {} - Yayıncı: {} - ISBN: {}", 
                    title, author.name, author.surname, page_count, price, publisher, isbn);
            }
            Publication::Magazine {
                title,
                issue,
                topic,
                price,
                publisher,
                isbn,
            } => {
                println!("Dergi: {} - Sayı: {} - Konu: {} - Fiyat: {} - Yayıncı: {} - ISBN: {}", 
                    title, issue, topic, price, publisher, isbn);
            }
        }
    }
}

fn main() {
    // Yazar bilgileri
    let steve_klabnik = Author {
        name: String::from("Steve"),
        surname: String::from("Klabnik"),
    };

    // Kitap ve dergi örneklerinin oluşturulması
    let publications: Vec<Publication> = vec![
        Publication::Book {
            title: String::from("The Rust Programming Language"),
            author: steve_klabnik,
            page_count: 519,
            price: 39.99,
            publisher: String::from("No Starch Press"),
            isbn: String::from("978-1593278281"),
        },
        Publication::Magazine {
            title: String::from("National Geographic"),
            issue: 238,
            topic: String::from("Wildlife Conservation"),
            price: 8.99,
            publisher: String::from("National Geographic Partners"),
            isbn: String::from("NGMAG238"),
        },
    ];

    // Tüm yayınların listelenmesi
    for publication in &publications {
        publication.display();
    }
}
