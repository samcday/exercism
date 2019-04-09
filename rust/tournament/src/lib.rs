#![deny(clippy::all, clippy::pedantic)]

use std::collections::HashMap;
use std::fmt::Write;

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

#[derive(Default)]
struct TeamStats(u16, u16, u16, u16, u16);
impl std::ops::AddAssign for TeamStats {
    #[rustfmt::skip]
    fn add_assign(&mut self, o: Self) {
        self.0 += o.0; self.1 += o.1; self.2 += o.2; self.3 += o.3; self.4 += o.4;
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<&str, TeamStats> = HashMap::new();
    match_results
        .lines()
        .map(|data| data.split(';').collect::<Vec<&str>>())
        .enumerate()
        .for_each(|(line, match_info)| {
            assert_eq!(match_info.len(), 3, "Malformed data on line {}", line);
            let (team_1_results, team_2_results) = match match_info[2] {
                "win" => (TeamStats(1, 1, 0, 0, 3), TeamStats(1, 0, 1, 0, 0)),
                "loss" => (TeamStats(1, 0, 1, 0, 0), TeamStats(1, 1, 0, 0, 3)),
                "draw" => (TeamStats(1, 0, 0, 1, 1), TeamStats(1, 0, 0, 1, 1)),
                result => panic!("Unexpected match result: {}", result),
            };
            *teams.entry(match_info[0]).or_default() += team_1_results;
            *teams.entry(match_info[1]).or_default() += team_2_results;
        });

    let mut sorted_teams = teams.keys().cloned().collect::<Vec<&str>>();
    sorted_teams.sort_unstable_by(|team1, team2| teams[team2].4.cmp(&teams[team1].4).then(team1.cmp(team2)));

    let mut out = String::with_capacity((1 + teams.len()) * HEADER.len());
    out.push_str(HEADER);
    for name in &sorted_teams {
        let TeamStats(mp, w, l, d, pts) = teams[name];
        write!(out, "\n{:31}| {:^3}| {:^3}| {:^3}| {:^3}|{:>3}", name, mp, w, d, l, pts).unwrap()
    }
    out
}
