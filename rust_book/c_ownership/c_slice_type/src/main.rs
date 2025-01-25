/*
    Slice
    - Slice adalah representasi block of memory berbentuk pointer dan memiliki size yang dinamis(tidak fixed seperti array)
    - Notasi tipe data slice adalah &[T] dimana T adalah tipe data element
    - Slice bisa dibuat dari data array atau tipe data kolektif lainnya, dengan menggunakan kombinasi operator & dan range syntax ...
    - Slice range syntax:
        - start_index..end_index    = slicing data dari start_index hingga sebelum end_index
        - start_index..=end_index   = slicing data dari start_index hingga end_index
        - ..end_index               = slicing data dari index 0 hingga sebelum end_index
        - ..=end_index              = slicing data dari index 0 hingga end_index
        - start_index..             = slicing data dari start_index hingga indeks terakhir
        - ...                       = slicing semua data
*/

fn first_word(s: &String) -> &str {
    // convert string menjadi array
    let bytes = s.as_bytes();
    for (index, &item) in bytes.iter().enumerate(){
        // akan berhenti jika spasi di temukan
        if item == b' '{
            return &s[0..index];
        }
    }

    &s[..]
}

fn main() {
    let sample = String::from("hello world");
    let word = first_word(&sample);

    println!("first word: {}", word); // tetap akan return 5 walaupun string di kosongkan

    let mut numbers = [3, 12, 55, 4];
    println!("numbers     : {:?}, len : {:?}", numbers, numbers.len());
    println!("numbers[0]  : {:?}", numbers[0]);
    println!("numbers[1]   : {:?}", numbers[1]);

    let slice_a = &numbers[0..3];
    println!("slice_a     : {:?}, len : {:?}", slice_a, slice_a.len());
    println!("slice_a[0]  : {:?}", slice_a[0]);
    println!("slice_a[1]   : {:?}", slice_a[1]);

    let slice_b = &slice_a[1..=2];
    println!("slice_b     : {:?}, len : {:?}", slice_b, slice_b.len());
    println!("slice_b[0]  : {:?}", slice_b[0]);
    println!("slice_b[1]   : {:?}", slice_b[1]);

    // mutable slice
    let slice_c = &mut numbers[1..];
    slice_c[1] = 7;
    println!("slice_c     : {:?}, len : {:?}", slice_c, slice_c.len());
    println!("slice_c[0]  : {:?}", slice_c[0]);
    println!("slice_c[1]   : {:?}", slice_c[1]);
    // for in sliece mutable
    for slice in &mut slice_c[..]{
        print!("{:?} ", slice);
    }

    println!("=====");

    // for in slice immutable
    let scores = [7, 8, 9];
    for score in &scores[1..]{
        print!("{:?} ", score);
    }
}
