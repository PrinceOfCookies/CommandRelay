use std::fs;
use std::process::Command;
use std::os::windows::process::CommandExt;
use rand::Rng;
use regex::Regex;

fn clearFile() {
    fs::write("E:\\Steam\\steamapps\\common\\GarrysMod\\garrysmod\\data\\fudgy_chat.txt", "").expect("Failed to clear the file");
}

fn sendConsoleCommand(command: &str) {
    let mut commands: Vec<String> = command.split("; ").map(|s| s.to_string()).collect();
    let length = commands.len();

    // Splitting the first command by the space
    let mut splitCommand: Vec<String> = commands[0].split(" ").map(|s| s.to_string()).collect();
    splitCommand[1] = format!(r#""{}"#, splitCommand[1]);
    commands[0] = format!("{} {}", splitCommand[0], splitCommand[1]); 

    commands[length - 1] = format!(r#"{}""#, commands[length - 1]);

    let finalCommand = format!("+{}", commands.join("; "));

    println!("Attempting to run: {}", finalCommand);


    // Run the command with gmod.exe
    let executable = "E:\\Steam\\steamapps\\common\\GarrysMod\\gmod.exe";
    let mut binding = Command::new(executable);
    let commandToRun = binding.arg("-hijack").raw_arg(finalCommand);

    println!("Running command: {:?}", commandToRun);

    // Wait 1.5 seconds before running the command
    std::thread::sleep(std::time::Duration::from_secs(1));
    commandToRun.spawn().expect("Failed to run command");

    clearFile();
    println!("Successfully ran command.");
}

// Reads the last line of the specified file.
fn checkLastLine() -> String {
    // Path to the file
    let filePath = "E:\\Steam\\steamapps\\common\\GarrysMod\\garrysmod\\data\\fudgy_chat.txt";
    // Read the file into a String
    let contents = fs::read_to_string(filePath).expect("Failed to read the file");

    // Split contents into lines and get the last one
    if let Some(lastLine) = contents.lines().last() {
        if lastLine.contains("[]") {
            String::from("")
        } else {
            lastLine.to_string()
        }
    } else {
        String::from("") // Return an empty string if the file is empty
    }
}

// Strips the chat type from a message, or extracts the name if `getName` is true.
fn stripChatType(input: &str, getName: bool) -> String {
    // Regular expression to match `[text]` or `(text)`
    let chatTypeRegex = Regex::new(r"[\[\(](.*?)[\]\)]").unwrap();

    if !getName {
        if input.is_empty() {
            return "local".to_string();
        }
        // Extract the chat type or return "local" if not found
        if let Some(capture) = chatTypeRegex.captures(&input[..10.min(input.len())]) {
            return capture.get(1).map_or("local".to_string(), |m| m.as_str().to_string());
        }
        return "local".to_string();
    }

    // Remove the chat type and trim the remaining string
    let stripped = chatTypeRegex.replace_all(input, "").trim().to_string();
    return stripped
}

fn _randomWeapon(second: bool, knife: bool) -> String {
    let mainGun = vec![
        "tfa_minigun",
        "tfa_contender",
    ]; // Main guns

    let secondGun = vec![
        "tfa_contender",
    ]; // Second guns

    let knives = vec![
        "tfa_knife",
    ];

    // If second and knife are false, return a random main gun
    if !second && !knife {
        return mainGun[rand::thread_rng().gen_range(0..mainGun.len())].to_string();
    }

    // If second is true and knife is false, return a random second gun
    if second && !knife {
        return secondGun[rand::thread_rng().gen_range(0..secondGun.len())].to_string();
    }

    // If knife is true, return a random knife
    if knife {
        return knives[rand::thread_rng().gen_range(0..knives.len())].to_string();
    }

    // If all else fails, return a random main gun
    return mainGun[rand::thread_rng().gen_range(0..mainGun.len())].to_string();
}

fn main() {
    loop {
        let lastLine = checkLastLine();

        let chatType = if !lastLine.is_empty() {
            stripChatType(&lastLine, false)
        } else {
            String::from("")
        };

        let namePlusMessage = if !lastLine.is_empty() {
            stripChatType(&lastLine, true)
        } else {
            String::from("")
        };

        let parts: Vec<&str> = namePlusMessage.split(": ").collect();

        let name = parts[0];
        let message = if parts.len() > 1 {
            parts[1]
        } else {
            "!NO_AVAILABLE_MESSAGE!"
        };

            
        let fileLastEdited = fs::metadata("E:\\Steam\\steamapps\\common\\GarrysMod\\garrysmod\\data\\fudgy_chat.txt").unwrap().modified().unwrap();
        let fileLastEdited = fileLastEdited.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs();

        println!("{} | [{}] {}: {}", fileLastEdited, chatType, name, message);

        match chatType.as_str() {
            "local" => {
                // If the name isnt "Ace Pandas PrinceOfCookies", ignore it
                if name != "Ace Pandas PrinceOfCookies" {
                    continue;
                }

                match message {
                    "event_ffa_ra" => {
                        sendConsoleCommand("ulx bring EVENT; say !confirm; ulx freeze EVENT; ulx strip EVENT; ulx stripjetpack EVENT; ulx eventweapon tfa_minigun EVENT; ulx eventweapon tfa_contender EVENT; ulx hp EVENT 400; ulx armor EVENT 200");
                    }
                    "test" => {
                        sendConsoleCommand("say hi; ulx asay test; ulx asay wrawrawr warawrwa");
                    }
                    _ => {
                        // This is just to handle unrecognized messages, like the default case in a switch statement!
                        clearFile();
                        continue;
                    }
                }
            }
            _ => {
                // This is just to handle unrecognized messages, like the default case in a switch statement!
                continue;
            }
        }
    }
}
