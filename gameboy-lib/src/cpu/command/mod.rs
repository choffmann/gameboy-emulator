use self::{alu_commands::ArithmeticCommand, load_commands::LoadCommand};

use super::{instructions::Instruction, Cpu};

pub mod alu_commands;
pub mod load_commands;
pub mod misc_commands;
pub mod rotate_commands;

pub trait Command {
    fn execute(&mut self) -> u16;
}

pub struct CommandFactory<'a> {
    cpu: &'a mut Cpu,
}

impl<'a> CommandFactory<'a> {
    pub fn new(cpu: &'a mut Cpu) -> CommandFactory<'a> {
        CommandFactory { cpu }
    }

    pub fn create_command(&'a mut self, instruction: &'a Instruction, prefixed: bool) -> Box<dyn Command + 'a> {
        if prefixed {
            self.prefix_command(instruction)
        } else {
            self.command(instruction)
        }
    }

    fn command(&'a mut self, instruction: &'a Instruction) -> Box<dyn Command + 'a> {
        match instruction {
            Instruction::Load(load_command) => Box::new(LoadCommand::new(load_command, self.cpu)),
            Instruction::Arithmetic(alu_command) => Box::new(ArithmeticCommand::new(alu_command, self.cpu)),
            Instruction::Misc(misc_command) => Box::new(misc_commands::MiscCommand::new(&misc_command, self.cpu)),
            Instruction::Rotate(rotate_command) => Box::new(rotate_commands::RotateCommand::new(rotate_command, self.cpu)),
            _ => unimplemented!(),
        }
    }

    fn prefix_command(&'a mut self, instruction: &'a Instruction) -> Box<dyn Command + 'a> {
        match instruction {
            Instruction::Misc(misc_command) => Box::new(misc_commands::MiscCommand::new(&misc_command, self.cpu)),
            Instruction::Rotate(rotate_command) => Box::new(rotate_commands::RotateCommand::new(rotate_command, self.cpu)),
            _ => unimplemented!(),
        }
    }
}
