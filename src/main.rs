use macroquad::input::KeyCode;
use macroquad::rand::gen_range;
use macroquad::rand::ChooseRandom;
use macroquad::prelude::{
    draw_text,
    clear_background,
    get_frame_time,
    screen_width,
    screen_height,
    is_key_pressed,
    get_last_key_pressed,
    next_frame,
    WHITE,
    GREEN,
    BLACK,
    YELLOW
};

struct Word {
    text: String,
    x: f32,
    y: f32,
    speed: f32,
}

impl Word {
    fn update(&mut self) {
        self.x -= self.speed;
    }

    fn draw(&self) {
        draw_text(&self.text, self.x, self.y, 30.0, WHITE);
    }

    fn is_off_screen(&self) -> bool {
        self.x + self.text.len() as f32 * 20.0 < 0.0
    }
}

#[macroquad::main("Typing Game")]
async fn main() {
    //let word_list = vec![
    //    "rust", "macroquad", "async", "cargo", "borrow", "trait", "crates", "thread",
    //];

    let mut active_words: Vec<Word> = Vec::new();
    let mut timer = 0.0;
    let mut input = String::new();
    let mut score = 0;

    // Read file
    let word_list = std::fs::read_to_string("arch/english-words-master/words.txt").unwrap();
    let word: Vec<&str> = word_list.lines().collect();

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();
        timer += dt;

        // Spawna nova palavra a cada 2 segundos
        if timer >= 2.0 {
            timer = 0.0;
            let word = word.choose().unwrap().to_string();
            active_words.push(Word {
                text: word,
                x: screen_width(),
                y: gen_range(50.0, screen_height() - 50.0),
                speed: 100.0 * dt,
            });
        }

        // Atualiza e desenha palavras
        for word in &mut active_words {
            word.update();
            word.draw();
        }

        // Remove palavras que saíram da tela
        active_words.retain(|w| !w.is_off_screen());

        // Verifica se o input é igual a alguma palavra
        if is_key_pressed(KeyCode::Enter) {
            if let Some(pos) = active_words.iter().position(|w| w.text == input) {
                active_words.remove(pos);
                score += 1;
            }
            input.clear();
        }

        // Captura letras
        if let Some(c) = get_last_key_pressed() {
            match c {
                KeyCode::Backspace => {
                    input.pop();
                },
                _keys_toogeter_check => {
                    if matches!(c, KeyCode::A | KeyCode::B | KeyCode::C | KeyCode::D | KeyCode::E |
                        KeyCode::F | KeyCode::G | KeyCode::H | KeyCode::I | KeyCode::J |
                        KeyCode::K | KeyCode::L | KeyCode::M | KeyCode::N | KeyCode::O |
                        KeyCode::P | KeyCode::Q | KeyCode::R | KeyCode::S | KeyCode::T |
                        KeyCode::U | KeyCode::V | KeyCode::W | KeyCode::X | KeyCode::Y |
                        KeyCode::Z) {
                            let string_for = format!("{:?}", c);
                            input.push(string_for.to_string().to_lowercase().chars().next().unwrap());
                        }
                }
            }
        }

        draw_text(&format!("Input: {}", input), 20.0, 40.0, 30.0, GREEN);
        draw_text(&format!("Score: {}", score), 20.0, 80.0, 30.0, YELLOW);

        next_frame().await;
    }
}
