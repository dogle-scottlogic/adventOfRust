pub fn solve_part_one(ip_list: Vec<String>) -> u32 {
    let mut count = 0;
    for ip in ip_list {
        let sub_strings = get_hyper_net_substrings(ip);
        let string_list = sub_strings.0;
        let hyper_net_string_list = sub_strings.1;

        let has_abba_in_list = strings_have_at_least_one_abba(string_list);
        let has_abba_in_hyper_net = strings_have_at_least_one_abba(hyper_net_string_list);

        if has_abba_in_list && !has_abba_in_hyper_net {
            count = count + 1;
        }
    }
    return count;
}

pub fn solve_part_two(ip_list: Vec<String>) -> u32 {
    let mut count = 0;
    for ip in ip_list {
        let sub_strings = get_hyper_net_substrings(ip);
        let string_list = sub_strings.0;
        let hyper_net_string_list = sub_strings.1;

        let valid_babs = get_all_valid_babs(string_list);
        let has_valid_bab = strings_have_at_least_one_bab(hyper_net_string_list, valid_babs);

        if has_valid_bab {
            count = count + 1;
        }
    }
    return count;
}

fn get_hyper_net_substrings(ip: String) -> (Vec<String>, Vec<String>) {
    let mut hyper_net_string_list = Vec::<String>::new();
    let mut string_list = Vec::<String>::new();
    let mut sub_string = "".to_string();
    for i in 0..ip.len() {
        if ip.chars().nth(i).unwrap() == '[' {
            string_list.push(sub_string.clone());
            sub_string = "".to_string();
        } else if ip.chars().nth(i).unwrap() == ']' {
            hyper_net_string_list.push(sub_string.clone());
            sub_string = "".to_string();
        } else {
            sub_string.push(ip.chars().nth(i).unwrap());
        }
    }

    if sub_string.len() > 0 {
        string_list.push(sub_string);
    }

    return (string_list, hyper_net_string_list);
}

fn strings_have_at_least_one_abba(string_list: Vec<String>) -> bool {
    for s in string_list {
        if has_at_least_one_abba(s) {
            return true;
        }
    }
    return false;
}

fn get_all_valid_babs(string_list: Vec<String>) -> Vec<String> {
    let mut all_abas = Vec::<String>::new();
    for list in string_list {
        all_abas.append(&mut &mut get_all_valid_bab(list));
    }
    return all_abas;
}

fn strings_have_at_least_one_bab(string_list: Vec<String>, valid_abas: Vec<String>) -> bool {
    for s in string_list {
        if has_at_least_one_valid_bab(s, valid_abas.clone()) {
            return true;
        }
    }
    return false;
}

fn has_at_least_one_abba(input: String) -> bool {
    for i in 0..input.len() {
        if input.len() - i < 4 {
            return false;
        }
        let range: String = input.chars().skip(i).take(4).collect();
        if range.chars().nth(0) == range.chars().nth(3)
            && range.chars().nth(1) == range.chars().nth(2)
            && range.chars().nth(0) != range.chars().nth(1)
        {
            return true;
        }
    }
    return false;
}

fn get_all_valid_bab(input: String) -> Vec<String> {
    let mut all_babs = Vec::<String>::new();
    for i in 0..input.len() {
        if input.len() - i < 3 {
            return all_babs;
        }
        let range: String = input.chars().skip(i).take(3).collect();
        if is_aba(&range) {
            all_babs.push(format!(
                "{}{}{}",
                range.chars().nth(1).unwrap(),
                range.chars().nth(0).unwrap(),
                range.chars().nth(1).unwrap()
            ));
        }
    }
    return all_babs;
}

fn has_at_least_one_valid_bab(input: String, valid_babs: Vec<String>) -> bool {
    for i in 0..input.len() {
        if input.len() - i < 3 {
            return false;
        }
        let range: String = input.chars().skip(i).take(3).collect();
        if is_aba(&range) && valid_babs.contains(&range) {
            return true;
        }
    }
    return true;
}

fn is_aba(range: &String) -> bool {
    return range.chars().nth(0) == range.chars().nth(2)
        && range.chars().nth(0) != range.chars().nth(1);
}
