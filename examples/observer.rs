use sh::report::Report;
use sh::room::Room as RoomOps;
use sh::{Device, Room, SmartSocket, SmartThermo, Subscriber};

#[derive(Default)]
struct MySubscriber {
    events: usize,
}

impl Subscriber<f32> for MySubscriber {
    fn on_event(&mut self, device: &Device) {
        self.events += 1;
        println!(
            "object subscriber event {}: {}",
            self.events,
            device.report()
        );
    }
}

fn main() {
    let mut room = Room::default();
    room.subscribe(MySubscriber::default());
    room.subscribe(|device: &Device| println!("closure subscriber: {}", device.report()));

    room.add_device("Socket_1", SmartSocket::default());
    room.add_device("Thermo_1", SmartThermo::default());
}
