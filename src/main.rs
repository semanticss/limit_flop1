use std::collections::{BTreeMap, HashMap};

extern "C" {
    pub fn eval_7cards(
        c0: i32, c1: i32, c2: i32, c3: i32, c4: i32, c5: i32, c6: i32
    ) -> i32;
}


const RANKS: [char; 3] = ['K', 'Q', 'J'];
const ACTIONS: [char; 2] = ['b', 'p'];

#[derive(Clone, Debug)]
struct InfoSetActionData {
    strategy: f64,
    util: f64,
    cumulative_gain: f64,
}

#[derive(Clone, Debug)]
struct InfoSetData {
    actions: HashMap<char, InfoSetActionData>,
    beliefs: BTreeMap<char, f64>,
    expected_util: f64,
    likelihood: f64,
}

impl InfoSetData {
    fn new_uniform() -> Self {
        let init = 1.0 / ACTIONS.len() as f64;
        let mut actions = HashMap::new();
        for &a in &ACTIONS {
            actions.insert(
                a,
                InfoSetActionData {
                    strategy: init,
                    util: 0.0,
                    cumulative_gain: init,
                },
            );
        }
        Self {
            actions,
            beliefs: BTreeMap::new(),
            expected_util: 0.0,
            likelihood: 0.0,
        }
    }
}

fn get_deciding_player_for_infoset_str(info: &str) -> usize {
    ((info.len() as isize) - 1).rem_euclid(2) as usize
}

fn terminal_action(action_str: &str) -> bool {
    matches!(action_str, "pp" | "bb" | "bp" | "pbb" | "pbp")
}

fn get_possible_opponent_pockets(pocket: char) -> Vec<char> {
    RANKS.iter().copied().filter(|&r| r != pocket).collect()
}

fn get_ancestral_infoset_strs(info: &str) -> Vec<String> {
    if info.len() == 1 {
        panic!("no ancestors of infoSet={}", info);
    }
    let opps = get_possible_opponent_pockets(info.chars().next().unwrap());
    let mid = &info[1..info.len() - 1];
    opps.into_iter().map(|c| format!("{c}{mid}")).collect()
}

fn get_descendant_infoset_strs(info: &str, action: char) -> Vec<String> {
    let opps = get_possible_opponent_pockets(info.chars().next().unwrap());
    let action_str = format!("{}{}", &info[1..], action);
    opps.into_iter().map(|c| format!("{c}{action_str}")).collect()
}

fn player_one_pocket_is_higher(p1: char, p2: char) -> bool {
    match (p1, p2) {
        ('K', _) => true,
        ('J', _) => false,
        (_, 'K') => false,
        (_, 'J') => true,
        _ => panic!("invalid pockets"),
    }
}

fn calc_utility_at_terminal_node(p1: char, p2: char, action_str: &str) -> (i32, i32) {
    match action_str {
        "pp" => {
            if player_one_pocket_is_higher(p1, p2) {
                (1, -1)
            } else {
                (-1, 1)
            }
        }
        "pbp" => (-1, 1),
        "bp" => (1, -1),
        "bb" | "pbb" => {
            if player_one_pocket_is_higher(p1, p2) {
                (2, -2)
            } else {
                (-2, 2)
            }
        }
        _ => panic!("unexpected actionStr={}", action_str),
    }
}

fn init_infosets(info_sets: &mut HashMap<String, InfoSetData>, order: &mut Vec<String>) {
    let mut info_action_strs = vec!["", "p", "b", "pb"];
    info_action_strs.sort_by_key(|s| s.len());
    for s in info_action_strs {
        for &r in &RANKS {
            let key = format!("{r}{s}");
            info_sets.insert(key.clone(), InfoSetData::new_uniform());
            order.push(key);
        }
    }
}

