fn main() {
    tscale_sequence::tscale_sequence::TScale::new_with_config([1.,2.], [0.5, 1.])
        .into_iter()
        .take(100)
        .for_each(|x| println!("{:?}", x));
}
