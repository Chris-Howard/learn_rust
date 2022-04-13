
pub fn sum_list(list:&[u32]) -> Option<u32> {
    let mut sum:u32 =0;
    for i in list.iter(){
                sum = sum.checked_add(*i)?;
        };
    //如何判断有无溢出，溢出return None，没有溢出return sum.
    Some(sum)
}
