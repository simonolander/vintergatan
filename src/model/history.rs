use crate::model::border::Border;

pub struct History {
    entries: Vec<HistoryEntry>,
    current_index: usize,
}

#[derive(Clone)]
pub enum HistoryEntry {
    ToggleBorder(Border),
}

impl History {
    pub fn new() -> Self {
        History {
            entries: Vec::new(),
            current_index: 0,
        }
    }

    pub fn has_past(&self) -> bool {
        self.current_index > 0
    }

    pub fn has_future(&self) -> bool {
        self.current_index < self.entries.len()
    }

    pub fn undo(&mut self) -> Option<&HistoryEntry> {
        if self.has_past() {
            self.current_index -= 1;
            self.entries.get(self.current_index)
        }
        else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<&HistoryEntry> {
        if self.has_future() {
            self.current_index += 1;
            self.entries.get(self.current_index - 1)
        }
        else {
            None
        }
    }

    pub fn push(&mut self, entry: HistoryEntry) {
        if self.has_future() {
            let mut future = self.entries[self.current_index..].iter().cloned().rev().collect();
            self.entries.append(&mut future);
            self.entries.push(entry);
            self.current_index = self.entries.len() + 1;
        }
        else {
            self.entries.push(entry);
            self.current_index += 1;
        }
    }
}
