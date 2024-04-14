#[derive(Debug)]
struct Piece {
    composer: String,
    title: String,
}

#[derive(Debug)]
struct Performance {
    date: String,
    venue: String,
    title: String,
    // performers
    pieces: Vec<Piece>,
}


fn main () {
    let performance = Performance {
        date: "2024-04-08".to_string(),
        venue: "Wigmore Hall".to_string(),
        title: "Brahms".to_string(),
        pieces: vec![
            Piece {
                composer: "Johannes Brahms".to_string(),
                title: "Violin Sonata No. 1".to_string(),
            },
            Piece {
                composer: "Johannes Brahms".to_string(),
                title: "Violin Sonata No. 2".to_string(),
            },
            Piece {
                composer: "Johannes Brahms".to_string(),
                title: "Violin Sonata No. 3".to_string(),
            },
        ],
    };

    println!("{:?}", performance);
}
