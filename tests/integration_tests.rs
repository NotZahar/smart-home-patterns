use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use sh::builder::HomeBuilder;
use sh::error::HomeError;
use sh::home::{Home, SmartHome};
use sh::make_room;
use sh::report::Report;
use sh::reporter::Reporter;
use sh::room::{Room, SmartRoom, Subscriber};
use sh::smart_device::{
    CelsiusThermometer, Device, PowerSocket, SmartSocket, SmartThermo, Socket, SocketState,
    Thermometer,
};
use sh::{Device as DefaultDevice, Reporter as DefaultReporter, Room as DefaultRoom};

#[test]
fn test_make_power_socket() {
    let socket = PowerSocket::<f32>::new(100.0, 20.0);
    assert_eq!(socket.get_state(), SocketState::Off);
}

#[test]
fn test_make_thermometer() {
    let mut thermometer = CelsiusThermometer::<f32>::new(22.0, -5.0, 8.0);
    let temperature = thermometer.get_temperature();
    assert!((17.0..=30.0).contains(&temperature));
}

#[test]
fn test_make_room() {
    let devices: BTreeMap<String, Device<f32>> = BTreeMap::new();
    let room = SmartRoom::<f32>::new(devices);
    assert!(room.get_device("nonexistent").is_none());
}

#[test]
fn test_make_home() {
    let rooms: BTreeMap<String, SmartRoom<f32>> = BTreeMap::new();
    let home = SmartHome::<f32>::new(rooms);
    assert!(home.get_room("nonexistent").is_none());
}

#[test]
fn test_make_room_macro() {
    let socket = PowerSocket::<f32>::new(100.0, 20.0);
    let thermometer = CelsiusThermometer::<f32>::new(22.0, -5.0, 8.0);
    let room = make_room!(
        "socket" => socket,
        "thermometer" => thermometer
    );
    assert!(room.get_device("socket").is_some());
    assert!(room.get_device("thermometer").is_some());
}

#[test]
fn test_add_room() {
    let mut home = SmartHome::<f32>::new(BTreeMap::new());
    let room = make_room!("socket" => PowerSocket::<f32>::new(100.0, 20.0));

    home.add_room("living", room);
    assert!(home.get_room("living").is_some());
}

#[test]
fn test_remove_room() {
    let mut home = SmartHome::<f32>::new(BTreeMap::new());
    let room = make_room!("socket" => PowerSocket::<f32>::new(100.0, 20.0));

    home.add_room("living", room);
    home.remove_room("living");

    assert!(home.get_room("living").is_none());
}

#[test]
fn test_add_device() {
    let new_socket = PowerSocket::<f32>::new(200.0, 30.0);
    let mut room = make_room!("socket" => PowerSocket::<f32>::new(100.0, 20.0));

    room.add_device("new_socket", new_socket);

    assert!(room.get_device("new_socket").is_some());
}

#[test]
fn test_remove_device() {
    let mut room = make_room!(
        "socket" => PowerSocket::<f32>::new(100.0, 20.0),
        "thermometer" => CelsiusThermometer::<f32>::new(22.0, -5.0, 8.0)
    );

    room.remove_device("thermometer");
    assert!(room.get_device("thermometer").is_none());
    assert!(room.get_device("socket").is_some());
}

#[test]
fn test_get_device_room_not_found() {
    let home = SmartHome::<f32>::new(BTreeMap::new());
    let result = home.get_device("nonexistent_room", "device");

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), HomeError::RoomNotFound(_)));
}

#[test]
fn test_get_device_device_not_found() {
    let mut home = SmartHome::<f32>::new(BTreeMap::new());
    let room = make_room!("socket" => PowerSocket::<f32>::new(100.0, 20.0));

    home.add_room("living", room);
    let result = home.get_device("living", "nonexistent_device");

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), HomeError::DeviceNotFound(_)));
}

#[test]
fn test_device_report() {
    let socket = PowerSocket::<f32>::new(100.0, 20.0);
    let report = socket.report();

    assert!(report.contains("Socket"));
    assert!(report.contains("100"));
    assert!(report.contains("20"));
}

