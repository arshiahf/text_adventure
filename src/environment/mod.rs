// Generic module for environment variables and functions

use user_input::requests::{request_input_string, request_input_to_vec};
use std::fs::{File, DirBuilder};

struct Room
{
    id: String,
    accepted_commands: Vec<String>,
}

pub fn create_rooms(root_dir:String)
{
    let mut more_rooms: bool = true;
    while more_rooms
    {
        let mut room_name: String = String::new();
        request_input_string("Please type in the name of a room: ", &mut room_name);
        let mut accepted_commands: Vec<String> = Vec::new();
        request_input_to_vec("Please type all accepted commands for this room, separated by spaces: ", &mut accepted_commands);
        println!("There is now a folder entitled {}, please fill in the returns for all locally accepted commands.", room_name);
        let mut builder = DirBuilder::new();
        let path = &(root_dir.clone() + &room_name);
        builder.recursive(true).create(path).unwrap();

        let mut continuevec:Vec<char> = Vec::new();
        request_input_to_vec("Would you like to make more rooms y/n? ", &mut continuevec);
        match continuevec[0]
        {
            'y' => more_rooms = true,
            _ => more_rooms = false,
        }
    }
}

fn initialize_room(name:String)
{

}
