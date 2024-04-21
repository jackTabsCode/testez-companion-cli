use axum::Json;
use console::style;
use serde_json::Value;
use std::process::exit;

use crate::testez::{ReporterChildNode, ReporterOutput, ReporterStatus};

fn print_children(children: Vec<ReporterChildNode>, indent: usize) {
    for child in children {
        let styled_phrase = match child.status {
            ReporterStatus::Success => style(format!("✓ {}", child.plan_node.phrase)).green(),
            ReporterStatus::Failure => style(format!("X {}", child.plan_node.phrase)).red(),
            ReporterStatus::Skipped => style(format!("↪ {}", child.plan_node.phrase)).blue(),
        };
        println!("{}{}", " ".repeat(indent), styled_phrase);

        for error in child.errors {
            // Thanks Copilot!
            let indented_error: String = error.split('\n').fold(String::new(), |mut acc, line| {
                acc.push_str(&format!("{}{}\n", " ".repeat(indent + 2), line));
                acc
            });
            print!("{}", indented_error);
        }

        print_children(child.children, indent + 2);
    }
}

pub async fn results(Json(body): Json<Value>) {
    let output: ReporterOutput = serde_json::from_value(body).unwrap();

    print_children(output.children, 0);

    println!();

    println!("{} {}", style("✓ Success:").green(), output.success_count);
    println!("{} {}", style("X Failure:").red(), output.failure_count);
    println!("{} {}", style("↪ Skip:").blue(), output.skipped_count);

    exit(0);
}
