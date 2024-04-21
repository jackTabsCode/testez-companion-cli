use crate::testez::{ReporterChildNode, ReporterOutput, ReporterStatus};
use axum::{http::StatusCode, Json};
use console::style;
use serde_json::Value;
use std::{process::exit, time::Duration};
use tokio::{spawn, time::sleep};

fn print_children(children: Vec<ReporterChildNode>, indent: u32) {
    for child in children {
        let styled_phrase = match child.status {
            ReporterStatus::Success => style(format!("✓ {}", child.plan_node.phrase)).green(),
            ReporterStatus::Failure => style(format!("X {}", child.plan_node.phrase)).red(),
            ReporterStatus::Skipped => style(format!("↪ {}", child.plan_node.phrase)).blue(),
        };
        println!("{}{}", " ".repeat(indent as usize), styled_phrase);

        for error in child.errors {
            // Thanks Copilot!
            let indented_error: String = error.split('\n').fold(String::new(), |mut acc, line| {
                acc.push_str(&format!("{}{}\n", " ".repeat((indent + 2) as usize), line));
                acc
            });
            print!("{}", indented_error);
        }

        print_children(child.children, indent + 2);
    }
}

pub async fn results(Json(body): Json<Value>) -> StatusCode {
    let output: ReporterOutput =
        serde_json::from_value(body).expect("Failed to parse JSON from plugin");

    print_children(output.children, 0);

    println!();

    println!("{} {}", style("✓ Success:").green(), output.success_count);
    println!("{} {}", style("X Failure:").red(), output.failure_count);
    println!("{} {}", style("↪ Skip:").blue(), output.skipped_count);

    // This is really cursed - we need to return a status code, but we also need
    // to exit the progam so that we don't keep receiving results.
    spawn(async move {
        sleep(Duration::from_millis(100)).await;

        exit(0);
    });

    StatusCode::OK
}
