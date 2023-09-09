use sha256::digest;

fn main() {
    let sum = digest("Hello, world!");
    println!("{}", sum);

    // $ sha256sum <(echo -n 'Hello, world!')
    // ==> 315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3  /dev/fd/63
    assert_eq!(
        sum,
        "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3"
    );
}
