use std::fs;
use rand::seq::SliceRandom;
use colored::*;
use lazy_static::*;
use std::sync::Mutex;

struct Sanuli {
    word: String,
    chars: Vec<char>,
    guessed: Vec<String>,
    tries: u8,
}

impl Sanuli{
    fn get_word(&self) -> String {
        self.word.clone()
    }
    fn get_chars(&self) -> Vec<char> {
        self.chars.clone()
    }

    fn get_guessed(&self) -> Vec<String> {
        self.guessed.clone()
    }

    fn add_guessed(&mut self, guess: &str) {
        self.guessed.push(guess.to_string());
    }

    fn increment_tries(&mut self) {
        self.tries -= 1;
    }
}


lazy_static! {
    static ref WORDS: Mutex<Vec<String>> = Mutex::new(vec![]);
}


fn main() {
    println!("Sanuli v0.1");
    WORDS.lock().unwrap().extend(read_words("./full.txt"));

    println!("Found {} words.",WORDS.lock().unwrap().len());
    println!("---------------");


    let word: String = WORDS.lock().unwrap().choose_multiple(&mut rand::thread_rng(), 1).cloned().collect();
    let chars = word.chars().collect();

    let mut game = Sanuli {
        word: word,
        chars: chars,
        guessed: Vec::new(),
        tries: 5
    };

    render_game(game.get_guessed(), game.get_chars());
    let mut input = get_input(game.get_word());

    loop {

        
        if input.to_uppercase().trim() == game.get_word() {
            game.add_guessed(input.to_uppercase().trim());
            render_game(game.get_guessed(), game.get_chars());
            println!("You Guessed the right word!");
            break;
        }

        if game.tries <= 0{
            println!("Failed to guess the word. Correct word was '{}'", game.get_word());
            break;
        }

        game.add_guessed(input.to_uppercase().trim());

        game.increment_tries();

        render_game(game.get_guessed(), game.get_chars());
        input = get_input(game.get_word());
    }
    
}

fn get_input(word: String) -> String {

    println!("\nEnter your guess:");

    let mut guess = String::new();

    loop {

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your guess.");

        let parsed = guess.trim();

        if guess.chars().count() - 2 == word.chars().count() {

            if !WORDS.lock().unwrap().contains(&parsed.to_uppercase()) {
                println!("That is not a word!");
                guess = String::new();
            }
            else{
                break;
            }

        }

        else{
            println!("Invalid guess");
            guess = String::new();
        }

    }

    guess
}

fn render_game(guessed: Vec<String>, chars: Vec<char>) {
    // Render the current Game
    for _ in 0..chars.len() {
        print!(" _");
    }
    print!("\n");
    
    for i in 0..6 {
        print!("|");

        if guessed.len() > i {
            for j in 0..chars.len() {
                let guessed_char = guessed[i].chars().nth(j).unwrap();

                if chars.contains(&guessed_char) {

                    if chars[j] == guessed_char {
                        print!("{}|", guessed_char.to_string().green())
                    }
                    else{
                        print!("{}|", guessed_char.to_string().yellow())
                    }
                }
                else{
                    print!("{}|", guessed_char.to_string())
                }

            }
            print!("\n");
        }
        else{
            for _ in 0..chars.len() {
                print!("_|");
            }
            print!("\n");
        }
    }
}

fn read_words(path: &str) -> Vec<String> {

    let contents = fs::read_to_string(path.to_string())
        .expect("Couldn't read the file.");

    contents.lines().map(str::to_string).collect()

}
