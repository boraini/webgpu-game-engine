use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use lazy_static::lazy_static;

pub enum MouseAction {
    DOWN,
    UP,
    MOVE,
}

pub struct MouseEvent {
    pub action: MouseAction,
    pub x: f64,
    pub y: f64,
}

pub struct MouseService {
    pub last_x: f64,
    pub last_y: f64,
    pub delta_x: f64,
    pub delta_y: f64,
    pub gesture_start_x: f64,
    pub gesture_start_y: f64,
    pub is_down: bool,
    pub event_handlers: Vec<fn(&MouseEvent)>,
}

lazy_static! {
    static ref SERVICE: Arc<RwLock<MouseService>> = Arc::new(MouseService { last_x: 0.0, last_y: 0.0, gesture_start_x: 0.0, gesture_start_y: 0.0, delta_x: 0.0, delta_y: 0.0, is_down: false, event_handlers: vec!() }.into());
}

impl MouseService {
    pub fn get() -> RwLockReadGuard<'static, MouseService> {
        SERVICE.read().unwrap()
    }

    pub fn get_mut() -> RwLockWriteGuard<'static, MouseService> {
        SERVICE.write().unwrap()
    }

    pub fn handle_mouse_down() {
        let mut service = Self::get_mut();
        service.is_down = true;
        service.gesture_start_x = service.last_x;
        service.gesture_start_y = service.last_y;
        let event = MouseEvent { action: MouseAction::DOWN, x: service.last_x, y: service.last_y };

        drop(service);

        Self::get().dispatch(&event);
    }

    pub fn handle_mouse_up() {
        let mut service = Self::get_mut();
        service.is_down = false;
        let event = MouseEvent { action: MouseAction::UP, x: service.last_x, y: service.last_y };

        drop(service);

        Self::get().dispatch(&event);
    }

    pub fn handle_mouse_move(x: f64, y: f64) {
        let mut service = Self::get_mut();
        service.delta_x = x - service.last_x;
        service.delta_y = y - service.last_y;
        service.last_x = x;
        service.last_y = y;
        let event = MouseEvent { action: MouseAction::MOVE, x: service.last_x, y: service.last_y };

        drop(service);

        Self::get().dispatch(&event);
    }

    pub fn clear_deltas(&mut self) {
        self.delta_x = 0.0;
        self.delta_y = 0.0;
    }

    pub fn add_handler(&mut self, f: fn(&MouseEvent)) {
        self.event_handlers.push(f);
    }

    pub fn dispatch(&self, event: &MouseEvent) {
        self.event_handlers.iter().for_each(|f| f(&event));
    }
}