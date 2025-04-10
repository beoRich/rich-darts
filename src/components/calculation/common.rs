pub fn is_finish(val: u16) -> bool {
    let boogey_nr: Vec<u16> = vec![169, 168, 166, 165, 163, 162, 159];
    match val {
        val if boogey_nr.contains(&val) => false,
        2..170 => true,
        _ => false,
    }
}