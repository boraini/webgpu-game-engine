pub enum ServiceType {
    MouseService,
    TweeningService,
}

pub struct EnabledServices {
    pub mouse_service: bool,
    pub tweening_service: bool,
}

impl Default for EnabledServices {
    fn default() -> Self {
        Self {
            mouse_service: false,
            tweening_service: false,
        }
    }
}

impl EnabledServices {
    pub fn enable(&mut self, service: ServiceType) -> &mut Self {
        match service {
            ServiceType::MouseService => self.mouse_service = true,
            ServiceType::TweeningService => self.tweening_service = true,
        };

        self
    }
}
