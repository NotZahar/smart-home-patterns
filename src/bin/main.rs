use sh::home::Home;
use sh::report::Report;
use sh::room::Room as RoomOps;
use sh::{
    Device, HomeBuilder, Reporter, Room, SmartSocket, SmartThermo, Socket, Subscriber, Thermo,
};

#[derive(Default)]
struct DeviceCounter {
    count: usize,
}

impl Subscriber<f32> for DeviceCounter {
    fn on_event(&mut self, device: &Device) {
        self.count += 1;
        println!(
            "subscriber object saw device #{:?}: {}",
            self.count,
            device.report()
        );
    }
}

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

    println!("{}", home.report());

    let room = Room::default();
    let device = Device::default();
    let socket1 = Socket::default();
    let socket2 = Socket::default();
    let thermo1 = Thermo::default();
    let thermo2 = Thermo::default();

    let _report = Reporter::new()
        .add(&room)
        .add(&device)
        .add(&socket1)
        .add(&socket2)
        .add(&thermo1)
        .add(&thermo2)
        .report();

    let mut observable_room = Room::default();
    observable_room.subscribe(DeviceCounter::default());
    observable_room.subscribe(|device: &Device| {
        println!("closure subscriber saw: {}", device.report());
    });
    observable_room.add_device("Observed socket", SmartSocket::default());

    assert!(home.get_room("First room").is_some());
}
