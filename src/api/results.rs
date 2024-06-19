use crate::testez::{ReporterChildNode, ReporterOutput, ReporterStatus};
use axum::{http::StatusCode, Json};
use console::style;
use serde_json::Value;
use std::{fmt::Write, process::exit, time::Duration};
use tokio::{spawn, time::sleep};

fn print_children(children: Vec<ReporterChildNode>, indent: u32) -> bool {
    let mut success = true;

    for child in children {
        let styled_phrase = match child.status {
            ReporterStatus::Success => style(format!("✓ {}", child.plan_node.phrase)).green(),
            ReporterStatus::Failure => {
                success = false;
                style(format!("X {}", child.plan_node.phrase)).red()
            }
            ReporterStatus::Skipped => style(format!("↪ {}", child.plan_node.phrase)).blue(),
        };
        println!("{}{}", " ".repeat(indent as usize), styled_phrase);

        for error in child.errors {
            let indented_error: String = error.split('\n').fold(String::new(), |mut acc, line| {
                writeln!(
                    acc,
                    "{:indent$}{}",
                    " ",
                    line,
                    indent = (indent + 2) as usize
                )
                .unwrap();
                acc
            });
            print!("{}", indented_error);
        }

        if !print_children(child.children, indent + 2) {
            success = false;
        }
    }
    success
}

pub async fn results(Json(body): Json<Value>) -> StatusCode {
    let output: ReporterOutput =
        serde_json::from_value(body).expect("Failed to parse JSON from plugin");

    let success = print_children(output.children, 0);

    println!();

    println!("{} {}", style("✓ Success:").green(), output.success_count);
    println!("{} {}", style("X Failure:").red(), output.failure_count);
    println!("{} {}", style("↪ Skip:").blue(), output.skipped_count);

    // This is mildly cursed - we need to return a status code, but we also need
    // to exit the progam so that we don't keep receiving results.
    spawn(async move {
        sleep(Duration::from_millis(100)).await;

        exit(if success { 0 } else { 1 });
    });

    StatusCode::OK
}
