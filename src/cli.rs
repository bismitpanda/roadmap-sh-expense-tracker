use clap::{Parser, Subcommand};
use ulid::Ulid;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    Add {
        #[arg(long, short)]
        description: String,

        #[arg(long, short)]
        amount: usize,
    },

    Update {
        #[arg(long, short)]
        description: String,

        #[arg(long, short)]
        amount: usize,

        #[arg(long, short)]
        id: Ulid,
    },

    Delete {
        #[arg(long, short)]
        id: Ulid,
    },

    List,

    Summary {
        #[arg(long, short, value_parser = clap::value_parser!(u32).range(1..12))]
        month: Option<u32>,
    },
}
