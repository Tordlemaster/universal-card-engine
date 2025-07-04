use std::slice::Iter;

use rand::seq::SliceRandom;

#[derive (Clone)]
pub struct PlayerIndices {
    name_idx: usize,
    team_idx: usize
}

#[derive (Clone)]
pub struct PlayerSet {
    players: Vec<PlayerIndices>,
    names: Vec<String>,
    teams: Vec<usize>
}

impl PlayerSet {
    pub fn new(mut players: Vec<Player>) -> PlayerSet {
        players.shuffle(&mut rand::rng());

        let mut player_set = PlayerSet{players: Vec::new(), names: Vec::new(), teams: Vec::new()};
        for p in players {
            let mut new_player = PlayerIndices{name_idx: 0, team_idx: 0};

            //Initialize player name
            if let Some(i) = player_set.names.iter().position(|s| *s == p.name) {
                new_player.name_idx = i;
            }
            else {
                player_set.names.push(p.name);
                new_player.name_idx = player_set.names.len() - 1;
            }

            //Initialize player team
            if let Some(i) = player_set.teams.iter().position(|t| *t == p.team) {
                new_player.team_idx = i;
            }
            else {
                player_set.teams.push(p.team);
                new_player.team_idx = player_set.teams.len() - 1;
            }

            player_set.players.push(new_player);
        }
        player_set
    }

    pub fn names(&self) -> &Vec<String> {
        &self.names
    }

    pub fn teams(&self) -> &Vec<usize> {
        &self.teams
    }

    pub fn num_players(&self) -> usize {
        self.players.len()
    }

    pub fn get_player(&self, idx: usize) -> Player {
        Player { name: self.names[idx].clone(), team: self.teams[idx] }
    }
}

#[derive(Clone)]
pub struct Player {
    name: String,
    team: usize
}

impl Player {
    pub fn new(name: String, team: usize) -> Player {
        Player { name: name, team: team }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn team(&self) -> &usize {
        &self.team
    }
}