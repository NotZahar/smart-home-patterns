#[macro_export]
macro_rules! make_room {
    ( $($device_name:expr => $device:expr),* $(,)? ) => {
        {
            let mut room = $crate::room::SmartRoom::default();
            $(
                room.add_device($device_name, $device);
            )*
            room
        }
    };
}

mod utils;

pub mod builder;
pub mod error;
pub mod home;
pub mod report;
pub mod reporter;
pub mod room;
pub mod smart_device;

pub use builder::HomeBuilder;
pub use reporter::Reporter;
pub use room::Subscriber;
pub use smart_device::{SmartSocket, SmartThermo};

pub type Home = home::SmartHome<f32>;
pub type Room = room::SmartRoom<f32>;
pub type Device = smart_device::Device<f32>;
pub type Socket = smart_device::PowerSocket<f32>;
pub type Thermo = smart_device::CelsiusThermometer<f32>;
