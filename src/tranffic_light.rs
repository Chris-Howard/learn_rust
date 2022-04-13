pub enum TrafficLight{
    Red,
    Yellow,
    Green,
}

impl TrafficLight{
    pub(crate) fn time(&self) ->u8{
        return match self {
            TrafficLight::Red => 15,
            TrafficLight::Yellow => 10,
            TrafficLight::Green => 90,
        };
    }
}
