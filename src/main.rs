// use clap::Parser;
// use color_eyre::Result;
// use comfy_table::Table;
// use core::time;
// use image::{Frame, ImageReader};
// use ratatui::{
// 	buffer::Buffer,
// 	crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
// 	layout::{Constraint, Layout, Rect},
// 	style::{
// 		palette::{material::BLUE, tailwind::SLATE},
// 		Color, Modifier, Style, Stylize,
// 	},
// 	symbols,
// 	text::Line,
// 	widgets::{
// 		Block, Borders, Clear, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
// 		StatefulWidget, Widget,
// 	},
// 	DefaultTerminal, Terminal,
// };
// use ratatui_image::{picker::Picker, protocol::StatefulProtocol, Resize, StatefulImage};
// use serde::Deserialize;
// use serde_json::Value;
// use std::{
// 	fs::{self, read_to_string},
// 	io::prelude::*,
// 	ops::Index,
// 	path::{self, Path},
// 	thread::{sleep, sleep_ms},
// };

// #[derive(Parser)]
// struct Args {
// 	/// Display the current drop rate table of the game
// 	#[clap(short, long)]
// 	drop_rate: bool,

// 	#[clap(short, long, value_delimiter = ' ', num_args = 1..)]
// 	cards: Option<Vec<String>>,
// }

// fn main() {
// 	let args = Args::parse();

// 	if args.drop_rate {
// 		display_drop_rate();
// 	}

// 	if let Some(cards) = args.cards {
// 		let data = read_csv("./src/csv/genetic_apex.csv");
// 		let mut data: Vec<Vec<String>> = data
// 			.lines()
// 			.map(|line| line.split(',').map(|s| s.to_string()).collect())
// 			.collect::<Vec<Vec<String>>>();

// 		for card in cards {
// 			if card == "all" {
// 				for line in data.clone() {
// 					data[line[1]
// 						.split_whitespace()
// 						.last()
// 						.unwrap()
// 						.parse::<usize>()
// 						.unwrap() - 1][0] = "TRUE".to_string();
// 				}
// 			} else {
// 				for line in data.clone() {
// 					if line[2] == card {
// 						data[line[1]
// 							.split_whitespace()
// 							.last()
// 							.unwrap()
// 							.parse::<usize>()
// 							.unwrap() - 1][0] = "TRUE".to_string();
// 						break;
// 					}
// 				}
// 			}
// 		}

// 		let mut file = std::fs::File::create("./src/csv/genetic_apex.csv").unwrap();
// 		file.write_all(
// 			data.iter()
// 				.map(|line| line.join(",") + "\n")
// 				.collect::<String>()
// 				.as_bytes(),
// 		)
// 		.unwrap();
// 	}
// }

// fn read_csv(path: &str) -> String {
// 	read_to_string(path).unwrap_or("Unable to read file".to_string())
// }

// fn display_drop_rate() {
// 	let drop_rate = read_csv("./src/csv/drop_rate.csv");

// 	let mut table = Table::new();

// 	table.set_header(drop_rate.lines().next().unwrap().split(','));

// 	for line in drop_rate.lines().skip(1) {
// 		let vec_line = line.split(',');

// 		table.add_row(vec_line);
// 	}

// 	println!("{}", table);
// }

// struct App {
//  should_exit: bool,
//  pokemon_list: PokemonList,
//  yay: u32,
// }

// impl App {
//  fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
//      while !self.should_exit {
//          terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
//          if let Event::Key(key) = event::read()? {
//              self.handle_key(key);
//          };
//      }
//      Ok(())
//  }

//  fn handle_key(&mut self, key: KeyEvent) {
//      if key.kind != KeyEventKind::Press {
//          return;
//      }
//      match key.code {
//          KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
//          KeyCode::Char('h') | KeyCode::Left => self.select_none(),
//          KeyCode::Char('j') | KeyCode::Down => self.select_next(),
//          KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
//          _ => {}
//      }
//  }
// }

// impl Widget for &mut App {
//  fn render(self, area: Rect, buf: &mut Buffer) {
//      let [_, main_area, _] = Layout::vertical([
//          Constraint::Length(2),
//          Constraint::Fill(1),
//          Constraint::Length(1),
//      ])
//      .areas(area);

//      let [list_area, item_area] =
//          Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
//              .areas(main_area);
//      self.render_list(list_area, buf);
//      self.render_selected_item(item_area, buf);
//  }
// }

// impl App {
//  fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
//      let block = Block::new()
//          .title(Line::raw("Pokemon List").centered())
//          .borders(Borders::TOP)
//          .border_set(symbols::border::EMPTY)
//          .border_style(Style::new().fg(SLATE.c100).bg(BLUE.c800))
//          .bg(SLATE.c950);

