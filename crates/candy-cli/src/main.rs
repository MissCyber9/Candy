use std::fs;

use candy_diagnostics::{Diagnostic, DiagnosticReport, Span};
use candy_parser::parse_file;
use candy_typecheck::typecheck;

fn print_usage() {
    eprintln!(
        "Candy 🍭\n\nUSAGE:\n  candy check [--agent] <file.candy>\n\nFLAGS:\n  --agent   Output diagnostics as JSON ONLY (stdout)\n"
    );
}

fn render_human(report: &DiagnosticReport) {
    for d in &report.diagnostics {
        eprintln!(
            "[{:?}] {}: {} ({}:{}:{}-{}:{})",
            d.severity,
            d.code,
            d.message,
            d.span.file,
            d.span.start_line,
            d.span.start_col,
            d.span.end_line,
            d.span.end_col
        );
        if let Some(fix) = &d.fix {
            eprintln!("  fix: replace `{}` with `{}`", fix.replace, fix.with);
        }
    }
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(2);
    }

    let cmd = args[1].as_str();
    if cmd != "check" {
        print_usage();
        std::process::exit(2);
    }

    let mut agent = false;
    let mut file: Option<String> = None;

    let rest = args.drain(2..).collect::<Vec<_>>();
    for a in rest {
        if a == "--agent" {
            agent = true;
        } else if a.starts_with('-') {
            eprintln!("Unknown flag: {}", a);
            print_usage();
            std::process::exit(2);
        } else {
            file = Some(a);
        }
    }

    let Some(path) = file else {
        eprintln!("Missing <file.candy>");
        print_usage();
        std::process::exit(2);
    };

    let src = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            let mut r = DiagnosticReport::new();
            r.push(Diagnostic::error(
                "io-read-failed",
                format!("Failed to read file: {e}"),
                Span::unknown(path.clone()),
            ));

            if agent {
                println!("{}", r.to_json_pretty());
            } else {
                render_human(&r);
            }
            std::process::exit(1);
        }
    };

    let mut report = DiagnosticReport::new();

    let program = match parse_file(&path, &src) {
        Ok(p) => p,
        Err(mut r) => {
            report.diagnostics.append(&mut r.diagnostics);
            if agent {
                println!("{}", report.to_json_pretty());
            } else {
                render_human(&report);
            }
            std::process::exit(1);
        }
    };

    if let Err(mut r) = typecheck(&program) {
        report.diagnostics.append(&mut r.diagnostics);
    }

    if agent {
        println!("{}", report.to_json_pretty());
    } else if report.is_ok() {
        eprintln!("ok");
    } else {
        render_human(&report);
    }

    std::process::exit(if report.is_ok() { 0 } else { 1 });
}
