//抽象
pub trait  HasArea{
    fn area(&self) ->f64;
}
//泛型,约束必须实现HasArea
pub fn area<T:HasArea>(shape:T)->f64{
    shape.area()
}


//圆
pub struct Circle{
    //圆心坐标
    //x:f64,
    //y:f64,
    //半径
    pub(crate) radius:f64,
}
//实现HasArea
impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI*(self.radius*self.radius)
    }
}

//三角形
pub struct Triangle{
    //三边长
    pub a:f64,
    pub b:f64,
    pub c:f64,
}

impl HasArea for Triangle{
    fn area(&self) -> f64 {
        //海伦公式求面积
        let p = (self.a+self.b+self.c)/2.0;
        (p*(p-self.a)*(p-self.b)*(p-self.c)).sqrt()
    }
}
//正方形
pub struct Rectangle{
    //长
    pub(crate) length:f64,
    //宽
    pub(crate) width:f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.width*self.length
    }
    
}
