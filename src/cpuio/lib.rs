use core::marker::PhantomData;

pub struct Port<T> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T> Port<T> {
    pub unsafe fn new(port: u16) -> Port<T> {
        Port { port: port, phantom: PhantomData }
    }

    pub fn read(&self) -> u8 {
        unsafe { inb(self.port) }
    }

    pub fn write(&self, value: u8) {
        unsafe { outb(value, self.port) }
    }
}
