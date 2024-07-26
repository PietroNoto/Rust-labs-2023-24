use std::{any::Any, fs::OpenOptions, io::{Read, Write}, process::exit};
use battleboats::{Board, Boat};
use clap::Parser;


#[derive(Parser, Debug)]
struct Args
{
    #[arg(long)]
    new: bool,

    #[arg(long)]
    add: bool,

    #[arg(long)]
    file: String,

    #[arg(long)]
    boats: Option<String>,

    #[arg(long)]
    boat: Option<String>,

    #[arg(long)]
    start: Option<String>
}

fn main() 
{    
    let b = Boat::Vertical(1);
    
    
    let args = Args::parse();

    let rf = OpenOptions::new()
        .read(true)
        .write(true)
        .create(args.new)
        .open(args.file);

    match rf
    {
        Ok(mut file) =>
        {
            if args.new
            {
                if args.boats.is_none()
                {
                    println!("Boats missing!");
                    exit(1);
                }
                else 
                {
                    let boats = args.boats.unwrap()
                        .split(",")
                        .map(|b| b.parse::<u8>().unwrap())
                        .collect::<Vec<u8>>();
                    let board = Board::new(&boats);
                    file.write(board.to_string().as_bytes());
                }
            }

            else if args.add
            {
                if args.boat.is_none()
                {
                    println!("No boat to add!");
                    exit(1);
                }
                if args.start.is_none()
                {
                    println!("Invalid coordinates.");
                    exit(1);
                }
                let boat_params = args.boat.unwrap()
                    .chars()
                    .collect::<Vec<char>>();
                if boat_params.len() != 2
                {
                    println!("Boat format not accepted.");
                    exit(1);
                }
                let direction = boat_params[1].to_ascii_lowercase();
                if direction != 'h' && direction != 'v'
                {
                    println!("Provide a valid direction: H (horizontal) or V (vertical).");
                    exit(1);
                }
                let length = boat_params[0] as usize;
                if length < 1 || length > 4
                {
                    println!("Boat length not valid.");
                    exit(1);
                }
                if args.start.is_none()
                {
                    println!("Coordinates not valid.");
                    exit(1);
                }
                let xy = args.start.unwrap()
                    .split(",")
                    .map(|val| val.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                let boat: Boat;
                if direction == 'h'
                {
                    boat = Boat::Horizontal(length);
                }
                else
                {
                    boat = Boat::Vertical(length);
                }

                let mut board_string = String::new();
                file.read_to_string(&mut board_string);

                let mut board = Board::from(board_string);
                board.add_boat(boat, (xy[0], xy[1]));
            }
        }, 
        Err(e) => println!("{}", e.to_string())
    }
}
