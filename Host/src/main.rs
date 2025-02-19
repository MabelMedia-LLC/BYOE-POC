#![allow(nonstandard_style)]

use std::{
    fs, io::{
        stdin, stdout, BufRead, Write
    }, process::Command
};

use byteorder::{WriteBytesExt, LE};

enum CommandMessage {
    DosCommand(String)
}

impl CommandMessage {
    pub fn Write(&self) -> Option<()> {
        let mut Buffer = Vec::new();
        match self {
            Self::DosCommand(Command) => {
                Buffer.write_u32::<LE>(Command.len() as u32).ok()?;
                Buffer.write_all(&Command.bytes().collect::<Vec<u8>>()).ok()?;
            }
        }
        fs::write(&format!("C:\\Users\\{}\\AppData\\Local\\Temp\\RuntimeMessage.dat", whoami::username()), Buffer).ok()?;
        Some(())
    }
}

// dosbox-x -defaultconf -defaultmapper -nopromptfolder -exit -fastlaunch -silent -hostrun -c "IMGMOUNT A: Floppy.img" -c "A:\RUNTIME.EXE"
// dosbox-x -defaultconf -defaultmapper -nopromptfolder -hostrun -fastlaunch -c "IMGMOUNT A: Floppy.img" -c "A:\RUNTIME.EXE"

fn main() {
    let mut CommandRaw = String::new();
    let mut CurrentDir = "Z:\\";
    let mut Arguments = Vec::new();
    Command::new("dosbox-x").args(["-defaultconf", "-defaultmapper", "-nopromptfolder", "-hostrun", "-fastlaunch", "-c", "\"IMGMOUNT A: Floppy.img\"", "-c", "\"A:\\RUNTIME.EXE\""]);
    loop {
        CommandRaw.clear();
        print!("BYOE | {}>", CurrentDir);
        stdout().flush().expect("Failed To Flush StdOut!");
        stdin().lock().read_line(&mut CommandRaw).expect("Failed To Read StdIn!");
        CommandRaw = CommandRaw.trim().to_owned();
        Arguments = CommandRaw.split(" ").collect();
        if Arguments.is_empty() {
            continue
        }
        let Message = match Arguments[0].to_lowercase().as_str() {
            "dos-cmd" => {
                Some(CommandMessage::DosCommand(CommandRaw.strip_prefix("dos-cmd ").unwrap().to_owned()))
            }, _ => {
                eprintln!("\"{}\" Isn't A Valid Command.", Arguments[0]);
                None
            }
        };
        if let Some(Message) = Message {
            Message.Write().expect("Failed To Write Command Message File.");
        }
    }
}