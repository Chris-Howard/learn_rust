fn main() {
    println!("红灯时间：{}，黄灯时间{},绿灯时间{}",TrafficLight::Red.time(),TrafficLight::Yellow.time(),TrafficLight::Green.time());
}


enum TrafficLight{
    Red,
    Yellow,
    Green,
}

impl TrafficLight{
    fn time(&self)->u8{
        return match self {
            TrafficLight::Red => 15,
            TrafficLight::Yellow => 10,
            TrafficLight::Green => 90,
        };
    }
}
