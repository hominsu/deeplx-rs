use rand::{thread_rng, Rng};

pub(crate) fn get_random_number() -> i64 {
    let num = thread_rng().gen_range(1..99999) + 8300000;
    (num as i64) * 1000
}

pub(crate) fn get_i_count(text: &str) -> i64 {
    text.chars().filter(|c| *c == 'i').count() as i64
}

pub(crate) fn get_timestamp(mut i_count: i64) -> i64 {
    let ts = chrono::Utc::now().timestamp_millis();
    if i_count != 0 {
        i_count += 1;
        ts - ts % i_count + i_count
    } else {
        ts
    }
}

pub(crate) fn is_rich_text(text: &str) -> bool {
    text.contains("<") && text.contains(">")
}