#[test]
fn test_room_report() {
    let room = make_room!("socket" => PowerSocket::<f32>::new(100.0, 20.0));
    let report = room.report();

    assert!(report.contains("Device"));
    assert!(report.contains("socket"));
}

#[test]
fn test_from_power_socket_to_device() {
    let socket = PowerSocket::<f32>::new(100.0, 20.0);
    let device: Device<f32> = socket.into();

    match device {
        Device::Socket(_) => {}
        _ => panic!("Expected Device::Socket"),
    }
}

#[test]
fn test_from_thermometer_to_device() {
    let thermometer = CelsiusThermometer::<f32>::new(22.0, -5.0, 8.0);
    let device: Device<f32> = thermometer.into();

    match device {
        Device::Thermometer(_) => {}
        _ => panic!("Expected Device::Thermometer"),
    }
}

#[test]
fn test_home_builder_adds_devices_to_current_room() {
    let home = HomeBuilder::new()
        .add_room("First room")
        .add_device("Socket_1", SmartSocket::default())
        .add_device("Thermo_1", SmartThermo::default())
        .add_room("Second room")
        .add_device("Socket_2", SmartSocket::default())
        .build();

    let first_room = home.get_room("First room").expect("first room exists");
    let second_room = home.get_room("Second room").expect("second room exists");

    assert_eq!(home.room_count(), 2);
    assert!(first_room.get_device("Socket_1").is_some());
    assert!(first_room.get_device("Thermo_1").is_some());
    assert!(second_room.get_device("Socket_2").is_some());
    assert!(second_room.get_device("Socket_1").is_none());
}

#[test]
fn test_home_builder_can_build_empty_home() {
    let home = HomeBuilder::new().build();
    assert_eq!(home.room_count(), 0);
}

#[test]
fn test_reporter_collects_heterogeneous_reports() {
    let room = SmartRoom::<f32>::default();
    let device = Device::default();
    let socket = SmartSocket::default();
    let thermo = SmartThermo::default();

    let report = Reporter::new()
        .add(&room)
        .add(&device)
        .add(&socket)
        .add(&thermo)
        .report();

    assert!(report.contains("Report:"));
    assert!(report.contains("Room is empty"));
    assert!(report.contains("Socket"));
    assert!(report.contains("Thermometer"));
}

#[test]
fn test_reporter_supports_assignment_style_default_aliases() {
    let room = DefaultRoom::default();
    let device = DefaultDevice::default();
    let socket1 = sh::Socket::default();
    let socket2 = sh::Socket::default();
    let thermo1 = sh::Thermo::default();
    let thermo2 = sh::Thermo::default();

    let report = DefaultReporter::new()
        .add(&room)
        .add(&device)
        .add(&socket1)
        .add(&socket2)
        .add(&thermo1)
        .add(&thermo2)
        .report();

    assert!(report.contains("Item #6"));
}

#[derive(Default)]
struct CountingSubscriber {
    events: Rc<RefCell<usize>>,
}

impl CountingSubscriber {
    fn new(events: Rc<RefCell<usize>>) -> Self {
        Self { events }
    }
}

impl Subscriber<f32> for CountingSubscriber {
    fn on_event(&mut self, _device: &Device<f32>) {
        *self.events.borrow_mut() += 1;
    }
}

#[test]
fn test_room_notifies_object_subscriber_on_add_device() {
    let events = Rc::new(RefCell::new(0));
    let mut room = SmartRoom::<f32>::default();

    room.subscribe(CountingSubscriber::new(Rc::clone(&events)));
    room.add_device("socket", SmartSocket::default());
    room.add_device("thermo", SmartThermo::default());

    assert_eq!(*events.borrow(), 2);
}

#[test]
fn test_room_notifies_closure_subscriber_on_add_device() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let subscriber_events = Rc::clone(&events);
    let mut room = SmartRoom::<f32>::default();

    room.subscribe(move |device: &Device<f32>| {
        subscriber_events.borrow_mut().push(device.report());
    });
    room.add_device("socket", SmartSocket::default());

    let events = events.borrow();
    assert_eq!(events.len(), 1);
    assert!(events[0].contains("Socket"));
}