fn update_beliefs(info_sets: &mut HashMap<String, InfoSetData>, order: &[String]) {
    for key in order {
        let mut beliefs = BTreeMap::new();
        if key.len() == 1 {
            for opp in get_possible_opponent_pockets(key.chars().next().unwrap()) {
                beliefs.insert(opp, 1.0 / 2.0);
            }
        } else {
            let ancestors = get_ancestral_infoset_strs(key);
            let last_action = key.chars().last().unwrap();
            let mut tot = 0.0;
            for a in &ancestors {
                let opp = &info_sets[a];
                tot += opp.actions.get(&last_action).unwrap().strategy;
            }
            for a in &ancestors {
                let opp = &info_sets[a];
                let opp_pocket = a.chars().next().unwrap();
                let val = opp.actions.get(&last_action).unwrap().strategy / tot;
                beliefs.insert(opp_pocket, val);
            }
        }
        info_sets.get_mut(key).unwrap().beliefs = beliefs;
    }
}

fn update_utilities_for_infoset_str(key: &str, info_sets: &mut HashMap<String, InfoSetData>) {
    let player_idx = get_deciding_player_for_infoset_str(key);
    let beliefs = info_sets[key].beliefs.clone();

    let mut action_utils: HashMap<char, f64> = HashMap::new();

    for &action in &ACTIONS {
        let action_str = format!("{}{}", &key[1..], action);
        let descendants = get_descendant_infoset_strs(key, action);

        let mut util_from_infosets = 0.0f64;
        let mut util_from_terms = 0.0f64;

        for d in descendants {
            let opp_pocket = d.chars().next().unwrap();
            let prob_infoset = *beliefs.get(&opp_pocket).unwrap();

            let mut pockets = vec![key.chars().next().unwrap(), opp_pocket];
            if player_idx == 1 {
                pockets.reverse();
            }

            if terminal_action(&action_str) {
                let (u1, u2) = calc_utility_at_terminal_node(pockets[0], pockets[1], &action_str);
                let u = if player_idx == 0 { u1 } else { u2 } as f64;
                util_from_terms += prob_infoset * u;
            } else {
                let desc = info_sets.get(&d).unwrap().clone(); 
                for &opp_action in &ACTIONS {
                    let p_opp = desc.actions.get(&opp_action).unwrap().strategy;
                    let dest_key = format!("{key}{action}{opp_action}");
                    let dest_act_str = &dest_key[1..];

                    if terminal_action(dest_act_str) {
                        let (u1, u2) =
                            calc_utility_at_terminal_node(pockets[0], pockets[1], dest_act_str);
                        let u = if player_idx == 0 { u1 } else { u2 } as f64;
                        util_from_terms += prob_infoset * p_opp * u;
                    } else {
                        let dest_expected = info_sets[&dest_key].expected_util;
                        util_from_infosets += prob_infoset * p_opp * dest_expected;
                    }
                }
            }
        }
        action_utils.insert(action, util_from_infosets + util_from_terms);
    }

    let mut expected = 0.0;
    for &a in &ACTIONS {
        let u = *action_utils.get(&a).unwrap();
        info_sets.get_mut(key).unwrap().actions.get_mut(&a).unwrap().util = u;
    }
    for &a in &ACTIONS {
        let s = info_sets[key].actions.get(&a).unwrap().strategy;
        let u = info_sets[key].actions.get(&a).unwrap().util;
        expected += s * u;
    }
    info_sets.get_mut(key).unwrap().expected_util = expected;
}

fn calc_infoset_likelihoods(info_sets: &mut HashMap<String, InfoSetData>, order: &[String]) {
    for key in order {
        info_sets.get_mut(key).unwrap().likelihood = 0.0;
        let owner = key.chars().next().unwrap();
        let opps = get_possible_opponent_pockets(owner);

        if key.len() == 1 {
            info_sets.get_mut(key).unwrap().likelihood = 1.0 / RANKS.len() as f64;
        } else if key.len() == 2 {
            let last = key.chars().last().unwrap();
            for opp in &opps {
                let opp_info = format!("{opp}{}", &key[1..key.len() - 1]);
                let p = info_sets[&opp_info].actions.get(&last).unwrap().strategy;
                info_sets.get_mut(key).unwrap().likelihood +=
                    p / ((RANKS.len() * opps.len()) as f64);
            }
        } else {
            let last = key.chars().last().unwrap();
            let two_levels = &key[..key.len() - 2];
            let anc_like = info_sets[two_levels].likelihood / (opps.len() as f64);
            for opp in &opps {
                let opp_info = format!("{opp}{}", &key[1..key.len() - 1]);
                let p = info_sets[&opp_info].actions.get(&last).unwrap().strategy;
                info_sets.get_mut(key).unwrap().likelihood += anc_like * p;
            }
        }
    }
}

