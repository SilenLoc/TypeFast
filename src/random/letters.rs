use rand::prelude::Distribution;
use rand::Rng;

pub fn random_letters_inner(max: u32) -> String {
    let rng = rand::thread_rng();
    let s: String = rng
        .sample_iter(&Alpha)
        .take(max.try_into().unwrap_or_default())
        .map(char::from)
        .collect();
    s.trim().to_owned()
}

struct Alpha;

impl Distribution<u8> for Alpha {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: u32 = 26 + 26 + 1;
        const GEN_ASCII_STR_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                abcdefghijklmnopqrstuvwxyz\
                ";
        loop {
            let var = rng.next_u32() >> (32 - 5);
            if var < RANGE {
                return GEN_ASCII_STR_CHARSET[var as usize];
            }
        }
    }
}
