use evdev_rs::Device;
use std::fs::File;
use evdev_rs::ReadFlag;

fn input_event_to_bytes(event: evdev_rs::InputEvent) {
    let event_raw: libc::input_event = event.as_raw();
    let buffer = unsafe {
        std::slice::from_raw_parts(
            &event_raw as *const libc::input_event as *const u8,
            std::mem::size_of::<libc::input_event>(),
        )
    };
    // println!("{:?}", buffer);
    // println!("{:?}", buffer.len());
    if buffer.len() != 24 {
        println!("ALERT: buffer.len() != 24");
        println!("ALERT: buffer.len() != 24");
        println!("ALERT: buffer.len() != 24");
        println!("ALERT: buffer.len() != 24");
        println!("ALERT: buffer.len() != 24");
    }
    // buffer to input_event
    let new_input_event: libc::input_event = unsafe {
        let mut new_input_event: libc::input_event = std::mem::zeroed();
        std::ptr::copy_nonoverlapping(
            &event_raw as *const libc::input_event as *const u8,
            &mut new_input_event as *mut libc::input_event as *mut u8,
            std::mem::size_of::<libc::input_event>(),
        );
        new_input_event
    };
    let new_input_event_evdev = evdev_rs::InputEvent::from_raw(&new_input_event);
    // println!("{:?}", new_input_event_evdev.value);
}

fn main() {
    let file = File::open("/dev/input/event4").unwrap();
    let d = Device::new_from_file(file).unwrap();
    let mut skip = false;
    loop {
        let ev = d.next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING).map(|val| val.1);
        // if let Ok(ev) = ev {
        //     input_event_to_bytes(ev);
        // }

        match ev {
            // Normal config:
            Ok(ev) => println!("{} {} {} {} {}",
                              ev.value,
                              ev.event_code,
                              ev.event_type().map(|ev_type| format!("{}", ev_type)).
                                unwrap_or("".to_owned()),
                              ev.time.tv_sec,
                              ev.time.tv_usec,
                              ),

//             // Osu! config for low latency and shit:
//             // Ok(ev) => {
//             //     match ev.event_type().unwrap() {
//             //         evdev_rs::enums::EventType::EV_ABS => {
//             //             match ev.event_code {
//             //                 evdev_rs::enums::EventCode::EV_ABS(evdev_rs::enums::EV_ABS::ABS_X) => (),
//             //                 evdev_rs::enums::EventCode::EV_ABS(evdev_rs::enums::EV_ABS::ABS_Y) => (),
//             //                 evdev_rs::enums::EventCode::EV_ABS(evdev_rs::enums::EV_ABS::ABS_PRESSURE) => {
//             //                     if ev.value > 0 {
//             //                         if skip {
//             //                             continue;
//             //                         } else {
//             //                             skip = true;
//             //                         }
//             //                     } else {
//             //                         skip = false;
//             //                     }
//             //                 },
//             //                 _ => continue,
//             //                 };
//             //         },
//             //         _ => continue,
//             //     };
//             //     println!("{} {} {} {} {}",
//             //              ev.value,
//             //              ev.event_code,
//             //              ev.event_type().map(|ev_type| format!("{}", ev_type)).
//             //                  unwrap_or("".to_owned()),
//             //              ev.time.tv_sec,
//             //              ev.time.tv_usec,
//             //              );
//             // },
            Err(e) => println!("{}", e),

        }
    }
}
