//! Agent of Empires - Terminal session manager for AI coding agents

use agent_of_empires::cli::{self, Cli, Commands};
use agent_of_empires::migrations;
use agent_of_empires::tui;
use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use std::fs::OpenOptions;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // log to file on TUI mode
    let _guard = if cli.command.is_none() {
        let log_file = {
            let cfg_dir = agent_of_empires::session::get_app_dir()?;
            std::fs::create_dir_all(&cfg_dir)?;

            // Open (or create) your log file, appending to it.
            let mut log_file_opts = OpenOptions::new();
            log_file_opts.create(true).append(true);
            log_file_opts.open(cfg_dir.join("aoe.log"))?
        };

        let (non_blocking, _guard) = tracing_appender::non_blocking(log_file);

        if std::env::var("AGENT_OF_EMPIRES_DEBUG").is_ok() {
            tracing_subscriber::fmt()
                .with_env_filter("agent_of_empires=debug")
                .with_writer(non_blocking)
                .init();
        }
        Some(_guard)
    } else {
        if std::env::var("AGENT_OF_EMPIRES_DEBUG").is_ok() {
            tracing_subscriber::fmt()
                .with_env_filter("agent_of_empires=debug")
                .init();
        }
        None
    };

    // Handle commands that don't need app data or migrations.
    // These work in read-only/sandboxed environments (e.g. Nix builds).
    match cli.command {
        Some(Commands::Completion { shell }) => {
            generate(shell, &mut Cli::command(), "aoe", &mut std::io::stdout());
            return Ok(());
        }
        Some(Commands::Init(args)) => return cli::init::run(args).await,
        Some(Commands::Tmux { command }) => {
            use cli::tmux::TmuxCommands;
            return match command {
                TmuxCommands::Status(args) => cli::tmux::run_status(args),
            };
        }
        Some(Commands::Sounds { command }) => return cli::sounds::run(command).await,
        Some(Commands::Uninstall(args)) => return cli::uninstall::run(args).await,
        _ => {}
    }

    let profile = cli.profile.unwrap_or_default();

    // TUI mode handles migrations with a spinner; CLI runs them silently
    if cli.command.is_some() {
        migrations::run_migrations()?;
    }

    match cli.command {
        Some(Commands::Add(args)) => cli::add::run(&profile, args).await,
        Some(Commands::List(args)) => cli::list::run(&profile, args).await,
        Some(Commands::Remove(args)) => cli::remove::run(&profile, args).await,
        Some(Commands::Status(args)) => cli::status::run(&profile, args).await,
        Some(Commands::Session { command }) => cli::session::run(&profile, command).await,
        Some(Commands::Group { command }) => cli::group::run(&profile, command).await,
        Some(Commands::Profile { command }) => cli::profile::run(command).await,
        Some(Commands::Worktree { command }) => cli::worktree::run(&profile, command).await,
        None => tui::run(&profile).await,
        _ => unreachable!(),
    }
}
