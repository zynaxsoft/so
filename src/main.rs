use clap::{App, AppSettings, Arg};

mod config;
mod stackexchange;

use config::Config;
use stackexchange::StackExchange;

// TODO maybe consts for these keywords?

// TODO pull defaults from config file
// TODO --set-api-key KEY
// TODO --update-sites
// TODO --install-filter-key --force
// TODO --sites plural
// TODO --add-site (in addition to defaults)
//?TODO --set-default-opt opt val # e.g. --set-default-opt sites site1;site2;site3
fn mk_app<'a, 'b>() -> App<'a, 'b> {
    App::new("so")
        .setting(AppSettings::ColoredHelp)
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("list-sites")
                .long("list-sites")
                .help("Print available StackExchange sites"),
        )
        .arg(
            Arg::with_name("site")
                .long("site")
                .short("s")
                .multiple(true)
                .number_of_values(1)
                .takes_value(true)
                .default_value("stackoverflow")
                .help("StackExchange site code to search"), // TODO sites plural
        )
        .arg(
            Arg::with_name("limit")
                .long("limit")
                .short("l")
                .number_of_values(1)
                .takes_value(true)
                .default_value("1")
                .validator(|s| s.parse::<u32>().map_err(|e| e.to_string()).map(|_| ()))
                .help("Question limit per site query")
                .hidden(true), // TODO unhide once more than just --lucky
        )
        .arg(
            Arg::with_name("lucky")
                .long("lucky")
                .help("Print the top-voted answer of the most relevant question")
                .hidden(true), // TODO unhide
        )
        .arg(
            Arg::with_name("query")
                .multiple(true)
                .index(1)
                .required(true)
                .required_unless("list-sites"),
        )
}

fn main() {
    let matches = mk_app().get_matches();

    // TODO merge config from ArgMatch
    let se = StackExchange::new(Config {
        api_key: String::from("8o9g7WcfwnwbB*Qp4VsGsw(("),
        filter: String::from("0euqgThy5XMKqGfXzPS_nVSuunbQUZLlX7OuNJSlfvlW4"),
        limit: 1,
        site: String::from("stackoverflow"),
    });

    (|| -> Option<_> {
        let q = matches
            .values_of("query")?
            .into_iter()
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", q);
        let que = se.search(&q).ok()?;
        let ans = que.first()?.answers.first()?;
        println!("{}", ans.body);
        Some(())
    })();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli() {
        let m = mk_app().get_matches_from(vec![
            "so", "--site", "meta", "how", "do", "I", "exit", "Vim",
        ]);
        assert_eq!(m.value_of("site"), Some("meta"));
        assert_eq!(
            m.values_of("query").unwrap().collect::<Vec<_>>().join(" "),
            "how do I exit Vim"
        );
    }
}