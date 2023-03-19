#[repr(packed)]
struct SegmentDescriptor {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    limit_high_flags: u8,
    base_high: u8,
}

#[repr(packed)]
struct GlobalDescriptorTable {
    limit: u16,
    base: u64,
    null_segment: SegmentDescriptor,
    code_segment: SegmentDescriptor,
    data_segment: SegmentDescriptor,
}

fn setup_gdt() {
    // Create a new GDT
    let gdt = Box::new(GlobalDescriptorTable {
        limit: 0,
        base: 0,
        null_segment: SegmentDescriptor {
            limit_low: 0,
            base_low: 0,
            base_middle: 0,
            access: 0,
            limit_high_flags: 0,
            base_high: 0,
        },
        code_segment: SegmentDescriptor {
            limit_low: 0xffff,
            base_low: 0,
            base_middle: 0,
            access: 0x9a,
            limit_high_flags: 0xcf,
            base_high: 0,
        },
        data_segment: SegmentDescriptor {
            limit_low: 0xffff,
            base_low: 0,
            base_middle: 0,
            access: 0x92,
            limit_high_flags: 0xcf,
            base_high: 0,
        },
    });

    // Load the GDT by setting the GDTR register
    unsafe {
        asm!("lgdt [{0}]", in(reg) &gdt.limit as *const u16);
        asm!("mov ds, ax", out("ax") _);
        asm!("mov es, ax", out("ax") _);
        asm!("mov fs, ax", out("ax") _);
        asm!("mov gs, ax", out("ax") _);
        asm!("mov ss, ax", out("ax") _);
    }
}
