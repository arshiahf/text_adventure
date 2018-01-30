// Generic module for environment variables and functions

use user_input::requests::{
    request_input_string,
    request_input_to_vec,
    new_directories_files
};
use std::fs::{File};
use std::io::Write;
use std::process::{exit};

#[derive(Clone)]
struct Room
{
    id: String,
    accepted_commands: Vec<String>,
    dir_path: String,
}

// Used to initially create all rooms and skeleton commands for a text adventure
// Only use in debug build of text adventure. Not for release build
pub fn create_rooms()
{
    let mut root_dir:String = String::new();
    request_input_string("Please input root directory with no spaces or slashes: ", &mut root_dir);
    let mut commands:Vec<String> = Vec::new();
    let mut rooms:Vec<Room> = Vec::new();

    let mut more_rooms: bool = true;
    while more_rooms
    {
        rooms.push(initialize_room());

        let mut continuevec:Vec<String> = Vec::new();
        request_input_to_vec("Would you like to make more rooms y/n? ", &mut continuevec);
        match continuevec[0].as_str()
        {
            "y"|"yes" => more_rooms = true,
            _ => more_rooms = false,
        }
    }

    for room in rooms
    {
        new_directories_files(root_dir.clone(), vec!(room.id.clone()), String::from("dir"));
        create_commands(root_dir.clone(), room.clone(), &mut commands);
    }
}

/* Initializes the room by aggregating list of all commands and creating a new room for the list of
 all rooms */
fn initialize_room() -> Room
{
    let mut room_name:String = String::new();
    request_input_string("Please type in the name of a room: ", &mut room_name);

    let mut commands_list:Vec<String> = Vec::new();
    request_input_to_vec("Please type all accepted commands for this room, separated by spaces: ",
        &mut commands_list);

    let new_room = Room{id:room_name, accepted_commands:commands_list};
    new_room
}

/* Initializes a command by creating a skeleton command in the global commands directory if the
command does not exist, then places a reference JSON document in the specific room's
 subdirectory */
fn create_commands(root_dir:String, room:Room, commands:&mut Vec<String>)
{

    for command in room.accepted_commands.clone()
    {

        // Skeleton JSON document to describe returns for command
        let command_return_skeleton:String = String::from(
            "Test"
        );
        if !commands.contains(&command.clone())
        {
            // Skeleton function for new command files
            let mut command_function_skeleton:String = String::from("fn ");
            command_function_skeleton = command_function_skeleton + &command.clone();
            

            commands.push(command);
            let commands_dir:String = String::from(root_dir.clone() + "/commands/");
            match File::create(commands_dir.as_str())
            {
                Ok(mut buf) => {
                    match buf.write(command_function_skeleton.clone().as_bytes())
                    {
                        Ok(_) =>match buf.flush(){
                                Ok(_) => {}
                                Err(err) => {
                                    eprintln!("{:?}", err);
                                    exit(1);
                                }
                            }
                        Err(err) => {
                            eprintln!("{:?}", err);
                            exit(1);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("{:?}", err);
                    exit(1);
                }
            }
        }
        let command_dir:String = String::from(root_dir.clone() + "/rooms/" + room.id.as_str().clone());
        match File::create(command_dir.as_str())
        {
            Ok(mut buf) => {
                match buf.write(command_return_skeleton.clone().as_bytes())
                {
                    Ok(_) =>match buf.flush(){
                            Ok(_) => {}
                            Err(err) => {
                                eprintln!("{:?}", err);
                                exit(1);
                            }
                        }
                    Err(err) => {
                        eprintln!("{:?}", err);
                        exit(1);
                    }
                }
            }
            Err(err) => {
                eprintln!("{:?}", err);
                exit(1);
            }
        }
    }

}
