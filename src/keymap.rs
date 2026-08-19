use std::collections::HashMap;

use eframe::egui::Key;

/// User-remappable keyboard actions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Flip,
    Next,
    Previous,
    Edit,
}

impl Action {
    pub const ALL: [Action; 4] = [Action::Flip, Action::Next, Action::Previous, Action::Edit];

    pub fn name(self) -> &'static str {
        match self {
            Action::Flip => "flip",
            Action::Next => "next",
            Action::Previous => "previous",
            Action::Edit => "edit",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Action::Flip => "Flip card",
            Action::Next => "Next card",
            Action::Previous => "Previous card",
            Action::Edit => "Edit card",
        }
    }

    pub fn from_name(name: &str) -> Option<Action> {
        Action::ALL.into_iter().find(|a| a.name() == name)
    }
}

/// Maps every action to the key that triggers it.
#[derive(Clone)]
pub struct KeyBindings {
    map: HashMap<Action, Key>,
}

impl KeyBindings {
    pub fn defaults() -> Self {
        let mut map = HashMap::new();
        map.insert(Action::Flip, Key::Enter);
        map.insert(Action::Next, Key::ArrowRight);
        map.insert(Action::Previous, Key::ArrowLeft);
        map.insert(Action::Edit, Key::Enter);
        KeyBindings { map }
    }

    pub fn key(&self, action: Action) -> Key {
        self.map.get(&action).copied().unwrap_or(Key::Enter)
    }

    pub fn bind(&mut self, action: Action, key: Key) -> Result<(), String> {
        if let Some(other) = self.map.iter().find(|(a, k)| **k == key && **a != action) {
            return Err(format!(
                "{} is already bound to {}",
                key.name(),
                other.0.label()
            ));
        }
        self.map.insert(action, key);
        Ok(())
    }

    pub fn entries(&self) -> impl Iterator<Item = (Action, Key)> + '_ {
        Action::ALL.into_iter().map(|a| (a, self.key(a)))
    }

    pub fn from_pairs(pairs: impl IntoIterator<Item = (Action, Key)>) -> Self {
        let mut bindings = Self::defaults();
        for (action, key) in pairs {
            bindings.map.insert(action, key);
        }
        bindings
    }
}

pub fn key_from_name(name: &str) -> Option<Key> {
    Key::ALL.iter().copied().find(|k| k.name() == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_assign_each_action() {
        let b = KeyBindings::defaults();
        assert_eq!(b.key(Action::Flip), Key::Enter);
        assert_eq!(b.key(Action::Next), Key::ArrowRight);
        assert_eq!(b.key(Action::Previous), Key::ArrowLeft);
        assert_eq!(b.key(Action::Edit), Key::Enter);
        let o = KeyBindings::from_pairs([(Action::Flip, Key::Space)]);
        assert_eq!(o.key(Action::Flip), Key::Space);
        assert_eq!(o.key(Action::Next), Key::ArrowRight);
    }

    #[test]
    fn remap_errors_and_names_round_trip() {
        let mut b = KeyBindings::defaults();
        assert!(b.bind(Action::Edit, Key::Enter).is_err());
        assert!(b.bind(Action::Flip, Key::Space).is_ok());
        for action in Action::ALL {
            assert_eq!(Action::from_name(action.name()), Some(action));
        }
        assert_eq!(key_from_name("Enter"), Some(Key::Enter));
        assert_eq!(key_from_name("Nope"), None);
    }
}
