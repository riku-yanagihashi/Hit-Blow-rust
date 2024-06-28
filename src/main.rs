extern crate rand;
use rand::Rng;

struct Answer {
    answer: Vec<u8>,
}

impl Answer {
    fn new(rng: &mut ThreadRng) -> Self {
        let mut ans = vec![];
        for _ in 0..4 {
            ans.push(rng.gen_range(0..10));
        }
        return Self { answer: ans };
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let answer = Answer::new(&mut rng);
}
