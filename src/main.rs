mod sum_list;
mod traffic_light;
mod area_something;

use sum_list::sum_list;
use traffic_light::TrafficLight;
use area_something::*;
fn main() {

    //04题测试
    println!("红灯时间：{}，黄灯时间{},绿灯时间{}",TrafficLight::Red.time(),TrafficLight::Yellow.time(),TrafficLight::Green.time());

    //05题测试
    let list1:[u32;5]=[2,5,1,7,4];
    println!("list的和为:{:?}",sum_list(&list1).unwrap());

    let list2:[u32;5]=[1,u32::MAX,3,6,9];
    println!("list的和为:{:?}",sum_list(&list2));

    //06题测试
    let circle = Circle{
        radius: 100.0,
    };
    let triangle = Triangle{
        a: 3.0,
        b: 3.0,
        c: 3.0
    };

    let rectangle = Rectangle{
        length: 3.0,
        width: 4.0
    };

    println!("circle面积为：{}，triangle面积为：{}，rectangle面积为：{}",area(circle),area(triangle),area(rectangle));

}



