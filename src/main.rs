fn main() {
    struct _Socket {
        ip: u32,
        port: u16,
        description: String,
        is_active: bool,
        power: f32,
    }

    struct _Termometer {
        ip: u32,
        port: u16,
        temperature: f32,
    }

    impl _Socket {
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

    impl _Termometer {
        fn _get_current_temperature(&self) -> f32 {
            todo!()
        }
    }
}
