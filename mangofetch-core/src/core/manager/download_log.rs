use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

pub const MAX_LINES_PER_DOWNLOAD: usize = 200;
const EMIT_THROTTLE_MS: u64 = 200;

struct Entry {
    lines: VecDeque<String>,
    last_emit: Option<Instant>,
    pending_emit: bool,
}

impl Entry {
    fn new() -> Self {
        Self {
            lines: VecDeque::with_capacity(MAX_LINES_PER_DOWNLOAD),
            last_emit: None,
            pending_emit: false,
        }
    }
}

static STORE: OnceLock<Mutex<HashMap<u64, Entry>>> = OnceLock::new();

fn store() -> &'static Mutex<HashMap<u64, Entry>> {
    STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn push_line(id: u64, line: &str) -> bool {
    let mut map = match store().lock() {
        Ok(g) => g,
        Err(_) => return false,
    };
    let entry = map.entry(id).or_insert_with(Entry::new);
    if entry.lines.len() >= MAX_LINES_PER_DOWNLOAD {
        entry.lines.pop_front();
    }
    entry.lines.push_back(line.to_string());

    let now = Instant::now();
    let should_emit = match entry.last_emit {
        Some(t) => now.duration_since(t) >= Duration::from_millis(EMIT_THROTTLE_MS),
        None => true,
    };
    if should_emit {
        entry.last_emit = Some(now);
        entry.pending_emit = false;
        true
    } else {
        entry.pending_emit = true;
        false
    }
}

pub fn get(id: u64) -> Vec<String> {
    match store().lock() {
        Ok(g) => g
            .get(&id)
            .map(|e| e.lines.iter().cloned().collect())
            .unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

pub fn clear(id: u64) {
    if let Ok(mut g) = store().lock() {
        g.remove(&id);
    }
}

pub fn clear_all() {
    if let Ok(mut g) = store().lock() {
        g.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::thread::sleep;

    static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

    fn get_lock() -> std::sync::MutexGuard<'static, ()> {
        TEST_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    #[test]
    fn test_push_line_and_get() {
        let _guard = get_lock();
        let id = 1;
        clear(id);

        let emitted = push_line(id, "first line");
        assert!(emitted, "First line should emit");

        let lines = get(id);
        assert_eq!(lines, vec!["first line"]);

        let emitted2 = push_line(id, "second line");
        assert!(!emitted2, "Second line within throttle should not emit");

        let lines2 = get(id);
        assert_eq!(lines2, vec!["first line", "second line"]);
    }

    #[test]
    fn test_max_lines_limit() {
        let _guard = get_lock();
        let id = 2;
        clear(id);

        for i in 0..(MAX_LINES_PER_DOWNLOAD + 5) {
            push_line(id, &format!("line {}", i));
        }

        let lines = get(id);
        assert_eq!(lines.len(), MAX_LINES_PER_DOWNLOAD);
        // Should have dropped 0..4, so first is 5
        assert_eq!(lines.first().unwrap(), "line 5");
        assert_eq!(lines.last().unwrap(), &format!("line {}", MAX_LINES_PER_DOWNLOAD + 4));
    }

    #[test]
    fn test_emit_throttle() {
        let _guard = get_lock();
        let id = 3;
        clear(id);

        assert!(push_line(id, "line 1"));
        assert!(!push_line(id, "line 2")); // Within throttle

        sleep(Duration::from_millis(EMIT_THROTTLE_MS + 10));

        assert!(push_line(id, "line 3")); // Outside throttle
    }

    #[test]
    fn test_clear() {
        let _guard = get_lock();
        let id = 4;
        clear(id);

        push_line(id, "line");
        assert_eq!(get(id).len(), 1);

        clear(id);
        assert!(get(id).is_empty());
    }

    #[test]
    fn test_clear_all() {
        let _guard = get_lock();
        let id1 = 5;
        let id2 = 6;

        push_line(id1, "line");
        push_line(id2, "line");

        assert_eq!(get(id1).len(), 1);
        assert_eq!(get(id2).len(), 1);

        clear_all();

        assert!(get(id1).is_empty());
        assert!(get(id2).is_empty());
    }
}
