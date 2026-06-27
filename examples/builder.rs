use sh::home::Home;
use sh::{HomeBuilder, SmartSocket, SmartThermo};

fn main() {
    let home = HomeBuilder::new()
        .add_room("First room")
        .add_device("Socket_1", SmartSocket::default())
        .add_device("Socket_2", SmartSocket::default())
        .add_device("Thermo_1", SmartThermo::default())
        .add_room("Second room")
        .add_device("Socket_3", SmartSocket::default())
        .add_device("Thermo_2", SmartThermo::default())
        .build();

    println!("rooms: {}", home.room_count());
    println!(
        "first room devices: {}",
        home.get_room("First room")
            .map_or(0, |room| room.device_count())
    );
}
