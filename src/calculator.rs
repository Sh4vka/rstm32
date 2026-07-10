use crate::{calculator::State::RESULT, components::{keypad::{self, KeyPad}, lcd1602a::{self, LCD8}}, f103::{gpio::Port, systick}};

pub struct Calc {
    display : LCD8,
    keypad  : KeyPad,

    a       : u32,
    b       : u32,
    op      : Command,
    state   : State
}

#[derive(PartialEq)]
enum Command {
    ADD,
    SUB,
    MUL,
    RET,
    EQ,
    NONE,
}

#[derive(PartialEq)]
enum State {
    FIRST,
    SECOND,
    RESULT,
}

impl Calc {
    pub fn new() -> Self {
        let display = lcd1602a::LCD8::new();
        let keypad = keypad::KeyPad::new(
            [
                (8, Port::PA),
                (9, Port::PA),
                (10, Port::PA),
                (11, Port::PA),
            ],
            [
                (15, Port::PB),
                (14, Port::PB),
                (12, Port::PB),
                (13, Port::PB),
            ]
        );
        display.start();
        Calc { 
            display, 
            keypad,
            a   : 0,
            b   : 0,
            op  : Command::NONE,
            state   :State::FIRST
        }
    }

    fn get_num(&self, num : u8) -> Option<u32> {
        match num {
            0 => Some(1),
            1 => Some(2),
            2 => Some(3),
            4 => Some(4),
            5 => Some(5),
            6 => Some(6),
            8 => Some(7),
            9 => Some(8),
            10 => Some(9),
            13 => Some(0),
            _ => None
        }
    }

    fn get_command(&self, num : u8) -> Option<Command> {
        match num {
            0x3 => Some(Command::RET),
            0x7 => Some(Command::ADD),
            0xb => Some(Command::SUB),
            0xe => Some(Command::EQ),
            0xf => Some(Command::MUL),
            _   => None
        }
    }

    pub fn execute(&mut self) {
        match self.state {
            State::FIRST => {
                self.display.clear();
                self.display.print_u32(self.a);
                if let Some(key) = self.keypad.get_key() {
                    if self.a == 0 {
                        if let Some(num) = self.get_num(key) {
                            self.a = num;
                        }
                    } else if let Some(num) = self.get_num(key) {
                        self.a *= 10;
                        self.a += num;
                    } else if let Some(com) = self.get_command(key) {
                        if com == Command::RET {
                            self.a /= 10;
                        }
                        else if com != Command::EQ {
                            self.state = State::SECOND;
                            self.op = com
                        }
                    }
                }
            }
            State::SECOND => {
                self.display.clear();
                self.display.print_u32(self.a);
                if let Some(char) = self.get_char_command() {
                    self.display.print(char);
                }
                 self.display.print_u32(self.b);
                if let Some(key) = self.keypad.get_key() {
                    if self.b == 0 {
                        if let Some(num) = self.get_num(key) {
                            self.b = num;
                        }
                    } else if let Some(num) = self.get_num(key) {
                        self.b *= 10;
                        self.b += num;
                    }
                    else if let Some(com) = self.get_command(key) {
                        if com == Command::RET {
                            self.a /= 10;
                        }
                        else if com == Command::EQ {
                            self.state = RESULT;
                        }
                    }
                }
            }
            State::RESULT => {
                self.display.clear();
                self.display.print_u32(self.a);
                if let Some(char) = self.get_char_command() {
                    self.display.print(char);
                }
                self.display.print_u32(self.b);
                self.display.print(" = ");
                self.display.set_cursor(0, 1);
                self.display.print_u32(self.get_res());
                systick::delay_ms(100);
            }
        }
    }

    fn get_char_command(&self) -> Option<&str> {
        match self.op {
            Command::ADD => Some(" + "),
            Command::MUL => Some(" x "),
            Command::SUB => Some(" - "),
            _ => None,
        }
    }

    fn get_res(&self) -> u32 {
        match self.op {
            Command::ADD => self.a + self.b,
            Command::MUL => self.a * self.b,
            Command::SUB => self.a - self.b,
            _ => 0
        }
    }
}
