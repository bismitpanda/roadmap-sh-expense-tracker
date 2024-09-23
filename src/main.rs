mod cli;
mod expense;

use clap::Parser;
use cli::Cli;
use expense::Expenses;

fn main() {
    let cli = Cli::parse();
    let mut expenses = Expenses::read();

    match cli.subcommand {
        cli::Subcommands::Add {
            description,
            amount,
        } => {
            let id = expenses.add(description, amount);

            println!("Successfully added with ID {id}")
        }

        cli::Subcommands::List => {
            let mut table = comfy_table::Table::new();

            table.set_header(["Id", "Description", "Amount"]);

            for expense in expenses.list() {
                table.add_row([
                    expense.id.to_string(),
                    expense.description.clone(),
                    expense.amount.to_string(),
                ]);
            }

            println!("{table}");
        }

        cli::Subcommands::Delete { id } => {
            expenses.delete(id);

            println!("Successfully deleted expense with ID {id}")
        }

        cli::Subcommands::Update {
            description,
            amount,
            id,
        } => {
            expenses.update(id, description, amount);

            println!("Successfully updated expense with ID {id}")
        }

        cli::Subcommands::Summary { month } => {
            let summary = expenses.summary(month);

            if let Some(month) = month {
                println!(
                    "Total expenses for the month {} is ${summary}",
                    MONTHS[month as usize - 1]
                )
            } else {
                println!("Total expenses is ${summary}")
            }
        }
    }
}

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
