use nanoid::nanoid;

use rand::distr::StandardUniform;
use rand::{rng, Rng};

fn random(size: usize) -> Vec<u8> {
    let rng = rng();

    rng.sample_iter(&StandardUniform).take(size).collect()
}

fn main() {
    nanoid!(10, &['a', 'b', 'c', 'd', 'e', 'f'], random); //=> "fbaefaadeb"
}
