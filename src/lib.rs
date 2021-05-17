#[cfg(feature = "fake-hardware")]
extern crate ansi_term;
extern crate failure;
extern crate rgb;
#[cfg(feature = "hardware")]
extern crate rppal;

#[cfg(feature = "fake-hardware")]
use ansi_term::ANSIStrings;
#[cfg(feature = "fake-hardware")]
use ansi_term::Color::RGB;
use failure::Error;
#[cfg(feature = "hardware")]
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

const BUFFER_SIZE: usize = 256 * 3;
const BLACK: u8 = 0;

#[cfg(feature = "hardware")]
/// Provide high-level access to the Unicorn HAT HD.
pub struct UnicornHatHd {
    leds: [u8; BUFFER_SIZE],
    spi: Spi,
}

#[cfg(feature = "fake-hardware")]
/// Provide high-level access to an emulated Unicorn HAT HD.
pub struct UnicornHatHd {
    leds: [u8; (BUFFER_SIZE)],
}

impl UnicornHatHd {
    #[cfg(feature = "hardware")]
    /// Create a new `UnicornHatHd` with the provided Bus and SlaveSelect
    pub fn new(bus: Bus, slave_select: SlaveSelect) -> Result<UnicornHatHd, Error> {
        let spi = Spi::new(bus, slave_select, 9_000_000, Mode::Mode0)?;

        Ok(UnicornHatHd {
            leds: [BLACK; BUFFER_SIZE],
            spi,
        })
    }

    #[cfg(feature = "fake-hardware")]
    /// Create a fake `UnicornHatHd`
    ///
    /// `_bus` and `_slave_select` are completely unused by the fake `UnicornHatHd`.
    pub fn new(_bus: Bus, _slave_select: SlaveSelect) -> Result<UnicornHatHd, Error> {
        Ok(UnicornHatHd {
            leds: [BLACK; BUFFER_SIZE],
        })
    }

    #[cfg(feature = "hardware")]
    /// Write the display buffer to the Unicorn HAT HD.
    pub fn display(&mut self) -> Result<(), Error> {
        self.spi.write(&[0x72])?;
        self.spi.write(&self.leds)?;
        Ok(())
    }

    #[cfg(feature = "fake-hardware")]
    /// Write the display buffer to the Unicorn HAT HD.
    pub fn display(&mut self) -> Result<(), Error> {
        println!("Unicorn HAT HD:");
        for y in 0..16 {
            let mut line = vec![];
            for x in 0..16 {
                let pixel = self.get_pixel(x, y);
                line.push(RGB(pixel.r, pixel.g, pixel.b).paint("*"));
            }
            println!("{}", ANSIStrings(&line));
        }

        Ok(())
    }

    /// Set an individual pixel's RGB value.
    ///
    /// The origin (`(0, 0)`) is the top-left of the display, with `x` & `y`
    /// increasing to the right, and down, respectively.
    pub fn set_pixel(&mut self, x_coord: usize, y_coord: usize, c: rgb::RGB8) {
        self.leds[(y_coord * 16) + (x_coord * 3)] = c.r;
        self.leds[(y_coord * 16) + (x_coord * 3 + 1)] = c.g;
        self.leds[(y_coord * 16) + (x_coord * 3 + 2)] = c.b;
    }

    /// Return a tuple of an individual pixel's RGB value.
    ///
    /// The origin (`(0, 0)`) is the top-left of the display, with `x` & `y`
    /// increasing to the right, and down, respectively.
    ///
    /// *NOTE*: This returns what's in the display buffer, not what the
    /// physical pixel is set to.
    pub fn get_pixel(&self, x_coord: usize, y_coord: usize) -> rgb::RGB8 {
        let red = self.leds[(y_coord * 16) + (x_coord * 3)];
        let green = self.leds[(y_coord * 16) + (x_coord * 3 + 1)];
        let blue = self.leds[(y_coord * 16) + (x_coord * 3 + 2)];

        rgb::RGB8::new(red, green, blue)
    }

    /// Clear the internal buffer of pixel states.
    ///
    /// To clear the display itself, you'll still need to call
    /// [`display`](#method.display) to update the Unicorn HAT HD.
    pub fn clear_pixels(&mut self) {
        self.leds = [BLACK; BUFFER_SIZE];
    }
}

impl Default for UnicornHatHd {
    /// Create a `UnicornHatHd` using the default `Bus::Spi0` and `SlaveSelect::Ss0`.
    ///
    /// This will panic if initialization fails.
    fn default() -> UnicornHatHd {
        UnicornHatHd::new(Bus::Spi0, SlaveSelect::Ss0).unwrap()
    }
}