//      // Iterate through all elements in the `items` and stylize them.
//      let items: Vec<ListItem> = self
//          .pokemon_list
//          .items
//          .iter()
//          .enumerate()
//          .map(|(i, todo_item)| {
//              let color = alternate_colors(i);
//              ListItem::from(todo_item).bg(color)
//          })
//          .collect();

//      // Create a List from all list items and highlight the currently selected one
//      let list = List::new(items)
//          .block(block)
//          .highlight_style(Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD))
//          .highlight_symbol(">")
//          .highlight_spacing(HighlightSpacing::Always);

//      // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
//      // same method name `render`.
//      StatefulWidget::render(list, area, buf, &mut self.pokemon_list.state);
//  }

//  fn render_selected_item(&mut self, area: Rect, buf: &mut Buffer) {
//      Clear.render(area, buf);
//      if let Some(index) = self.pokemon_list.state.selected() {
//          let pokemon = self.pokemon_list.items.index(index);
//          self.yay += 1;
//          let dyn_img = ImageReader::open(format!("img/{}.png", pokemon.name))
//              .unwrap()
//              .decode()
//              .unwrap();
//          let mut picker = Picker::from_fontsize((8, 12));
//          let mut image_protocol = picker.new_resize_protocol(dyn_img);
//          let image = StatefulImage::new(None).resize(Resize::Fit(None));
//          StatefulWidget::render(image, area, buf, &mut image_protocol);
//      } else {
//          Paragraph::new(format!("none {}", self.yay)).render(area, buf);
//          self.yay += 1;
//      }
//  }

//  fn select_none(&mut self) {
//      self.pokemon_list.state.select(None);
//  }

//  fn select_next(&mut self) {
//      self.pokemon_list.state.select_next();
//  }
//  fn select_previous(&mut self) {
//      self.pokemon_list.state.select_previous();
//  }
// }

// #[derive(Debug, Deserialize, Clone)]
// struct Pokemon {
//  name: String,
//  number: u16,
// }

// #[derive(Clone)]
// struct PokemonList {
//  items: Vec<Pokemon>,
//  state: ListState,
// }

// struct PokemonCard {
//  image_state: StatefulProtocol,
//  dimensions: (u32, u32),
// }

// impl From<&Pokemon> for ListItem<'_> {
//  fn from(value: &Pokemon) -> Self {
//      ListItem::new(Line::styled(value.name.to_string(), SLATE.c200))
//  }
// }

// trait ToPokemonList {
//  fn to_pokemon_list(&self) -> PokemonList;
// }

// impl ToPokemonList for Value {
//  fn to_pokemon_list(&self) -> PokemonList {
//      let mut res = PokemonList {
//          items: Vec::new(),
//          state: ListState::default(),
//      };
//      for (key, number) in self.as_object().unwrap() {
//          res.items.push(Pokemon {
//              name: key.clone(),
//              number: number.to_string().parse::<u16>().unwrap(),
//          });
//      }
//      res
//  }
// }

// impl Default for App {
//  fn default() -> Self {
//      let all_pokemons_json: Value =
//          serde_json::from_reader(fs::File::open("src/name-to-number.json").unwrap()).unwrap();
//      let all_pokemon = all_pokemons_json.to_pokemon_list();
//      initialize_images(all_pokemon.clone());
//      Self {
//          should_exit: false,
//          pokemon_list: all_pokemon,
//          yay: 0,
//      }
//  }
// }
// fn initialize_images(pokemon_list: PokemonList) {
//  for pokemon in pokemon_list.items {
//      let file_name = format!("img/{}.png", pokemon.name);
//      if !Path::new(&file_name).exists() {
//          let mut file = std::fs::File::create(format!("img/{}.png", pokemon.name)).unwrap();
//          reqwest::blocking::get(format!(
//              "https://img.gamewith.net/article_tools/pokemon-tcg-pocket/gacha/m{}.png",
//              pokemon.number
//          ))
//          .unwrap()
//          .copy_to(&mut file)
//          .unwrap();
//      }
//  }
// }
// const fn alternate_colors(i: usize) -> Color {
//  if i % 2 == 0 {
//      SLATE.c950
//  } else {
//      SLATE.c900
//  }
// }
use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{crossterm, DefaultTerminal, Frame};

fn main() -> Result<()> {
	color_eyre::install()?;
	let terminal = ratatui::init();
	let result = run(terminal);
	ratatui::restore();
	result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
	loop {
		terminal.draw(|frame| frame.render_widget("hello world", frame.area()))?;
		if matches!(event::read()?, Event::Key(_)) {
			break Ok(());
		}
	}
}

fn render(frame: &mut Frame) {
	frame.render_widget("hello world", frame.area());
}
