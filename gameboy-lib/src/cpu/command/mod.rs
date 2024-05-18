use self::{alu_commands::ArithmeticCommand, load_commands::LoadCommand};

use super::{instructions::Instruction, Cpu};

pub mod alu_commands;
pub mod load_commands;
pub mod misc_commands;
pub mod rotate_commands;
pub mod bit_commands;
pub mod jump_commands;
pub mod call_commands;
pub mod return_commands;

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

    pub fn create_command(&'a mut self, instruction: &'a Instruction) -> Box<dyn Command + 'a> {
        match instruction {
            Instruction::Load(load_command) => Box::new(LoadCommand::new(load_command, self.cpu)),
            Instruction::Arithmetic(alu_command) => Box::new(ArithmeticCommand::new(alu_command, self.cpu)),
            Instruction::Misc(misc_command) => Box::new(misc_commands::MiscCommand::new(&misc_command, self.cpu)),
            Instruction::Rotate(rotate_command) => Box::new(rotate_commands::RotateCommand::new(rotate_command, self.cpu)),
            Instruction::Bit(bit_command) => Box::new(bit_commands::BitCommand::new(bit_command, self.cpu)),
            Instruction::Jump(jump_command) => Box::new(jump_commands::JumpCommand::new(jump_command, self.cpu)),
            Instruction::Call(call_command) => Box::new(call_commands::CallCommand::new(call_command, self.cpu)),
            Instruction::Return(return_command) => Box::new(return_commands::ReturnCommand::new(return_command, self.cpu)),
        }
    }

    // fn command(&'a mut self, instruction: &'a Instruction) -> Box<dyn Command + 'a> {
    //     match instruction {
    //         Instruction::Load(load_command) => Box::new(LoadCommand::new(load_command, self.cpu)),
    //         Instruction::Arithmetic(alu_command) => Box::new(ArithmeticCommand::new(alu_command, self.cpu)),
    //         Instruction::Misc(misc_command) => Box::new(misc_commands::MiscCommand::new(&misc_command, self.cpu)),
    //         Instruction::Rotate(rotate_command) => Box::new(rotate_commands::RotateCommand::new(rotate_command, self.cpu)),
    //         _ => unimplemented!(),
    //     }
    // }

    // fn prefix_command(&'a mut self, instruction: &'a Instruction) -> Box<dyn Command + 'a> {
    //     match instruction {
    //         Instruction::Misc(misc_command) => Box::new(misc_commands::MiscCommand::new(&misc_command, self.cpu)),
    //         Instruction::Rotate(rotate_command) => Box::new(rotate_commands::RotateCommand::new(rotate_command, self.cpu)),
    //         _ => unimplemented!(),
    //     }
    // }
}
