use crosstermion::crossterm::style::Stylize;
use owo_colors::OwoColorize;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
enum Usage {
    NotApplicable,
    Planned {
        note: Option<&'static str>,
    },
    InModule {
        name: &'static str,
        deviation: Option<&'static str>,
    },
}
use Usage::*;

impl Display for Usage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NotApplicable => f.write_str("not applicable")?,
            Planned { note } => {
                write!(f, "{}", "planned".blink())?;
                if let Some(note) = note {
                    write!(f, " ℹ {} ℹ", note.bright_white())?;
                }
            }
            InModule { name, deviation } => {
                write!(f, "mod {name}")?;
                if let Some(deviation) = deviation {
                    write!(f, "{}", format!(" ❗️{deviation}❗️").bright_white())?
                }
            }
        }
        Ok(())
    }
}

impl Usage {
    pub fn icon(&self) -> &'static str {
        match self {
            NotApplicable => "❌",
            Planned { .. } => "🕒",
            InModule { deviation, .. } => deviation.is_some().then(|| "👌️").unwrap_or("✅"),
        }
    }
}

#[derive(Clone)]
struct Record {
    config: &'static str,
    usage: Usage,
}

static GIT_CONFIG: &[Record] = &[
    Record {
        config: "fetch.output",
        usage: NotApplicable,
    },
    Record {
        config: "fetch.negotiationAlgorithm",
        usage: Planned {
            note: Some("Implements our own 'naive' algorithm, only"),
        },
    },
    Record {
        config: "pack.threads",
        usage: InModule {
            name: "remote::connection::fetch",
            deviation: Some("if unset, it uses all threads as opposed to just 1"),
        },
    },
];

/// A programmatic way to record and display progress.
pub fn show_progress() -> anyhow::Result<()> {
    let sorted = {
        let mut v: Vec<_> = GIT_CONFIG.into();
        v.sort_by_key(|r| r.config);
        v
    };

    for Record { config, usage } in sorted {
        println!("{} {}: {usage}", usage.icon(), config.bold(),);
    }
    Ok(())
}
