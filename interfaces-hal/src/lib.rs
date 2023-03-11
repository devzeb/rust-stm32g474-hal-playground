#![no_std]

pub trait ReadByte {
    fn read_byte(&self) -> u8;
}

pub trait WriteByte {
    fn write_byte(&self, byte: u8);
}
