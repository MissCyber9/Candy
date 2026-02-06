use std::fs;

use candy_diagnostics::DiagnosticReport;
use candy_parser::parse_file;

fn print_usage() {
    eprintln!(
        "Candy 🍭\n\nUSAGE:\n  candy check [--agent] <file.candy>\n\nFLAGS:\n  --agent   Output diagnostics as JSON ONLY (stdout)\n"
    );
}

fn render_human(report: &DiagnosticReport) {
    for d in &report.diagnostics {
        eprintln!(
            "[{:?}] {}: {} ({}:{}-{}:{})",
            d.severity,
            d.code,
            d.message,
            d.span.file,
            d.span.start_line,
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

    // args[0] = binary
    if args.len() < 2 {
        print_usage();
        std::process::exit(2);
    }

    let cmd = args[1].as_str();

    if cmd != "check" {
        print_usage();
        std::process::exit(2);
    }

    // Parse flags + file
    let mut agent = false;
    let mut file: Option<String> = None;

    // consume after "check"
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
            if agent {
                // JSON-only even for IO errors
                let mut r = DiagnosticReport::new();
                r.push(candy_diagnostics::Diagnostic::error(
                    "io-read-failed",
                    format!("Failed to read file: {e}"),
                    candy_diagnostics::Span::unknown(path.clone()),
                ));
                println!("{}", r.to_json_pretty());
            } else {
                eprintln!("error: failed to read {}: {}", path, e);
            }
            std::process::exit(1);
        }
    };

    match parse_file(&path, &src) {
        Ok(_program) => {
            if agent {
                // JSON-only: empty diagnostics list
                let r = DiagnosticReport::new();
                println!("{}", r.to_json_pretty());
            } else {
                eprintln!("ok");
            }
            std::process::exit(0);
        }
        Err(report) => {
            if agent {
                println!("{}", report.to_json_pretty());
            } else {
                render_human(&report);
            }
            std::process::exit(1);
        }
    }
}
