use clap::{Parser, Subcommand};

const ABOUT: &str = "A command line tool to schedule a mouse click at a specific time.
GitHub: https://github.com/JasonWei512/schedule-mouse-click";

#[derive(Parser)]
#[clap(author, version, about = ABOUT)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Commands>,

    /// Double-click. For example:
    ///     schedule-mouse-click --double at 18:00:00
    ///     schedule-mouse-click --double in 2m30s
    ///     schedule-mouse-click -d now
    #[clap(short, long, verbatim_doc_comment)]
    pub double: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Click at a specific time. For example, at 18:00:00 :
    /// schedule-mouse-click at 18:00:00
    /// schedule-mouse-click at 18:00
    /// schedule-mouse-click at 6:00pm
    #[clap(verbatim_doc_comment)]
    At {
        /// Time to click, such as "18:00:00", "18:00" or "6:00pm"
        time: String,
    },

    /// Click after a specific amount of time. For example, in 2 minutes and 30 seconds:
    /// schedule-mouse-click in 2m30s
    /// schedule-mouse-click in 150s
    /// schedule-mouse-click in 150
    #[clap(verbatim_doc_comment)]
    In {
        /// Amount of time to wait before clicking, such as "2m30s", "150s" or "150"
        duration: String,
    },

    /// Click now.
    /// schedule-mouse-click now
    #[clap(verbatim_doc_comment)]
    Now,
}
