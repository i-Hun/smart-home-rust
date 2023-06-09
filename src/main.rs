use std::collections::HashMap;

// Дом имеет название и содержит несколько помещений.
struct House {
    name: String,
    rooms: Vec<Room>
}

//Помещение имеет уникальное название и содержит названия нескольких устройств.
struct Room {
    name: String,
    devices: HashMap<String, Box<dyn Device>> // Устройство имеет уникальное в рамках помещения имя. (т.к. словарь)
}

trait Device {
    fn state(&self) -> str;
}

// Библиотека позволяет запросить список помещений в доме.

impl House {
    fn new() -> Self {
        House {
            name: "Default house".to_string(),
            rooms: Default::default()
        }
    }
    fn rooms(&self) -> &Vec<Room> {
        &self.rooms
    }
    fn devices(&self, room: Room) -> &HashMap<String, Box<dyn Device>> {
        todo!()
    }
    fn create_report() -> String {
        todo!()
    }
}

// Библиотека позволяет получать список устройств в помещении.
impl Room {
    fn devices(&self) -> &HashMap<String, Box<dyn Device>> {
        &self.devices
    }
}

struct SmartSocket {
    ip: u32,
    port: u16,
    description: String,
    is_active: bool,
    power: f32,
}

struct SmartThermometer {
    ip: u32,
    port: u16,
    temperature: f32,
}

impl SmartSocket {
    fn _get_description(&self) -> String {
        todo!()
    }
    fn _switch_power(&self) {
        todo!()
    }
    fn _get_power(&self) -> f32 {
        self.power
    }
}

impl SmartThermometer {
    fn _get_current_temperature(&self) -> f32 {
        todo!()
    }
}

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {};
    let socket2 = SmartSocket {};
    let thermo = SmartThermometer {};
}
