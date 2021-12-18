use std::collections::HashMap;

pub trait ValidWords {
    fn contains(&self, word: String) -> bool;
    fn size(&self) -> i32;
}

pub struct AllowedWords {
    pub words: Vec<String>,
}

impl ValidWords for AllowedWords {
    fn contains(&self, word: String) -> bool {
        self.words.contains(&word)
    }

    fn size(&self) -> i32 {
        self.words.len() as i32
    }
}

impl AllowedWords {
    pub fn new(word: String) -> Self {
        Self { words: vec![word] }
    }

    pub fn add(&mut self, word: String) {
        self.words.push(word)
    }
}

pub trait WordGame {
    type Words;
    fn submit_word(&mut self, player_name: String, word: String) -> i32;
    fn get_player_name_at_position(&self, position: i32) -> String;
    fn get_word_entry_at_position(&self, position: i32) -> String;
    fn get_score_at_position(&self, position: i32) -> i32;
}

pub struct ScoreBoard {
    pub starting_string: String,
    pub position_players: HashMap<i32, Vec<Player>>,
    pub score_position: HashMap<i32, i32>,
    pub player_details: HashMap<String, Player>,
    pub position_player_name: HashMap<i32, String>,
    pub position_word: HashMap<i32, String>,
    pub position_score: HashMap<i32, i32>,
    pub words: Vec<String>,
}

impl ScoreBoard {
    fn new() -> Self {
        Self {
            starting_string: String::default(),
            position_players: HashMap::new(),
            score_position: HashMap::new(),
            player_details: HashMap::new(),
            position_player_name: HashMap::new(),
            position_word: HashMap::new(),
            position_score: HashMap::new(),
            words: Vec::new(),
        }
    }
}

pub struct Player {
    pub player_name: String,
    pub words: Vec<String>,
    pub score: i32,
}

impl WordGame for ScoreBoard {
    type Words = ValidWords;
    fn submit_word(&mut self, player_name: String, word: String) -> i32 {
        if !is_only_lower_case_elements(word.as_bytes())
            || !is_all_char_present_in_start_string(
                word.as_bytes(),
                self.starting_string.as_bytes(),
            )
            || self.words.contains(&word)
        {
            return 0;
        };

        let score = word.len() as i32;

        let player = Player {
            player_name,
            words: vec![word],
            score,
        };

        let player_info = self
            .player_details
            .get_mut(&player_name)
            .expect("Player details not present.");
        player_info.score += score;
        player_info.words.push(word);

        self.words.push(word);
        score
    }

    fn get_player_name_at_position(&self, position: i32) -> String {
        *self
            .position_player_name
            .get(&position)
            .unwrap_or(&String::default())
    }

    fn get_word_entry_at_position(&self, position: i32) -> String {
        *self
            .position_word
            .get(&position)
            .unwrap_or(&String::default())
    }

    fn get_score_at_position(&self, position: i32) -> i32 {
        *self.position_score.get(&position).unwrap_or(&0)
    }
}

fn is_only_lower_case_elements(words: &[u8]) -> bool {
    for word in words.iter() {
        if word < &97 || word > &122 {
            return false;
        }
    }
    true
}

fn is_all_char_present_in_start_string(words: &[u8], start_string: &[u8]) -> bool {
    let mut words_element_counts: [u8; 26] = [0; 26];
    let mut start_string_element_counts: [u8; 26] = [0; 26];

    // count word elements
    for char in words {
        words_element_counts[*char as usize - 97] += 1;
    }

    // count start_string elements
    for char in start_string {
        start_string_element_counts[*char as usize - 97] += 1;
    }

    for index in 0..26 {
        if words_element_counts[index] > start_string_element_counts[index] {
            return false;
        }
    }
    true
}
