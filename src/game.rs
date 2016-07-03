use ::input::{Input, PlayerInput, KeyInput};
use ::fighter::Fighter;
use ::package::Package;
use ::player::Player;
use ::rules::Rules;
use ::stage::Stage;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use glium::glutin::VirtualKeyCode;

enum GameState {
    Running,
    Paused,
    Results,
}

pub struct Game {
    // package data
    rules:    Rules,
    fighters: Arc<Mutex<Vec<Fighter>>>,
    stages:   Arc<Mutex<Vec<Stage>>>,

    // variables
    pub players:       Arc<Mutex<Vec<Player>>>,
    selected_fighters: Vec<usize>,
    selected_stage:    usize,
    edit_player:       usize,
    debug_outputs:     Vec<DebugOutput>,
    state:             GameState,
    frames:            u64,
}

impl Game {
    pub fn new(package: &Package, selected_fighters: Vec<usize>, selected_stage: usize) -> Game {
        let mut players: Vec<Player> = Vec::new();

        {
            let stages = package.stages.lock().unwrap();
            for i in 0..selected_fighters.len() {
                let spawn = stages[selected_stage].spawn_points[i].clone();
                players.push(Player::new(spawn.clone(), package.rules.stock_count));
            }
        }

        Game {
            state:    GameState::Running,
            rules:    package.rules.clone(),
            fighters: package.fighters.clone(),
            stages:   package.stages.clone(),

            selected_fighters: selected_fighters,
            selected_stage:    selected_stage,
            edit_player:       0,
            debug_outputs:     vec!(),
            players:           Arc::new(Mutex::new(players)),
            frames:            0,
        }
    }

    pub fn run(&mut self, input: &mut Input, key_input: &Arc<Mutex<KeyInput>>) {
        loop {
            let player_input = input.read(self.frames);
            let key_input = key_input.lock().unwrap();
            match self.state {
                GameState::Running => { self.step_game(player_input); },
                GameState::Paused  => { self.step_pause(player_input, &key_input); },
                GameState::Results => { self.step_results(); },
            }

            thread::sleep(Duration::from_millis(16));
            //TODO: when finished results screen, return, without aborting
        }
    }

    fn step_game(&mut self, player_input: &Vec<PlayerInput>) {
            // lock resources
            let mut players = self.players.lock().unwrap();
            let fighters = self.fighters.lock().unwrap();
            let stages = self.stages.lock().unwrap();
            let stage = &stages[self.selected_stage];

            // step each player
            for (i, player) in (&mut *players).iter_mut().enumerate() {
                if true { // TODO: check not netplay
                    if player_input[i].start.press {
                        self.state = GameState::Paused;
                    }
                }

                let fighter = &fighters[self.selected_fighters[i]];
                player.step(&player_input[i], fighter, stage);
            }

            // handle timer
            self.frames += 1;
            if self.frames / 60 > self.rules.time_limit {
                self.state = GameState::Results;
            }

        println!("\n-------------------------------------------");
        println!("Frame {} ", self.frames);

        for debug_output in &self.debug_outputs {
            match debug_output {
                &DebugOutput::Physics{ player } => {
                    print!("Player: {}    ", player);
                    players[player].debug_physics();
                },
                &DebugOutput::Input{ player } => {
                    print!("Player: {}    ", player);
                    players[player].debug_input(&player_input[player]);
                },
                &DebugOutput::InputDiff{ player } => {
                    print!("Player: {}    ", player);
                    players[player].debug_input_diff(&player_input[player]);
                },
                &DebugOutput::Action{ player } => {
                    print!("Player: {}    ", player);
                    players[player].debug_action(&fighters[self.selected_fighters[player]]);
                },
                &DebugOutput::Frame{ player } => {
                    print!("Player: {}    ", player);
                    players[player].debug_frame(&fighters[self.selected_fighters[player]]);
                },
            }
        }
    }

    fn step_pause(&mut self, player_input: &Vec<PlayerInput>, key_input: &KeyInput) {
        // frame advance
        if key_input.pressed(VirtualKeyCode::Space) {
            self.step_game(player_input);
            return;
        }

        let players = self.players.lock().unwrap();

        if key_input.pressed(VirtualKeyCode::Key1) && players.len() >= 1 {
            self.edit_player = 0;
        }
        else if key_input.pressed(VirtualKeyCode::Key2) && players.len() >= 2 {
            self.edit_player = 1;
        }
        else if key_input.pressed(VirtualKeyCode::Key3) && players.len() >= 3 {
            self.edit_player = 2;
        }
        else if key_input.pressed(VirtualKeyCode::Key4) && players.len() >= 4 {
            self.edit_player = 3;
        }

        if key_input.pressed(VirtualKeyCode::F1) {
            self.debug_outputs.push(DebugOutput::Physics{ player: self.edit_player });
        }
        else if key_input.pressed(VirtualKeyCode::F2) {
            if key_input.held(VirtualKeyCode::LShift) || key_input.held(VirtualKeyCode::RShift) {
                self.debug_outputs.push(DebugOutput::InputDiff{ player: self.edit_player });
            }
            else {
                self.debug_outputs.push(DebugOutput::Input{ player: self.edit_player });
            }
        }
        else if key_input.pressed(VirtualKeyCode::F3) {
            self.debug_outputs.push(DebugOutput::Action{ player: self.edit_player });
        }
        else if key_input.pressed(VirtualKeyCode::F4) {
            self.debug_outputs.push(DebugOutput::Frame{ player: self.edit_player });
        }
        else if key_input.pressed(VirtualKeyCode::F5) {
            self.debug_outputs = vec!();
        }

        // allow players to resume game
        for (i, _) in (&players).iter().enumerate() {
            if player_input[i].start.press {
                self.state = GameState::Running;
            }
        }

        //TODO: Handle character/stage edits here
    }

    fn step_results(&mut self) {
    }
}

enum DebugOutput {
    Physics   {player: usize},
    Input     {player: usize},
    InputDiff {player: usize},
    Action    {player: usize},
    Frame     {player: usize},
}

#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct Point {
    pub x: f64,
    pub y: f64
}