fn calc_gains(info_sets: &mut HashMap<String, InfoSetData>, order: &[String]) -> f64 {
    let mut tot_added = 0.0;
    for key in order {
        let expected = info_sets[key].expected_util;
        let like = info_sets[key].likelihood;
        for &a in &ACTIONS {
            let util = info_sets[key].actions.get(&a).unwrap().util;
            let gain = (util - expected).max(0.0);
            tot_added += gain;
            let entry = info_sets.get_mut(key).unwrap().actions.get_mut(&a).unwrap();
            entry.cumulative_gain += gain * like;
        }
    }
    tot_added
}

fn update_strategy(info_sets: &mut HashMap<String, InfoSetData>, order: &[String]) {
    for key in order {
        let g0 = info_sets[key].actions.get(&ACTIONS[0]).unwrap().cumulative_gain;
        let g1 = info_sets[key].actions.get(&ACTIONS[1]).unwrap().cumulative_gain;
        let tot = g0 + g1;
        for &a in &ACTIONS {
            let g = info_sets[key].actions.get(&a).unwrap().cumulative_gain;
            info_sets
                .get_mut(key)
                .unwrap()
                .actions
                .get_mut(&a)
                .unwrap()
                .strategy = if tot > 0.0 { g / tot } else { 0.5 };
        }
    }
}

#[allow(dead_code)]
fn set_initial_strategies_to_specific_values(info_sets: &mut HashMap<String, InfoSetData>) {

    info_sets.get_mut("K").unwrap().actions.get_mut(&'b').unwrap().strategy = 2.0 / 3.0;
    info_sets.get_mut("K").unwrap().actions.get_mut(&'p').unwrap().strategy = 1.0 / 3.0;

    info_sets.get_mut("Q").unwrap().actions.get_mut(&'b').unwrap().strategy = 0.5;
    info_sets.get_mut("Q").unwrap().actions.get_mut(&'p').unwrap().strategy = 0.5;

    info_sets.get_mut("J").unwrap().actions.get_mut(&'b').unwrap().strategy = 1.0 / 3.0;
    info_sets.get_mut("J").unwrap().actions.get_mut(&'p').unwrap().strategy = 2.0 / 3.0;

    info_sets.get_mut("Kpb").unwrap().actions.get_mut(&'b').unwrap().strategy = 1.0;
    info_sets.get_mut("Kpb").unwrap().actions.get_mut(&'p').unwrap().strategy = 0.0;

    info_sets.get_mut("Qpb").unwrap().actions.get_mut(&'b').unwrap().strategy = 0.5;
    info_sets.get_mut("Qpb").unwrap().actions.get_mut(&'p').unwrap().strategy = 0.5;

    info_sets.get_mut("Jpb").unwrap().actions.get_mut(&'b').unwrap().strategy = 0.0;
    info_sets.get_mut("Jpb").unwrap().actions.get_mut(&'p').unwrap().strategy = 1.0;

    info_sets.get_mut("Kb").unwrap().actions.get_mut(&'b').unwrap().strategy = 1.0;
    info_sets.get_mut("Kb").unwrap().actions.get_mut(&'p').unwrap().strategy = 0.0;
    info_sets.get_mut("Kp").unwrap().actions.get_mut(&'b').unwrap().strategy = 1.0;
    info_sets.get_mut("Kp").unwrap().actions.get_mut(&'p').unwrap().strategy = 0.0;

    info_sets.get_mut("Qb").unwrap().actions.get_mut(&'b').unwrap().strategy = 0.5;
    info_sets.get_mut("Qb").unwrap().actions.get_mut(&'p').unwrap().strategy = 0.5;
    info_sets.get_mut("Qp").unwrap().actions.get_mut(&'b').unwrap().strategy = 2.0 / 3.0;
    info_sets.get_mut("Qp").unwrap().actions.get_mut(&'p').unwrap().strategy = 1.0 / 3.0;

    info_sets.get_mut("Jb").unwrap().actions.get_mut(&'b').unwrap().strategy = 0.0;
    info_sets.get_mut("Jb").unwrap().actions.get_mut(&'p').unwrap().strategy = 1.0;
    info_sets.get_mut("Jp").unwrap().actions.get_mut(&'b').unwrap().strategy = 1.0 / 3.0;
    info_sets.get_mut("Jp").unwrap().actions.get_mut(&'p').unwrap().strategy = 2.0 / 3.0;
}

