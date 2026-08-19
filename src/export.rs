use serde_json::{Value, json};

/// One card row ready for serialization, with its deck name.
#[derive(Clone, Debug, PartialEq)]
pub struct ExportRow {
    pub deck: String,
    pub front: String,
    pub back: String,
    pub ease: f64,
    pub interval: i64,
    pub repetitions: i64,
    pub due_at: i64,
    pub updated_at: i64,
}

pub fn to_csv(rows: &[ExportRow]) -> String {
    let mut out = String::from("deck,front,back,ease,interval,repetitions,due_at,updated_at\n");
    for r in rows {
        out.push_str(&csv_field(&r.deck));
        out.push(',');
        out.push_str(&csv_field(&r.front));
        out.push(',');
        out.push_str(&csv_field(&r.back));
        out.push_str(&format!(
            ",{},{},{},{},{}\n",
            r.ease, r.interval, r.repetitions, r.due_at, r.updated_at
        ));
    }
    out
}

pub fn to_json(rows: &[ExportRow]) -> Value {
    let mut decks: Vec<Value> = Vec::new();
    let mut current: Option<(String, Vec<Value>)> = None;
    for r in rows {
        if current.as_ref().map(|(name, _)| name.as_str()) != Some(r.deck.as_str()) {
            if let Some((name, cards)) = current.take() {
                decks.push(json!({ "name": name, "cards": cards }));
            }
            current = Some((r.deck.clone(), Vec::new()));
        }
        if let Some((_, cards)) = &mut current {
            cards.push(json!({
                "front": r.front,
                "back": r.back,
                "ease": r.ease,
                "interval": r.interval,
                "repetitions": r.repetitions,
                "due_at": r.due_at,
                "updated_at": r.updated_at,
            }));
        }
    }
    if let Some((name, cards)) = current {
        decks.push(json!({ "name": name, "cards": cards }));
    }
    json!({ "decks": decks })
}

fn csv_field(s: &str) -> String {
    if s.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(deck: &str, front: &str, back: &str) -> ExportRow {
        ExportRow {
            deck: deck.to_string(),
            front: front.to_string(),
            back: back.to_string(),
            ease: 2.5,
            interval: 1,
            repetitions: 2,
            due_at: 3,
            updated_at: 4,
        }
    }

    #[test]
    fn csv_escapes_commas_and_quotes() {
        let rows = [row("a,b", "say \"hi\"", "line1\nline2")];
        let csv = to_csv(&rows);
        assert!(csv.starts_with("deck,front,back,ease,interval,repetitions,due_at,updated_at\n"));
        assert!(csv.contains("\"a,b\""));
        assert!(csv.contains("\"say \"\"hi\"\"\""));
        assert!(csv.contains("\"line1\nline2\""));
    }

    #[test]
    fn json_groups_cards_by_deck() {
        let rows = [
            row("A", "f1", "b1"),
            row("A", "f2", "b2"),
            row("B", "f3", "b3"),
        ];
        let value = to_json(&rows);
        assert_eq!(value["decks"].as_array().unwrap().len(), 2);
        assert_eq!(value["decks"][0]["name"], "A");
        assert_eq!(value["decks"][0]["cards"].as_array().unwrap().len(), 2);
        assert_eq!(value["decks"][1]["cards"][0]["front"], "f3");
    }
}
