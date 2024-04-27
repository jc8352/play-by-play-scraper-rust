use std::thread;
//use std::sync::{Arc, Mutex};
use reqwest;
use scraper;
use csv;

use std::time::Duration;
use std::env;

struct GamePlay {
	time_remaining: String,
	away_play: String,
	score: String,
	home_play: String
}

//each thread creates and modifies its own variables so there shouldn't be any data races. there aren't any shared mutable variable between threads
fn process_game(link: String) {
	thread::sleep(Duration::from_secs(5));

	let root_url = "https://www.basketball-reference.com";

	let response = reqwest::blocking::get(root_url.to_owned()+&link.clone());
	let html_content = response.unwrap().text().unwrap();
	let document = scraper::Html::parse_document(&html_content);

	let plays_selector = scraper::Selector::parse("div#all_pbp table#pbp tbody tr").unwrap();
	let plays = document.select(&plays_selector);

	let mut game_plays: Vec<GamePlay> = Vec::new();

	for play in plays {
		let time = play.select(&scraper::Selector::parse("td").unwrap())
			.next()
			.map(|t| t.text().collect::<String>());

		let away_play = play.select(&scraper::Selector::parse("td:nth-of-type(2)").unwrap())
			.next()
			.map(|t| t.text().collect::<String>());

		let score = play.select(&scraper::Selector::parse("td:nth-of-type(4)").unwrap())
			.next()
			.map(|t| t.text().collect::<String>());

		let home_play = play.select(&scraper::Selector::parse("td:nth-of-type(6)").unwrap())
			.next()
			.map(|t| t.text().collect::<String>());

		match (time, away_play, score, home_play) {
		    (Some(t), Some(ap), Some(s), Some(hp)) => {
		        let new_play = GamePlay {
		        	time_remaining: t,
		        	away_play: ap,
		        	score: s,
		        	home_play: hp,
		        };
		        game_plays.push(new_play);
		    },
		    (None, _, _, _) => println!("No time provided"),
		    (_, None, _, _) => println!("No away play provided"),
		    (_, _, None, _) => println!("No score provided"),
		    (_, _, _, None) => println!("No home play provided"),
		}
	}

	let parts: Vec<&str> = link.split(|c| c == '/' || c == '.').collect();

	let game_id = parts[3];
	println!("{}", game_id);
	let file_name = game_id.to_owned() + "_pbp.csv";
	let path = std::path::Path::new(&file_name);
	let mut writer = csv::Writer::from_path(path).unwrap();

	writer
		.write_record(&["TimeRemaining", "AwayPlay", "Score", "HomePlay"])
		.unwrap();

	for play in game_plays {
		writer.write_record(&[play.time_remaining, 
					play.away_play, 
					play.score,
					play.home_play]).unwrap();
	}

	writer.flush().unwrap();

	println!("{link}");
}

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 4 {
        println!("Usage: program <month> <day> <year>");
        return;
    }

    // Parse the command line arguments into integers
    let month = match args[1].parse::<i32>() {
        Ok(m) => m,
        Err(_) => {
            println!("Invalid month input");
            return;
        }
    };

    let day = match args[2].parse::<i32>() {
        Ok(d) => d,
        Err(_) => {
            println!("Invalid day input");
            return;
        }
    };

    let year = match args[3].parse::<i32>() {
        Ok(y) => y,
        Err(_) => {
            println!("Invalid year input");
            return;
        }
    };

    println!("getting games for {}/{}/{}", month, day, year);


	let url = format!("https://www.basketball-reference.com/boxscores/?month={}&day={}&year={}", month, day, year);


	println!("{}", url);

	let response = reqwest::blocking::get(url);
	let html_content = response.unwrap().text().unwrap();

	let document = scraper::Html::parse_document(&html_content);


	let game_links_selector = scraper::Selector::parse("div.game_summaries p.links").unwrap();
	let game_links = document.select(&game_links_selector);
	let mut pbp_links: Vec<String> = Vec::new(); 
	for game_link in game_links {
		let pbp_link = game_link.select(&scraper::Selector::parse("a:nth-of-type(2)").unwrap())
				.next()
				.and_then(|a| a.value().attr("href"))
				.map(str::to_owned);

		match pbp_link {
        	Some(s) => pbp_links.push(s),
        	None => println!("No string provided"),
    	}


	}

	let mut handles = vec![];


	for link in pbp_links {
		let cloned_link = link.clone();
		let handle = thread::spawn(move || {
			process_game(cloned_link);
		});
		handles.push(handle);
	}

	for handle in handles {
        handle.join().unwrap();
    }

    println!("finished");

 

}
