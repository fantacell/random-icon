pub fn hash(string: String) -> u32 {
    let mut vec: Vec<u8> = string.into();

    if vec.is_empty() {
        vec = vec![0]
    }

    for mut c in vec.iter_mut() {
        let noise = u8::try_from(c.count_zeros()).unwrap() & 0b1111;

        *c = *c ^ noise;
        *c = *c ^ (noise << 4);
    }

    let groups_of_3 = vec
        .chunks_exact(3)
    ;
    let remainder = groups_of_3.remainder();

    let groups_of_3  = groups_of_3.chain({
        let mut modifiable_slice = remainder.to_vec();
        modifiable_slice.push(remainder[0].rotate_left(3));
        if modifiable_slice.len() < 3 {
            modifiable_slice.push(remainder[0].rotate_right(3));
        }

        vec![modifiable_slice.as_slice()]
    });

    let large_values = groups_of_3
        .map(|group_of_three| {
            let arr: [u8; 3] = group_of_three.try_into().unwrap();
            let fourth_value = bitwise_choose(arr);

            let mut final_value = 0u32;
            final_value |= u32::from(fourth_value) << 24;
            final_value |= u32::from(arr[0]) << 16;
            final_value |= u32::from(arr[1]) <<8;
            final_value |= u32::from(arr[2]);

            final_value
        })
    ;

    large_values
        .reduce(|acc, e| acc.wrapping_add(e))
        .unwrap()
}

fn bitwise_choose(values: [u8; 3]) -> u8 {
    (values[0] & values[1]) ^ (!values[0] & values[2])
}