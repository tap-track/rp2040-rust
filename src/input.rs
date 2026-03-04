use embedded_hal::digital::InputPin;

pub struct Button<P> {
    pin: P,
}

impl<P> Button<P>
where
    P: InputPin,
{
    pub fn new(pin: P) -> Self {
        Self { pin }
    }

    pub fn is_pressed(&mut self) -> bool {
        self.pin.is_low().unwrap_or(false) 
    }
}
