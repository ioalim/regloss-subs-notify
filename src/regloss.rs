use num_format::{Locale, ToFormattedString};

pub fn subscribers_as_str() -> String {
    let mut result = String::new();
    let names = vec![
        "Hiodoshi Ao",
        "Otonose Kanade",
        "Ichijou Ririka",
        "Juufuutei Raden",
        "Todoroki Hajime",
    ];
    let ids = vec![
        "hiodoshiao".to_string(),
        "otonosekanade".to_string(),
        "ichijouririka".to_string(),
        "juufuuteiraden".to_string(),
        "todorokihajime".to_string(),
    ];
    let (subs, total) = yt_subs::get(&ids);
    for i in 0..names.len() {
        result.push_str(&format!("{} has {} subscribers\n", names[i], subs[i].to_formatted_string(&Locale::en)));
    }
    result.push_str(&format!("Regloss have {} subscribers in total", total.to_formatted_string(&Locale::en)));
    result
}

#[test]
fn test_subscribers_as_str() {
    let result = subscribers_as_str();
    println!("{}", result);
}
