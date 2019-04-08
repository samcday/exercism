#![deny(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

#[derive(Default)]
struct TeamInfo(u16, u16, u16, u16, u16);
impl std::ops::AddAssign for TeamInfo {
    fn add_assign(&mut self, o: Self) {
        self.0 += o.0;
        self.1 += o.1;
        self.2 += o.2;
        self.3 += o.3;
        self.4 += o.4;
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<&str, TeamInfo> = HashMap::new();

    for (line, data) in match_results.lines().enumerate() {
        let parts = data.split(';').collect::<Vec<&str>>();
        if parts.len() != 3 {
            panic!("malformed data on line {}", line);
        }

        let (team_1, team_2, result) = (parts[0], parts[1], parts[2]);
        let (team_1_results, team_2_results) = match result {
            "win" => (TeamInfo(1, 1, 0, 0, 3), TeamInfo(1, 0, 1, 0, 0)),
            "loss" => (TeamInfo(1, 0, 1, 0, 0), TeamInfo(1, 1, 0, 0, 3)),
            "draw" => (TeamInfo(1, 0, 0, 1, 1), TeamInfo(1, 0, 0, 1, 1)),
            _ => Default::default(),
        };

        *teams.entry(team_1).or_default() += team_1_results;
        *teams.entry(team_2).or_default() += team_2_results;
    }

    let mut sorted_teams = teams.keys().cloned().collect::<Vec<&str>>();
    sorted_teams.sort_unstable_by(|team1, team2| {
        teams[team2].4.cmp(&teams[team1].4).then(team1.cmp(team2))
    });

    [HEADER.to_string()]
        .iter()
        .cloned()
        .chain(sorted_teams.iter().map(|team_name| {
            let TeamInfo(team_mp, team_wins, team_losses, team_draws, team_pts) = teams[team_name];
            format!(
                "\n{:31}| {:^3}| {:^3}| {:^3}| {:^3}|{:>3}",
                team_name, team_mp, team_wins, team_draws, team_losses, team_pts
            )
        }))
        .collect::<String>()
}
