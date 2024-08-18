use std::sync::{ Condvar, Mutex };

pub struct NotifyToken(u64);

#[derive(Default)]
pub struct ResourceManagerNotify {
    notify:	Condvar,
    lock:	Mutex<u64>,
}

impl ResourceManagerNotify {
    pub fn notify(&self) {
        let mut l = self.lock.lock().unwrap();

        *l += 1;

        self.notify.notify_all();
    }

    pub fn token(&self) -> NotifyToken {
        let serial = self.lock.lock().unwrap();

        NotifyToken(*serial)
    }

    pub fn wait(&self, token: NotifyToken) {
        let mut serial = self.lock.lock().unwrap();

        while *serial == token.0 {
            serial = self.notify.wait(serial).unwrap();
        }
    }
}
