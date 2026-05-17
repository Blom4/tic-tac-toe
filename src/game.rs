use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn other(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Player::X => "X",
            Player::O => "O",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameMode {
    TwoPlayer,
    VsAI,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    cells: [Option<Player>; 9],
    history: VecDeque<[Option<Player>; 9]>,
    history_index: usize,
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [None; 9],
            history: VecDeque::new(),
            history_index: 0,
        }
    }

    pub fn cells(&self) -> &[Option<Player>; 9] {
        &self.cells
    }

    pub fn set(&mut self, index: usize, player: Player) -> bool {
        if index >= 9 || self.cells[index].is_some() {
            return false;
        }
        if self.history_index < self.history.len() {
            self.history.truncate(self.history_index);
        }
        self.cells[index] = Some(player);
        self.history.push_back(self.cells);
        self.history_index = self.history.len();
        true
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().all(|c| c.is_some())
    }

    pub fn winner(&self) -> Option<Player> {
        let lines = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8],
            [0, 3, 6], [1, 4, 7], [2, 5, 8],
            [0, 4, 8], [2, 4, 6],
        ];
        for line in lines {
            let a = self.cells[line[0]];
            let b = self.cells[line[1]];
            let c = self.cells[line[2]];
            if a.is_some() && a == b && b == c {
                return a;
            }
        }
        None
    }

    pub fn is_game_over(&self) -> bool {
        self.winner().is_some() || self.is_full()
    }

    pub fn can_undo(&self) -> bool {
        self.history_index > 0
    }

    pub fn can_redo(&self) -> bool {
        self.history_index < self.history.len()
    }

    pub fn undo(&mut self) -> bool {
        if !self.can_undo() {
            return false;
        }
        self.history_index -= 1;
        self.cells = self.history[self.history_index];
        true
    }

    pub fn redo(&mut self) -> bool {
        if !self.can_redo() {
            return false;
        }
        self.history_index += 1;
        self.cells = self.history[self.history_index];
        true
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn available_moves(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, cell)| cell.is_none())
            .map(|(i, _)| i)
            .collect()
    }

    fn evaluate(&self, player: Player) -> i32 {
        if let Some(winner) = self.winner() {
            if winner == player {
                return 10;
            } else {
                return -10;
            }
        }
        0
    }

    fn minimax(&self, depth: usize, is_maximizing: bool, player: Player) -> i32 {
        let score = self.evaluate(player);
        if score == 10 {
            return score - depth as i32;
        }
        if score == -10 {
            return score + depth as i32;
        }
        if self.is_full() {
            return 0;
        }

        if is_maximizing {
            let mut max_score = i32::MIN;
            for &mv in &self.available_moves() {
                let mut new_board = self.clone();
                new_board.cells[mv] = Some(player);
                let score = new_board.minimax(depth + 1, false, player);
                max_score = max_score.max(score);
            }
            max_score
        } else {
            let mut min_score = i32::MAX;
            for &mv in &self.available_moves() {
                let mut new_board = self.clone();
                new_board.cells[mv] = Some(player.other());
                let score = new_board.minimax(depth + 1, true, player);
                min_score = min_score.min(score);
            }
            min_score
        }
    }

    pub fn best_move(&self, player: Player) -> Option<usize> {
        let moves = self.available_moves();
        if moves.is_empty() {
            return None;
        }
        let mut best_score = i32::MIN;
        let mut best_move = moves[0];
        for mv in moves {
            let mut new_board = self.clone();
            new_board.cells[mv] = Some(player);
            let score = new_board.minimax(0, false, player);
            if score > best_score {
                best_score = score;
                best_move = mv;
            }
        }
        Some(best_move)
    }

    pub fn ai_move(&mut self, difficulty: Difficulty, ai_player: Player) -> bool {
        let moves = self.available_moves();
        if moves.is_empty() {
            return false;
        }
        let mv = match difficulty {
            Difficulty::Easy => {
                moves[fastrand::usize(..moves.len())]
            }
            Difficulty::Medium => {
                if fastrand::f32() < 0.7 {
                    self.best_move(ai_player).unwrap_or(moves[fastrand::usize(..moves.len())])
                } else {
                    moves[fastrand::usize(..moves.len())]
                }
            }
            Difficulty::Hard => {
                self.best_move(ai_player).unwrap_or(moves[0])
            }
        };
        self.set(mv, ai_player)
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GameState {
    pub board: Board,
    pub current_player: Player,
    pub mode: GameMode,
    pub difficulty: Difficulty,
    pub x_wins: u32,
    pub o_wins: u32,
    pub draws: u32,
}

impl GameState {
    pub fn new() -> Self {
        let (x_wins, o_wins, draws) = Self::load_from_storage().unwrap_or((0, 0, 0));
        Self {
            board: Board::new(),
            current_player: Player::X,
            mode: GameMode::TwoPlayer,
            difficulty: Difficulty::Hard,
            x_wins,
            o_wins,
            draws,
        }
    }

    fn load_from_storage() -> Option<(u32, u32, u32)> {
        let window = web_sys::window()?;
        let storage = window.local_storage().ok().flatten()?;
        let data = storage.get_item("tictactoe_scores").ok().flatten()?;
        let parsed: (u32, u32, u32) = serde_json::from_str(&data).ok()?;
        Some(parsed)
    }

    fn save_to_storage(&self) {
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                if let Ok(data) = serde_json::to_string(&(self.x_wins, self.o_wins, self.draws)) {
                    let _ = storage.set_item("tictactoe_scores", &data);
                }
            }
        }
    }

    pub fn set_mode(&mut self, mode: GameMode) {
        self.mode = mode;
        self.reset();
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
        self.reset();
    }

    pub fn make_move(&mut self, index: usize) -> bool {
        if self.board.is_game_over() {
            return false;
        }
        if !self.board.set(index, self.current_player) {
            return false;
        }
        if let Some(winner) = self.board.winner() {
            match winner {
                Player::X => self.x_wins += 1,
                Player::O => self.o_wins += 1,
            }
            self.save_to_storage();
            return true;
        }
        if self.board.is_full() {
            self.draws += 1;
            self.save_to_storage();
            return true;
        }
        self.current_player = self.current_player.other();
        if self.mode == GameMode::VsAI && self.current_player == Player::O {
            let _ = self.board.ai_move(self.difficulty, Player::O);
            if let Some(winner) = self.board.winner() {
                match winner {
                    Player::X => self.x_wins += 1,
                    Player::O => self.o_wins += 1,
                }
                self.save_to_storage();
            } else if self.board.is_full() {
                self.draws += 1;
                self.save_to_storage();
            }
            self.current_player = Player::X;
        }
        true
    }

    pub fn undo(&mut self) -> bool {
        if self.mode == GameMode::VsAI {
            let undone = self.board.undo();
            if undone && self.current_player == Player::O {
                let _ = self.board.undo();
            }
            if self.board.history_index > 0 {
                self.current_player = Player::X;
            }
            undone
        } else {
            let undone = self.board.undo();
            if undone {
                self.current_player = self.current_player.other();
            }
            undone
        }
    }

    pub fn redo(&mut self) -> bool {
        if self.mode == GameMode::VsAI {
            let redone = self.board.redo();
            if redone && self.current_player == Player::X {
                let _ = self.board.redo();
                self.current_player = Player::X;
            }
            redone
        } else {
            let redone = self.board.redo();
            if redone {
                self.current_player = self.current_player.other();
            }
            redone
        }
    }

    pub fn reset(&mut self) {
        self.board.reset();
        self.current_player = Player::X;
    }

    pub fn reset_scores(&mut self) {
        self.x_wins = 0;
        self.o_wins = 0;
        self.draws = 0;
        self.save_to_storage();
        self.reset();
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}