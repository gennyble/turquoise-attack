use std::collections::HashSet;

use petname::Petnames;
use rand::{prelude::ThreadRng, rngs::SmallRng, Rng, SeedableRng};

struct RandomData {
    artists: Vec<String>,
    albums: Vec<String>,
    tracks: Vec<String>,
}

impl RandomData {
    /// `artist_count` unique artists. `album_count` unique artists. `track_count` is not unique across artists
    pub fn new(artist_count: usize, album_count: usize, track_count: usize) -> Self {
        let mut rng = SmallRng::from_entropy();

        Self {
            artists: Self::random_vec(&mut rng, artist_count, 3),
            albums: Self::random_vec(&mut rng, album_count, 3),
            tracks: Self::random_vec(&mut rng, track_count, 2),
        }
    }

    fn random_vec<R: Rng>(rng: &mut R, mut count: usize, words: u8) -> Vec<String> {
        let mut strings = HashSet::new();

        let names = Petnames::default();
        let cardinality = names.cardinality(words);

        if cardinality < count as u128 {
            eprintln!("Not enough words to form all unique names! This will likely loop forever")
        }

        while count > 0 {
            let name = names.generate(rng, words, " ");

            if strings.insert(name) {
                count -= 1;
            }
        }

        strings
            .drain()
            .map(|s| titlecase::titlecase(s.as_str()))
            .collect()
    }

    pub fn random_scrobble(&self) -> Scrobble {
        let mut rng = ThreadRng::default();
        let artist = &self.artists[rng.gen_range(0..self.artists.len())];
        let album = &self.albums[rng.gen_range(0..self.albums.len())];
        let track = &self.tracks[rng.gen_range(0..self.tracks.len())];

        Scrobble {
            artist,
            album,
            track,
        }
    }
}

struct Scrobble<'a> {
    artist: &'a str,
    album: &'a str,
    track: &'a str,
}

fn main() {
    let data = RandomData::new(50, 50, 12);

    for _ in 0..4 {
        let scrob = data.random_scrobble();

        println!("{} on {} by {}", scrob.track, scrob.album, scrob.artist);
    }
}
