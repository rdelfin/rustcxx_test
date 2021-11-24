use cxx::let_cxx_string;

#[cxx::bridge]
mod ffi {
    #[derive(Copy, Debug)]
    #[repr(i32)]
    enum Suit {
        #[cxx_name = "SPADES"]
        Spades = 1,
        #[cxx_name = "DIAMONDS"]
        Diamonds = 2,
        #[cxx_name = "CLUBS"]
        Clubs = 3,
        #[cxx_name = "HEARTS"]
        Hearts = 4,
    }

    unsafe extern "C++" {
        include!("cards.hpp");
        type Suit;
    }

    unsafe extern "C++" {
        include!("cards.hpp");
        fn suitToName(vehicle_model: Suit) -> UniquePtr<CxxString>;
        fn nameToSuit(vehicle_model: &CxxString) -> Suit;
    }
}

pub use self::ffi::Suit;

impl ToString for Suit {
    fn to_string(&self) -> String {
        let output = ffi::suitToName(*self);
        output.to_string()
    }
}

impl From<String> for Suit {
    fn from(name: String) -> Self {
        let_cxx_string!(name_cxx = &name);
        ffi::nameToSuit(&name_cxx)
    }
}

impl From<Suit> for String {
    fn from(suit: Suit) -> Self {
        suit.to_string()
    }
}
