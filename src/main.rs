use std::env;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::io::stdin;
use std::io::stdout;

use reqwest::get;
use serde_json::Value;

async fn get_nhl_stats(player: &str) -> Result<(), Box<dyn std::error::Error>> {
  let url = format!("https://statsapi.web.nhl.com/api/v1/people/{}/stats?stats=gameLog", player);
  let resp = reqwest::get(&url).await?.json::<serde_json::Value>().await?;

  let stats = resp["stats"][0]["splits"].as_array().unwrap();
  for stat in stats {
      let season = stat["season"].as_str().unwrap();
      let goals = stat["stat"]["goals"].as_u64().unwrap();
      let assists = stat["stat"]["assists"].as_u64().unwrap();
      let points = stat["stat"]["points"].as_u64().unwrap();
      println!("{}: {} G, {} A, {} P", season, goals, assists, points);
  }

  Ok(())
}

fn main() { 
  loop {
  
  print!("| ");
  stdout().flush();
  

  let mut input = String::new();
  stdin().read_line(&mut input).unwrap();

  let mut commands = input.trim().split(" | ").peekable();
  let mut previous_command: Option<String> = None;
  
  for command in commands {

    let mut parts = input.trim().split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;

    match command {
      "cd" => {
        let dir = args.peekable().peek().map_or("/", |x| *x);
        let root = Path::new(dir);
        if let Err(e) = env::set_current_dir(root) {
          eprintln!("{}", e);
        }
      },
      "exit" => return,
      "nhl-stats" => {
        if let Some(player) = args.peekable().peek().copied() {
          match tokio::runtime::Runtime::new().unwrap().block_on(get_nhl_stats(player)) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
          };
        } else {
          eprintln!("requires a player id")
        }
        previous_command = Some(command.to_string());
      },
      command => {
        let child = Command::new(command)
          .args(args)
          .spawn();
          
        match child {
          Ok(mut child) => { let _ = child.wait(); },
          Err(e) => eprintln!("{}", e),
        };
      }
    }
   }
  }
}