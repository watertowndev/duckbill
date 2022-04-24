use std::collections::{BTreeMap};
use std::fmt::{Display, Formatter};

pub struct MMenu {
    menu: BTreeMap<String, MenuEntry>,
}

struct MenuEntry {
    displayed: String,
    enabled: bool,
}

impl MMenu {
    pub fn new() -> MMenu {
        MMenu {
            menu: BTreeMap::new(),
        }
    }

    /// adds an item to the menu.
    /// if an entry with the same selector exists, updates instead
    pub fn add_entry(&mut self, selector: &str, displayed: &str, enabled: bool) {
        self.menu.insert(selector.to_lowercase(), MenuEntry {
            displayed: String::from(displayed),
            enabled
        });
    }

    /// validates a choice.
    /// a choice is valid if it exists and is enabled
    pub fn valid_choice(&self, selection: &str) -> bool {
        match self.menu.get(&selection.to_lowercase()) {
            Some(entry) => entry.enabled,
            None => false
        }
    }


    /// change the enabledness of a choice
    pub fn set_choice_avail(&mut self, selection:&str, enabled: bool) -> Result<(),()> {
        match self.menu.get_mut(&selection.to_lowercase()) {
            Some(entry) => {
                entry.enabled = enabled;
                Ok(())
            },
            None => Err(())
        }
    }
}


impl Display for MMenu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut items: Vec<String> = vec![];

        for (sel, entry) in self.menu.iter() {
            if entry.enabled {
                items.push(format!("{} {}", &sel, &entry.displayed))
            }
        }

        write!(f, "{}", items.join("\n"))
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn m_menu_dummytest() {
        assert_eq!(true, true);
    }
}