fn print_table(info_sets: &HashMap<String, InfoSetData>, order: &[String]) {
    let headers = [
        "InfoSet", "Strat:Bet", "Strat:Pass", "---", "Belief:H", "Belief:L", "---", "Util:Bet",
        "Util:Pass", "ExpectedUtil", "Likelihood", "---", "TotGain:Bet", "TotGain:Pass",
    ];
    println!(
        "{:<8} {:>9} {:>10} {:>3} {:>9} {:>9} {:>3} {:>9} {:>10} {:>12} {:>11} {:>3} {:>12} {:>13}",
        headers[0], headers[1], headers[2], headers[3], headers[4], headers[5], headers[6],
        headers[7], headers[8], headers[9], headers[10], headers[11], headers[12], headers[13]
    );

    for key in order {
        let isd = &info_sets[key];
        let s_b = isd.actions.get(&'b').unwrap().strategy;
        let s_p = isd.actions.get(&'p').unwrap().strategy;

        let mut blf: Vec<(char, f64)> = isd.beliefs.iter().map(|(c, v)| (*c, *v)).collect();
        blf.sort_by_key(|(c, _)| *c);
        let belief_hi = blf.get(0).map(|(_, v)| *v).unwrap_or(0.0);
        let belief_lo = blf.get(1).map(|(_, v)| *v).unwrap_or(0.0);

        let u_b = isd.actions.get(&'b').unwrap().util;
        let u_p = isd.actions.get(&'p').unwrap().util;
        let eg = isd.expected_util;
        let like = isd.likelihood;
        let g_b = isd.actions.get(&'b').unwrap().cumulative_gain;
        let g_p = isd.actions.get(&'p').unwrap().cumulative_gain;

        println!(
            "{:<8} {:>9.2} {:>10.2} {:>3} {:>9.2} {:>9.2} {:>3} {:>9.2} {:>10.2} {:>12.2} {:>11.2} {:>3} {:>12.2} {:>13.2}",
            key, s_b, s_p, "---", belief_hi, belief_lo, "---", u_b, u_p, eg, like, "---", g_b, g_p
        );
    }
}

fn main() {
    use std::fs::File;
    use std::io::Write;

    let mut info_sets: HashMap<String, InfoSetData> = HashMap::new();
    let mut order: Vec<String> = Vec::new();
    init_infosets(&mut info_sets, &mut order);

    let num_iterations: usize = 300_000;
    let num_gains_to_plot = 100usize;
    let mut gain_grp_size = num_iterations / num_gains_to_plot;
    if gain_grp_size == 0 {
        gain_grp_size = 1;
    }

    let mut tot_gains: Vec<f64> = Vec::new();
    let mut series: Vec<(usize, f64)> = Vec::new(); // (iteration, tot_gain)

    for i in 0..num_iterations {
        update_beliefs(&mut info_sets, &order);

        for key in order.iter().rev() {
            update_utilities_for_infoset_str(key, &mut info_sets);
        }

        calc_infoset_likelihoods(&mut info_sets, &order);
        let tot_gain = calc_gains(&mut info_sets, &order);

        if i % gain_grp_size == 0 {
            tot_gains.push(tot_gain);
            series.push((i, tot_gain));
        }

        update_strategy(&mut info_sets, &order);
    }

    print_table(&info_sets, &order);

    
    let mut f = File::create("tot_gain.csv").expect("create tot_gain.csv");
    writeln!(f, "iter,tot_gain").unwrap();
    for (i, g) in series {
        writeln!(f, "{},{}", i, g).unwrap();
    }
    println!("wrote tot_gain.csv");
}

