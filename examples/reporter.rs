use sh::{Device, Reporter, Room, Socket, Thermo};

fn main() {
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
}
