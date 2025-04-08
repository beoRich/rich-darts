pub fn is_boogey_nr(val: u16) -> bool {
    let boogey_nr: Vec<u16> = vec![169, 168, 166, 165, 163, 162, 159];
    match val {
        val if boogey_nr.contains(&val) => false,
        0..170 => true,
        _ => false,
    }
}