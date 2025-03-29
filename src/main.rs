use rand::Rng;
use rand::prelude::IndexedRandom;
use rand::rngs::ThreadRng;
use std::env;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
// Please don't run `cargo fmt` or this neat block gets ruined.
const CONSONANTS: [char; 20] = [
    'b', 'c', 'd', 'f', 'g',
    'h', 'j', 'k', 'l', 'm',
    'n', 'p', 'q', 'r', 's',
    't', 'v', 'w', 'x', 'z',
];

fn main() {
    let mut rng = rand::rng();
    let length: usize = env::args()
        .nth(1)
        .unwrap_or("default".to_string())
        .parse()
        .unwrap_or_else(|_| {
            // 3 through 9 characters with a bell curve.
            rng.random_range(1..=3) + rng.random_range(1..=3) + rng.random_range(1..=3)
        });
    let mut output = String::with_capacity(length);

    output.push(random_letter(&mut rng));
    output.push(random_letter(&mut rng));

    for i in 2..length {
        let last_two = &output[i - 2..i];

        output.push(if last_two.chars().all(|c| VOWELS.contains(&c)) {
            random_consonant(&mut rng)
        } else if last_two.chars().all(|c| CONSONANTS.contains(&c)) {
            random_vowel(&mut rng)
        } else {
            random_letter(&mut rng)
        });
    }

    println!("{}", output);
}

fn random_letter(rng: &mut ThreadRng) -> char {
    if rng.random_bool(0.5) {
        random_vowel(rng)
    } else {
        random_consonant(rng)
    }
}

fn random_vowel(rng: &mut ThreadRng) -> char {
    *VOWELS.choose(rng).unwrap()
}

fn random_consonant(rng: &mut ThreadRng) -> char {
    *CONSONANTS.choose(rng).unwrap()
}
