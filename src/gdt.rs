use core::{arch::asm};


#[repr(C,packed)]
pub struct SegmentDescriptor{
    limit_low:u16,
    base_low:u16,
    base_middle:u8,
    access:u8,
    flags:u8,
    base_high:u8
} // creating SegmentDescriptor struct
const KERNEL_CS: u16 = 0x08;  // Kernel code segment selector
const KERNEL_DS: u16 = 0x10;  // Kernel data segment selector

pub struct GlobalDescriptionTable{
    NullDescriptor:SegmentDescriptor,
    KernelCodeDescriptor:SegmentDescriptor,
    KernelDataDescriptor:SegmentDescriptor,

}
static mut GDT:GlobalDescriptionTable = GlobalDescriptionTable{
    NullDescriptor:SegmentDescriptor { limit_low:0,base_low:0,base_middle:0,access:0,flags:0,base_high:0 },
    KernelCodeDescriptor:SegmentDescriptor{limit_low:0xFFFF,base_low:0,base_middle:0,access:0x9A,flags:0xCF,base_high:0},
    KernelDataDescriptor:SegmentDescriptor{limit_low:0xFFFF,base_low:0,base_middle:0,access:0x92,flags:0xCF,base_high:0}
};

#[repr(C,packed)]
pub struct gdt_ptr{
    limit:u16,
    base:u32
}




#[repr(C, packed)]
struct GdtPointer {
    limit: u16,
    base: u32,
}

pub fn set_gdt() {
                                        //limit 
    //let gdtr_pointer:*const u8 = unsafe{&GDT as *const _ as *const u8};   
    
    let limit = unsafe{core::mem::size_of_val(&GDT) as u16 -1 };
    let base = unsafe{&GDT as *const _ as u32};
    let gdtr = GdtPointer{
        base:base,
        limit:limit
    };
    //pointer to base
    unsafe {
        asm!("lgdt [{0}]", in(reg) &gdtr);
    }
}
pub fn read_write_memory() {
    let data_ptr: *mut u32 = 0x1000 as *mut u32;
    let code_ptr: *mut u32 = 0x2000 as *mut u32;

    unsafe {
        // Write a value to the data segment
        *data_ptr = 42;

        // Read the value from the data segment
        let value = *data_ptr;

        // Move the value to the code segment
        asm!("mov eax, {0}", in(reg) value as u32);
        asm!("mov [{0}], eax", in(reg) code_ptr);
    }
}
