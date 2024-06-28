extern crate rand;

use std::fmt;
use std::io;
use std::io::Write;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

struct InvalidLengthError {
    answer: usize,
    guess: usize,
}

impl fmt::Display for InvalidLengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invalid length. answer={}, guess={}",
            self.answer, self.guess
        )
    }
}

struct InvalidDigitError {
    char: char,
}

impl fmt::Display for InvalidDigitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid digit. char={}", self.char)
    }
}

struct CheckResult {
    hit: u8,
    blow: u8,
}

impl CheckResult {
    fn correct(&self) -> bool {
        self.hit == 4 && self.blow == 0
    }
}

impl fmt::Display for CheckResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Result: {}hit, {}blow", self.hit, self.blow)
    }
}

struct Answer {
    answer: Vec<u8>,
}

impl Answer {
    fn new(rng: &mut ThreadRng) -> Self {
        let choices = (0..10).collect::<Vec<u8>>();
        return Self {
            answer: choices.choose_multiple(rng, 4).cloned().collect(),
        };
    }

    fn check(&self, guess: Vec<u8>) -> Result<CheckResult, InvalidLengthError> {
        let mut hit = 0;
        let mut blow = 0;

        if self.answer.len() != guess.len() {
            return Err(InvalidLengthError {
                answer: self.answer.len(),
                guess: guess.len(),
            });
        }

        for (idx, &val) in guess.iter().enumerate() {
            if let Some(ans_idx) = self.answer.iter().position(|&ans| ans == val) {
                if ans_idx == idx {
                    hit += 1;
                } else {
                    blow += 1;
                }
            }
        }

        Ok(CheckResult {
            hit: hit,
            blow: blow,
        })
    }
}

struct Guess {
    guess: Vec<u8>,
}

impl Guess {
    fn new(guess: String) -> Result<Self, InvalidDigitError> {
        let mut guess_vec = vec![];

        for c in guess.trim().chars() {
            match c.to_digit(10) {
                Some(d) => guess_vec.push(d as u8),
                None => return Err(InvalidDigitError { char: c }),
            };
        }

        Ok(Guess { guess: guess_vec })
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let answer = Answer::new(&mut rng);

    loop {
        print!("4桁の数字を入力してください: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("入力エラー。read_line()で失敗しました。");

        let guess = match Guess::new(guess) {
            Ok(guess) => guess,
            Err(e) => {
                println!("{}", e);
                println!("もう一度入力してください。");
                continue;
            }
        };

        let result = match answer.check(guess.guess) {
            Ok(result) => result,
            Err(e) => {
                println!("{}", e);
                println!("もう一度入力してください。");
                continue;
            }
        };

        println!("{}", result);

        if result.correct() {
            println!("Congratulations!!");
            break;
        }
    }
}
