use iana::hook::inline::InlineHook;

#[no_mangle]
unsafe extern "system" fn target(a: i32, b: i32) -> f64
{
    let c = a + b;
    let d = a * b;
    println!("Value c: {c}, d: {d}");

    (c + d) as _
}

#[no_mangle]
unsafe extern "system" fn detour(a: i32, b: i32) -> f64
{
    println!("Hacked a: {a}, b: {b}");
    1337.1337
}
fn main()
{
    println!(
        "target -> {:#x?} detour -> {:#x?}",
        target as *mut u8, detour as *mut u8
    );

    let mut hook = InlineHook::create(target as _, detour as _).unwrap();

    unsafe {
        target(1, 5);
        let _ = hook.destroy().unwrap();
        target(10, 15);
    }
}
