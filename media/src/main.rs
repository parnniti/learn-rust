mod content;
use content::{catalog::Catalog, media::Media};

fn main() {
    let audiobook = Media::Audiobook {
        title: "An audiobook".to_string(),
    };

    let good_movie = Media::Movie {
        title: "A good movie".to_string(),
        director: "A good director".to_string(),
    };

    let bad_book = Media::Book {
        title: "A bad book".to_string(),
        author: "A bad author".to_string(),
    };
    
    let podcast = Media::Podcast(42);
    let placeholder = Media::Placeholder {};

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    let placeholder = Media::Placeholder;

    println!("{:#?}", catalog.items.get(88).unwrap_or(&placeholder));
}
