#[derive(Clone, Debug, PartialEq)]
pub struct LabelPath {
    pub(crate) our: String,
    pub(crate) you: String,
    pub(crate) fight: bool,
}

pub fn new(label_path: LabelPath) -> (String, i32) {
    let our_str = replace_tag(label_path.our);
    let you_str = replace_tag(label_path.you);

    if our_str.len() < 4 || you_str.len() < 4 {
        return ("标签无效".to_string(), 3);
    }

    let our = our_str.as_bytes();
    let you = you_str.as_bytes();

    let fight = label_path.fight;
    println!("{} VS {}", our_str, you_str);

    let mut sum_num = 0;
    let max_tag = match our.len() >= you.len() {
        true => our,
        false => you
    };
    for i in 1..max_tag.len() {
        let curr_our = match i >= our.len() {
            true => -1,
            false => our[i] as i32
        };
        let curr_you = match i >= you.len() {
            true => -1,
            false => you[i] as i32
        };
        if curr_our == curr_you { continue; }
        if i > 3 && sum_num != 0 { break; }
        if curr_our > curr_you { sum_num = sum_num + 1 } else { sum_num = sum_num - 1 }
    }

    let stat_label = match (fight && sum_num > 0) || (!fight && sum_num < 0) {
        true => 1,
        false => match (fight && sum_num < 0) || (!fight && sum_num > 0) {
            true => 2,
            false => 3
        }
    };
    let bs = match fight {
        true => "比大",
        false => "比小"
    };
    let value_str = match stat_label {
        1 => format!("{} 赢局", bs),
        2 => format!("{} 输局", bs),
        _ => "无法比对".to_string()
    };
    (value_str, stat_label)
}

fn replace_tag(tag_str: String) -> String {
    let tag = tag_str.to_uppercase().replace("#", "");
    let mut str = String::from("#");
    str.push_str(tag.as_str());
    str
}
