use crate::cpu::CPU;
use crossterm::event::poll;

use crossterm::{event::read, event::Event, event::KeyEvent, event::KeyCode};
fn update_keyboard(cpu: &mut CPU, key_pressed: char) {
  match key_pressed {
    '0' => cpu.key_pressed = Some(0x0),
    '1' => cpu.key_pressed = Some(0x1),
    '2' => cpu.key_pressed = Some(0x2),
    '3' => cpu.key_pressed = Some(0x3),
    '4' => cpu.key_pressed = Some(0x4),
    '5' => cpu.key_pressed = Some(0x5),
    '6' => cpu.key_pressed = Some(0x6),
    '7' => cpu.key_pressed = Some(0x7),
    '8' => cpu.key_pressed = Some(0x8),
    '9' => cpu.key_pressed = Some(0x9),
    'a' => cpu.key_pressed = Some(0xA),
    'b' => cpu.key_pressed = Some(0xB),
    'c' => cpu.key_pressed = Some(0xC),
    'd' => cpu.key_pressed = Some(0xD),
    'e' => cpu.key_pressed = Some(0xE),
    'f' => cpu.key_pressed = Some(0xF),
    _ => {
      cpu.key_pressed = None
    }
  }
}

use std::time::Duration;
pub fn update_events(cpu: &mut CPU) -> bool {
    let event_available = poll(Duration::from_millis(0));
    if let Ok(true) = event_available {
      let event = read().unwrap();
      match event  {
        Event::Key(KeyEvent { code, .. }) => {
          match code {
            KeyCode::Esc => {
              return false;
            },
            KeyCode::Char(c) => {
              update_keyboard(cpu, c);
            }
            _ => {}
          }
        }
        _ => {}
      };
    }
    return true;
}

pub fn wait_for_key_events(cpu: &mut CPU) {
    loop {
        update_events(cpu);
        if let Some(_k) = cpu.key_pressed {
            cpu.key_pressed;
            break;
        }
    }
}