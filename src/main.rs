use std::fs;
use rand::seq::SliceRandom;
use colored::*;

struct Sanuli {
    word: String,
    chars: Vec<char>,
    guessed: Vec<String>,
    used_chars: Vec<char>,
    tries: u8,
}

impl Sanuli{
    fn get_word(&self) -> String {
        self.word.clone()
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

fn main() {
    let words = read_words("./full.txt");
    println!("Found {} words.", words.len());


    
    let word: String = words.choose_multiple(&mut rand::thread_rng(), 1).cloned().collect();
    let chars = word.chars().collect();

    let mut game = Sanuli {
        word: word,
        chars: chars,
        guessed: Vec::new(),
        used_chars: Vec::new(),
        tries: 5
    };

    println!("{:?}", game.chars);

    render_game(game.get_guessed());
    let mut input = get_input();

    loop {

        
        if input.to_uppercase().trim() == game.get_word() {
            game.add_guessed(input.to_uppercase().trim());
            render_game(game.get_guessed());
            println!("You won!");
            break;
        }

        if game.tries <= 0{
            println!("Failed to guess the word. Correct word was '{}'", game.get_word());
            break;
        }

        game.add_guessed(input.to_uppercase().trim());

        game.increment_tries();

        render_game(game.get_guessed());
        input = get_input();
    }
    
}

fn get_input() -> String {

    println!("\nEnter your guess:");

    let mut guess = String::new();

    loop {

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your guess.");

        if guess.chars().count() == 7 {
            break;
        }
        else{
            println!("Invalid guess: {}", guess.chars().count());
            guess = String::new();
        }

    }

    guess
}

fn render_game(guessed: Vec<String>) {
    // Render the current Game
    println!(" _ _ _ _ _ ");
    for i in 0..6 {

        if guessed.len() > i {
            println!("|{}|{}|{}|{}|{}|", 
            guessed[i].chars().nth(0).unwrap(),
            guessed[i].chars().nth(1).unwrap(),
            guessed[i].chars().nth(2).unwrap(),
            guessed[i].chars().nth(3).unwrap(),
            guessed[i].chars().nth(4).unwrap(), );
        }
        else{
            println!("|_|_|_|_|_|")
        }
    }
}

fn read_words(path: &str) -> Vec<String> {

    let contents = fs::read_to_string(path.to_string())
        .expect("Couldn't read the file.");

    contents.lines().map(str::to_string).collect()

}
