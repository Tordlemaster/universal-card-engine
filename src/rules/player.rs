use std::slice::Iter;

use rand::seq::SliceRandom;

#[derive (Clone)]
pub struct PlayerIndices {
    name_idx: usize,
    team_idx: usize,
    score: u32
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
            let mut new_player = PlayerIndices{name_idx: 0, team_idx: 0, score: 0};

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

    pub fn players_iter(&self) -> impl Iterator<Item = Player> {
        self.players.iter().map(|i| Player { name: self.names[i.name_idx].clone(), team: self.teams[i.team_idx], score: i.score })
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

    pub fn get_player_by_idx(&self, idx: usize) -> Option<Player> {
        if idx >= 0 && idx < self.players.len() {
            let p = &self.players[idx];
            Some(Player { name: self.names[p.name_idx].clone(), team: self.teams[p.team_idx], score: p.score })
        }
        else {
            None
        }
    }
    pub fn get_player_by_name(&self, s: &String) -> Option<Player> {
        let name_idx = self.names.iter().position(|x| x==s)?;
        let idx = self.players.iter().position(|x| x.name_idx==name_idx)?;

        if idx >= 0 && idx < self.players.len() {
            let p = &self.players[idx];
            Some(Player { name: self.names[p.name_idx].clone(), team: self.teams[p.team_idx], score: p.score })
        }
        else {
            None
        }
    }
    pub fn player_idx_from_name(&self, s: &String) -> Option<usize> {
        let name_idx = self.names.iter().position(|x| x==s)?;
        self.players.iter().position(|x| x.name_idx==name_idx)
    }
    pub fn add_player_score(&mut self, player_name: &String, score: u32) {
        let i = self.player_idx_from_name(player_name).unwrap();
        self.players[i].score += score; 
    }
}

#[derive(Clone)]
pub struct Player {
    name: String,
    team: usize,
    score: u32
}

impl Player {
    pub fn new(name: String, team: usize) -> Player {
        Player { name: name, team: team, score: 0 }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn team(&self) -> &usize {
        &self.team
    }

    pub fn score(&self) -> u32 {
        self.score
    }
}