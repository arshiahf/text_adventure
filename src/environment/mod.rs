// Generic module for environment variables and functions

use user_input::requests::{
    request_input_string,
    request_input_to_vec,
    new_directory,
    new_files
};
use std::fs::{File};
use std::process::{exit};

struct Room
{
    id: String,
    accepted_commands: Vec<String>,
}

pub fn create_rooms()
{
    let mut root_dir:String = String::new();
    request_input_string("Please input root directory with no spaces or slashes: ", &mut root_dir);
    let mut commands:Vec<String> = Vec::new();

    let mut more_rooms: bool = true;
    while more_rooms
    {
        initialize_room(root_dir.clone(), commands);

        let mut continuevec:Vec<char> = Vec::new();
        request_input_to_vec("Would you like to make more rooms y/n? ", &mut continuevec);
        match continuevec[0]
        {
            'y'|"yes" => more_rooms = true,
            _ => more_rooms = false,
        }
    }
    let commands_dir = root_dir.clone() + "/commands/";
    new_files(commands_dir, commands);
    for
}

fn initialize_room(root_dir:String, commands:Vec<String>) -> Room
{
    let mut room_name:String = String::new();
    let rooms_dir = root_dir.clone() + "/rooms/";
    request_input_string("Please type in the name of a room: ", &mut room_name);
    new_directory(rooms_dir.clone(), room_name.clone());

    let mut accepted_commands:Vec<String> = Vec::new();
    request_input_to_vec("Please type all accepted commands for this room, separated by spaces: ", &mut commands_list);
    for command in commands_list
    {
        if !commands.contains(command)
        {
            commands.push(command);
        }
    }


    println!("There is now a folder entitled {}, please fill in the returns for all locally accepted commands.", room_name);

    let mut new_room = struct Room{id:room_name, accepted_commands:commands_list};
    new_room
}
