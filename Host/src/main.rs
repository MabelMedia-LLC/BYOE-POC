#![allow(nonstandard_style)]

use std::{
    fs, io::{
        stdin, stdout, BufRead, Write
    }, process::{
        exit, Command
    }
};

use byteorder::{
    WriteBytesExt, LE
};

enum CommandMessage {
    DosCommand(String),
    Exit
}

impl CommandMessage {
    pub fn Write(&self) -> Option<()> {
        let mut Buffer = Vec::new();
        match self {
            Self::DosCommand(Command) => {
                Buffer.push(0);
                Buffer.write_u16::<LE>(Command.len() as u16).ok()?;
                Buffer.write_all(&Command.bytes().collect::<Vec<u8>>()).ok()?;
            }, Self::Exit => {
                Buffer.push(1);
            }
        }
        fs::write(&format!("C:\\Users\\{}\\AppData\\Local\\Temp\\RuntimeMessage.dat", whoami::username()), Buffer).ok()?;
        Some(())
    }
}

// dosbox-x -defaultconf -defaultmapper -nopromptfolder -exit -fastlaunch -silent -hostrun -c "IMGMOUNT A: Floppy.img" -c "A:\RUNTIME.EXE"
// dosbox-x -defaultconf -defaultmapper -nopromptfolder -exit -fastlaunch -hostrun -c "IMGMOUNT A: Floppy.img" -c "A:\RUNTIME.EXE"

fn main() {
    let mut CommandRaw = String::new();
    let mut CurrentDir = "A:\\";
    let mut Arguments: Vec<&str>;
    Command::new("dosbox-x").args([
        "-defaultconf", "-defaultmapper", "-nopromptfolder", "-exit", "-fastlaunch", "-hostrun", "-c", "IMGMOUNT A: Floppy.img", "-c", "A:\\RUNTIME.EXE"
    ]).spawn().expect("Failed To Launch DOSBox-X.");
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
            }, "exit" => {
                CommandMessage::Exit.Write().expect("Failed To Write Exit Command Message File.");
                exit(0)
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