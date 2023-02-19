pub struct GlobalDescriptionTable{

}
#[derive(Clone, Copy,Debug)]
pub struct SegmentDescriptor{
    pub limit_low:u16,
    pub base_low:u16,
    pub access:u8,
    base_middle:u8,
    flags:u8,
    base_high:u8
}
impl  GlobalDescriptionTable {
    pub fn new(&self){

    }
}
impl SegmentDescriptor {
    pub fn new(limit_low:u16,base_low:u16,access:u8)->SegmentDescriptor{
        return SegmentDescriptor { limit_low: limit_low, base_low: base_low, access: access, base_middle: 0, flags: 0, base_high: 0 }
    }
}   